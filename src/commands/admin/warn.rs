use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_sanction_helpers::{add_sanction, parse_targets};

pub async fn handle_warn(ctx: &Context, msg: &Message, args: &[&str]) {
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

    for uid in &targets {
        add_sanction(
            ctx,
            guild_id,
            *uid,
            msg.author.id,
            "warn",
            &reason,
            None,
            None,
        )
        .await;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Warn")
            .description(format!("{} membre(s) warn.", targets.len()))
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct WarnCommand;
pub static COMMAND_DESCRIPTOR: WarnCommand = WarnCommand;
impl crate::commands::command_contract::CommandSpec for WarnCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "warn",
            category: "admin",
            params: "<@membre/ID[,..]> [raison]",
            summary: "Donne un warn",
            description: "Ajoute un warn a un ou plusieurs membres.",
            examples: &["+warn @User spam"],
            default_aliases: &["avert"],
            default_permission: 8,
        }
    }
}
