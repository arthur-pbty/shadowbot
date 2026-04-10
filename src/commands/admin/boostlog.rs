use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::logs_command_helpers::{parse_target_channel, set_log_channel};

pub async fn handle_boostlog(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(action) = args.first().map(|s| s.to_lowercase()) else {
        let embed = CreateEmbed::new()
            .title("BoostLog")
            .description("Usage: +boostlog <on [salon]|off>")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    match action.as_str() {
        "on" => {
            let channel = parse_target_channel(msg, args, 1);
            set_log_channel(ctx, guild_id, "boost", channel, true).await;
            let embed = CreateEmbed::new()
                .title("BoostLog")
                .description(format!(
                    "Active dans {}.",
                    channel
                        .map(|c| format!("<#{}>", c.get()))
                        .unwrap_or_else(|| "ce salon".to_string())
                ))
                .color(theme_color(ctx).await);
            send_embed(ctx, msg, embed).await;
        }
        "off" => {
            set_log_channel(ctx, guild_id, "boost", None, false).await;
            let embed = CreateEmbed::new()
                .title("BoostLog")
                .description("Desactive.")
                .color(theme_color(ctx).await);
            send_embed(ctx, msg, embed).await;
        }
        _ => {
            let embed = CreateEmbed::new()
                .title("BoostLog")
                .description("Usage: +boostlog <on [salon]|off>")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
        }
    }
}

pub struct BoostlogCommand;
pub static COMMAND_DESCRIPTOR: BoostlogCommand = BoostlogCommand;

impl crate::commands::command_contract::CommandSpec for BoostlogCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "boostlog",
            category: "admin",
            params: "<on [salon]|off>",
            summary: "Active les logs de boosts",
            description: "Active ou desactive les logs de boosts.",
            examples: &["+boostlog on #logs", "+boostlog off"],
            default_aliases: &["blog"],
            default_permission: 8,
        }
    }
}
