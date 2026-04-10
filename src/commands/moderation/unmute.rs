use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_sanction_helpers::{handle_timeout, parse_targets, pool};

pub async fn handle_unmute(ctx: &Context, msg: &Message, args: &[&str]) {
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

    let affected = handle_timeout(ctx, guild_id, &targets, None).await;

    if let Some(pool) = pool(ctx).await {
        let bot_id = ctx.cache.current_user().id;
        for uid in &targets {
            let _ = sqlx::query(
                r#"
                UPDATE bot_sanctions
                SET active = FALSE
                WHERE bot_id = $1 AND guild_id = $2 AND user_id = $3 AND active = TRUE AND kind IN ('mute','tempmute');
                "#,
            )
            .bind(bot_id.get() as i64)
            .bind(guild_id.get() as i64)
            .bind(uid.get() as i64)
            .execute(&pool)
            .await;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("UnMute")
            .description(format!("{} membre(s) unmute.", affected))
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct UnmuteCommand;
pub static COMMAND_DESCRIPTOR: UnmuteCommand = UnmuteCommand;
impl crate::commands::command_contract::CommandSpec for UnmuteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "unmute",
            category: "moderation",
            params: "<@membre/ID[,..]>",
            description: "Met fin au mute d un ou plusieurs membres.",
            examples: &["+unmute @User"],
            default_aliases: &["um"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
