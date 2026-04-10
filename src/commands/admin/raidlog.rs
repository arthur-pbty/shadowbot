use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::logs_command_helpers::{parse_target_channel, set_log_channel};

pub async fn handle_raidlog(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    if args
        .first()
        .map(|a| a.eq_ignore_ascii_case("off"))
        .unwrap_or(false)
    {
        set_log_channel(ctx, guild_id, "raid", None, false).await;
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("RaidLog")
                .description("Desactive.")
                .color(theme_color(ctx).await),
        )
        .await;
        return;
    }

    let channel = parse_target_channel(msg, args, 0);
    set_log_channel(ctx, guild_id, "raid", channel, true).await;
    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("RaidLog")
            .description(format!(
                "Active dans {}.",
                channel
                    .map(|c| format!("<#{}>", c.get()))
                    .unwrap_or_else(|| "ce salon".to_string())
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct RaidlogCommand;
pub static COMMAND_DESCRIPTOR: RaidlogCommand = RaidlogCommand;

impl crate::commands::command_contract::CommandSpec for RaidlogCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "raidlog",
            category: "admin",
            params: "[salon]|off",
            summary: "Active les logs antiraid",
            description: "Active les logs antiraid dans un salon ou les desactive.",
            examples: &["+raidlog #logs", "+raidlog off"],
            default_aliases: &["rdlog"],
            default_permission: 8,
        }
    }
}
