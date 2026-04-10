use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_sanction_helpers::pool;

pub async fn handle_mutelist(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let rows = sqlx::query_as::<_, (i64, String, Option<i64>, Option<chrono::DateTime<Utc>>)>(
        r#"
        SELECT user_id, kind, channel_id, expires_at
        FROM bot_sanctions
        WHERE bot_id = $1 AND guild_id = $2 AND active = TRUE AND kind IN ('mute','tempmute','cmute','tempcmute')
        ORDER BY created_at DESC
        LIMIT 60;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let desc = if rows.is_empty() {
        "Aucun mute en cours.".to_string()
    } else {
        rows.into_iter()
            .map(|(uid, kind, channel_id, exp)| {
                let channel = channel_id
                    .map(|c| format!(" dans <#{}>", c))
                    .unwrap_or_default();
                let until = exp
                    .map(|d| format!(" jusqu'a <t:{}:R>", d.timestamp()))
                    .unwrap_or_default();
                format!("- <@{}> `{}`{}{}", uid, kind, channel, until)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("MuteList")
            .description(desc)
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct MutelistCommand;
pub static COMMAND_DESCRIPTOR: MutelistCommand = MutelistCommand;
impl crate::commands::command_contract::CommandSpec for MutelistCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "mutelist",
            category: "mod",
            params: "aucun",
            description: "Affiche tous les mutes en cours.",
            examples: &["+mutelist"],
            default_aliases: &["ml"],
            allow_in_dm: false,
            default_permission: 5,
        }
    }
}
