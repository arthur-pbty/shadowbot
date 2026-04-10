use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::{ban_user_everywhere, ensure_owner, parse_user_id};
use crate::commands::common::{add_list_fields, send_embed, theme_color, truncate_text};
use crate::db::{DbPoolKey, add_to_blacklist, list_blacklist};

pub async fn handle_bl(ctx: &Context, msg: &Message, args: &[&str]) {
    if ensure_owner(ctx, msg).await.is_err() {
        return;
    }

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    let Some(pool) = pool else {
        let embed = serenity::builder::CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if args.is_empty() {
        let rows = list_blacklist(&pool, bot_id).await.unwrap_or_default();
        let lines = rows
            .iter()
            .map(|r| format!("<@{}> · {}", r.user_id, truncate_text(&r.reason, 80)))
            .collect::<Vec<_>>();

        let color = theme_color(ctx).await;
        let mut embed = serenity::builder::CreateEmbed::new()
            .title("Blacklist")
            .color(color);
        embed = add_list_fields(embed, &lines, "Membres blacklistés");
        send_embed(ctx, msg, embed).await;
        return;
    }

    let Some(target) = parse_user_id(args[0]) else {
        let embed = serenity::builder::CreateEmbed::new()
            .title("Erreur")
            .description("Membre invalide.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let reason = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Aucune raison fournie".to_string()
    };

    let _ = add_to_blacklist(&pool, bot_id, target, &reason, Some(msg.author.id)).await;
    let (ok, ko) = ban_user_everywhere(ctx, target, &format!("Blacklist: {}", reason)).await;

    let embed = serenity::builder::CreateEmbed::new()
        .title("Blacklist mise à jour")
        .description(format!("<@{}> a été blacklisté.", target.get()))
        .field("Raison", truncate_text(&reason, 1024), false)
        .field(
            "Bans appliqués",
            format!("{} réussis · {} échecs", ok, ko),
            false,
        )
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub struct BlCommand;
pub static COMMAND_DESCRIPTOR: BlCommand = BlCommand;

impl crate::commands::command_contract::CommandSpec for BlCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "bl",
            category: "administration",
            params: "[<@membre/ID> [raison...]]",
            summary: "Gere la blacklist globale",
            description: "Affiche la blacklist ou ajoute un utilisateur a la blacklist globale du bot.",
            examples: &["+bl", "+help bl"],
            default_aliases: &["bls"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
