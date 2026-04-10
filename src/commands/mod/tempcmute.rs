use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_sanction_helpers::{
    add_sanction, channel_mute_users, duration_from_input, parse_targets,
};

pub async fn handle_tempcmute(ctx: &Context, msg: &Message, args: &[&str]) {
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

    let affected = channel_mute_users(ctx, msg.channel_id, &targets, true).await;

    for uid in &targets {
        add_sanction(
            ctx,
            guild_id,
            *uid,
            msg.author.id,
            "tempcmute",
            &reason,
            Some(msg.channel_id),
            expires_at,
        )
        .await;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("TempCMute")
            .description(format!("{} membre(s) cmute.", affected))
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct TempcmuteCommand;
pub static COMMAND_DESCRIPTOR: TempcmuteCommand = TempcmuteCommand;
impl crate::commands::command_contract::CommandSpec for TempcmuteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "tempcmute",
            category: "mod",
            params: "<@membre/ID[,..]> <duree> [raison]",
            description: "Mute temporaire sur le salon courant.",
            examples: &["+tempcmute @User 5m"],
            default_aliases: &["tcm"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
