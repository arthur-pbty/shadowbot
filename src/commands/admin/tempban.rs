use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_sanction_helpers::{
    add_sanction, duration_from_input, parse_targets,
};

pub async fn handle_tempban(ctx: &Context, msg: &Message, args: &[&str]) {
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

    let mut done = 0usize;
    for uid in &targets {
        if guild_id
            .ban_with_reason(&ctx.http, *uid, 0, &reason)
            .await
            .is_ok()
        {
            done += 1;
            add_sanction(
                ctx,
                guild_id,
                *uid,
                msg.author.id,
                "tempban",
                &reason,
                None,
                expires_at,
            )
            .await;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("TempBan")
            .description(format!("{} membre(s) banni(s).", done))
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct TempbanCommand;
pub static COMMAND_DESCRIPTOR: TempbanCommand = TempbanCommand;
impl crate::commands::command_contract::CommandSpec for TempbanCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "tempban",
            category: "admin",
            params: "<@membre/ID[,..]> <duree> [raison]",
            summary: "Ban temporaire",
            description: "Ban temporairement un ou plusieurs membres.",
            examples: &["+tempban @User 1d"],
            default_aliases: &["tb"],
            default_permission: 8,
        }
    }
}
