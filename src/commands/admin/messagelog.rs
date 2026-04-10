use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::logs_command_helpers::{parse_target_channel, set_log_channel};

pub async fn handle_messagelog(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(action) = args.first().map(|s| s.to_lowercase()) else {
        let embed = CreateEmbed::new()
            .title("MessageLog")
            .description("Usage: +messagelog <on [salon]|off>")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    match action.as_str() {
        "on" => {
            let channel = parse_target_channel(msg, args, 1);
            set_log_channel(ctx, guild_id, "message", channel, true).await;
            let embed = CreateEmbed::new()
                .title("MessageLog")
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
            set_log_channel(ctx, guild_id, "message", None, false).await;
            let embed = CreateEmbed::new()
                .title("MessageLog")
                .description("Desactive.")
                .color(theme_color(ctx).await);
            send_embed(ctx, msg, embed).await;
        }
        _ => {
            let embed = CreateEmbed::new()
                .title("MessageLog")
                .description("Usage: +messagelog <on [salon]|off>")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
        }
    }
}

pub struct MessagelogCommand;
pub static COMMAND_DESCRIPTOR: MessagelogCommand = MessagelogCommand;

impl crate::commands::command_contract::CommandSpec for MessagelogCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "messagelog",
            category: "admin",
            params: "<on [salon]|off>",
            summary: "Active les logs de messages",
            description: "Active ou desactive les logs des messages supprimes et edites.",
            examples: &["+messagelog on #logs", "+messagelog off"],
            default_aliases: &["msglog"],
            default_permission: 8,
        }
    }
}
