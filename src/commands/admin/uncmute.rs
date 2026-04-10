use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_sanction_helpers::{channel_mute_users, parse_targets, pool};

pub async fn handle_uncmute(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.is_empty() {
        return;
    }

    let targets = parse_targets(args[0]).await;
    if targets.is_empty() {
        return;
    }

    let affected = channel_mute_users(ctx, msg.channel_id, &targets, false).await;

    if let Some(pool) = pool(ctx).await {
        let bot_id = ctx.cache.current_user().id;
        for uid in &targets {
            let _ = sqlx::query(
                r#"
                UPDATE bot_sanctions
                SET active = FALSE
                WHERE bot_id = $1 AND guild_id = $2 AND user_id = $3 AND active = TRUE AND kind IN ('cmute','tempcmute') AND channel_id = $4;
                "#,
            )
            .bind(bot_id.get() as i64)
            .bind(guild_id.get() as i64)
            .bind(uid.get() as i64)
            .bind(msg.channel_id.get() as i64)
            .execute(&pool)
            .await;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("UnCMute")
            .description(format!("{} membre(s) uncmute.", affected))
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct UncmuteCommand;
pub static COMMAND_DESCRIPTOR: UncmuteCommand = UncmuteCommand;
impl crate::commands::command_contract::CommandSpec for UncmuteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "uncmute",
            category: "admin",
            params: "<@membre/ID[,..]>",
            summary: "Retire un cmute",
            description: "Met fin au mute salon.",
            examples: &["+uncmute @User"],
            default_aliases: &["ucm"],
            default_permission: 8,
        }
    }
}
