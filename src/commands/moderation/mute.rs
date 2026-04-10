use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_sanction_helpers::{add_sanction, handle_timeout, parse_targets};

pub async fn handle_mute(ctx: &Context, msg: &Message, args: &[&str]) {
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

    let expires_at = Some(Utc::now() + chrono::Duration::seconds(28 * 24 * 3600));
    let reason = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Aucune raison".to_string()
    };

    let affected = handle_timeout(ctx, guild_id, &targets, expires_at).await;

    for uid in &targets {
        add_sanction(
            ctx,
            guild_id,
            *uid,
            msg.author.id,
            "mute",
            &reason,
            None,
            expires_at,
        )
        .await;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Mute")
            .description(format!("{} membre(s) mute.", affected))
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct MuteCommand;
pub static COMMAND_DESCRIPTOR: MuteCommand = MuteCommand;
impl crate::commands::command_contract::CommandSpec for MuteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "mute",
            category: "moderation",
            params: "<@membre/ID[,..]> [raison]",
            description: "Applique un mute a un ou plusieurs membres.",
            examples: &["+mute @User abus"],
            default_aliases: &["tmute"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
