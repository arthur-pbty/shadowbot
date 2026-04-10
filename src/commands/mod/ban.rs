use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_sanction_helpers::{add_sanction, parse_targets};

pub async fn handle_ban(ctx: &Context, msg: &Message, args: &[&str]) {
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
                "ban",
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
            .title("Ban")
            .description(format!("{} membre(s) banni(s).", done))
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct BanCommand;
pub static COMMAND_DESCRIPTOR: BanCommand = BanCommand;
impl crate::commands::command_contract::CommandSpec for BanCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "ban",
            category: "mod",
            params: "<@membre/ID[,..]> [raison]",
            description: "Ban un ou plusieurs membres.",
            examples: &["+ban @User"],
            default_aliases: &["b"],
            allow_in_dm: false,
            default_permission: 7,
        }
    }
}
