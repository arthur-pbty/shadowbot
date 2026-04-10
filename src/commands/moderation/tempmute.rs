use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_sanction_helpers::{
    add_sanction, duration_from_input, handle_timeout, parse_targets,
};

pub async fn handle_tempmute(ctx: &Context, msg: &Message, args: &[&str]) {
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

    let Some(duration_raw) = args.get(1) else {
        return;
    };
    let Some(duration) = duration_from_input(duration_raw) else {
        return;
    };
    let expires_at = Some(Utc::now() + chrono::Duration::seconds(duration.as_secs() as i64));

    let reason = if args.len() > 2 {
        args[2..].join(" ")
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
            "tempmute",
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
            .title("TempMute")
            .description(format!("{} membre(s) mute.", affected))
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct TempmuteCommand;
pub static COMMAND_DESCRIPTOR: TempmuteCommand = TempmuteCommand;
impl crate::commands::command_contract::CommandSpec for TempmuteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "tempmute",
            category: "moderation",
            params: "<@membre/ID[,..]> <duree> [raison]",
            summary: "Mute temporaire",
            description: "Mute un ou plusieurs membres pour une duree donnee.",
            examples: &["+tempmute @User 10m"],
            default_aliases: &["tm"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
