use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_sanction_helpers::{add_sanction, parse_targets};

pub async fn handle_kick(ctx: &Context, msg: &Message, args: &[&str]) {
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

    let reason = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Aucune raison".to_string()
    };

    let mut done = 0usize;
    for uid in &targets {
        if guild_id
            .kick_with_reason(&ctx.http, *uid, &reason)
            .await
            .is_ok()
        {
            done += 1;
            add_sanction(
                ctx,
                guild_id,
                *uid,
                msg.author.id,
                "kick",
                &reason,
                None,
                None,
            )
            .await;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Kick")
            .description(format!("{} membre(s) expulse(s).", done))
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct KickCommand;
pub static COMMAND_DESCRIPTOR: KickCommand = KickCommand;
impl crate::commands::command_contract::CommandSpec for KickCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "kick",
            category: "moderation",
            params: "<@membre/ID[,..]> [raison]",
            summary: "Expulse un membre",
            description: "Kick un ou plusieurs membres.",
            examples: &["+kick @User"],
            default_aliases: &["k"],
            default_permission: 8,
        }
    }
}
