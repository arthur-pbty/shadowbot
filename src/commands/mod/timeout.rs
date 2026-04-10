use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::automod_service::{parse_on_off, pool};
use crate::commands::common::send_embed;
use crate::db;

pub async fn handle_timeout_toggle(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(value) = args.first().and_then(|raw| parse_on_off(raw)) else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Timeout")
                .description("Usage: +timeout <on/off>")
                .color(0xED4245),
        )
        .await;
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let Ok(settings) =
        db::set_use_timeout_for_mute(&pool, bot_id, guild_id.get() as i64, value).await
    else {
        return;
    };

    let mode = if settings.use_timeout {
        "Timeout Discord"
    } else {
        "Role mute"
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Timeout")
            .description(format!(
                "Mode mute mis a jour: **{}**.\nNote: les timeouts Discord sont limites a 28 jours.",
                mode
            ))
            .color(0x57F287),
    )
    .await;
}

pub struct TimeoutCommand;
pub static COMMAND_DESCRIPTOR: TimeoutCommand = TimeoutCommand;

impl crate::commands::command_contract::CommandSpec for TimeoutCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "timeout",
            category: "mod",
            params: "<on/off>",
            description: "Active ou desactive l utilisation du timeout Discord pour les mutes.",
            examples: &["+timeout on", "+timeout off", "+help timeout"],
            default_aliases: &["to"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
