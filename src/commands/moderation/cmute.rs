use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_sanction_helpers::{
    add_sanction, channel_mute_users, parse_targets,
};

pub async fn handle_cmute(ctx: &Context, msg: &Message, args: &[&str]) {
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

    let affected = channel_mute_users(ctx, msg.channel_id, &targets, true).await;

    for uid in &targets {
        add_sanction(
            ctx,
            guild_id,
            *uid,
            msg.author.id,
            "cmute",
            &reason,
            Some(msg.channel_id),
            None,
        )
        .await;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("CMute")
            .description(format!("{} membre(s) cmute.", affected))
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct CmuteCommand;
pub static COMMAND_DESCRIPTOR: CmuteCommand = CmuteCommand;
impl crate::commands::command_contract::CommandSpec for CmuteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "cmute",
            category: "moderation",
            params: "<@membre/ID[,..]> [raison]",
            summary: "Mute salon",
            description: "Mute un membre sur le salon courant.",
            examples: &["+cmute @User"],
            default_aliases: &["cm"],
            default_permission: 8,
        }
    }
}
