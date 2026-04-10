use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::logs_command_helpers::{parse_target_channel, set_log_channel};

pub async fn handle_rolelog(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(action) = args.first().map(|s| s.to_lowercase()) else {
        let embed = CreateEmbed::new()
            .title("RoleLog")
            .description("Usage: +rolelog <on [salon]|off>")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    match action.as_str() {
        "on" => {
            let channel = parse_target_channel(msg, args, 1);
            set_log_channel(ctx, guild_id, "role", channel, true).await;
            let embed = CreateEmbed::new()
                .title("RoleLog")
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
            set_log_channel(ctx, guild_id, "role", None, false).await;
            let embed = CreateEmbed::new()
                .title("RoleLog")
                .description("Desactive.")
                .color(theme_color(ctx).await);
            send_embed(ctx, msg, embed).await;
        }
        _ => {
            let embed = CreateEmbed::new()
                .title("RoleLog")
                .description("Usage: +rolelog <on [salon]|off>")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
        }
    }
}

pub struct RolelogCommand;
pub static COMMAND_DESCRIPTOR: RolelogCommand = RolelogCommand;

impl crate::commands::command_contract::CommandSpec for RolelogCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "rolelog",
            category: "logs",
            params: "<on [salon]|off>",
            summary: "Active les logs de roles",
            description: "Active ou desactive les logs des roles.",
            examples: &["+rolelog on #logs", "+rolelog off"],
            default_aliases: &["rlog"],
            default_permission: 8,
        }
    }
}
