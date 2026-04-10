use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_sanction_helpers::{channel_mute_users, handle_timeout, pool};

pub async fn handle_unmuteall(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let rows = sqlx::query_as::<_, (i64, String, Option<i64>)>(
        r#"
        SELECT user_id, kind, channel_id
        FROM bot_sanctions
        WHERE bot_id = $1 AND guild_id = $2 AND active = TRUE AND kind IN ('mute','tempmute','cmute','tempcmute');
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let mut changed = 0usize;
    for (uid, kind, channel_id) in rows {
        let user_id = UserId::new(uid as u64);
        if kind == "mute" || kind == "tempmute" {
            changed += handle_timeout(ctx, guild_id, &[user_id], None).await;
        } else if let Some(cid) = channel_id {
            changed += channel_mute_users(ctx, ChannelId::new(cid as u64), &[user_id], false).await;
        }
    }

    let _ = sqlx::query(
        r#"
        UPDATE bot_sanctions
        SET active = FALSE
        WHERE bot_id = $1 AND guild_id = $2 AND active = TRUE AND kind IN ('mute','tempmute','cmute','tempcmute');
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .execute(&pool)
    .await;

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("UnMuteAll")
            .description(format!(
                "{} operation(s) de unmute/cmute annule(es).",
                changed
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct UnmuteallCommand;
pub static COMMAND_DESCRIPTOR: UnmuteallCommand = UnmuteallCommand;
impl crate::commands::command_contract::CommandSpec for UnmuteallCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "unmuteall",
            category: "moderation",
            params: "aucun",
            summary: "Retire tous les mutes",
            description: "Supprime tous les mutes en cours.",
            examples: &["+unmuteall"],
            default_aliases: &["uma"],
            default_permission: 8,
        }
    }
}
