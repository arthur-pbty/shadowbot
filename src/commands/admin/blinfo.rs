use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::{ensure_owner, parse_user_id};
use crate::commands::common::{send_embed, truncate_text};
use crate::db::{DbPoolKey, get_blacklist_info};

pub async fn handle_blinfo(ctx: &Context, msg: &Message, args: &[&str]) {
    if ensure_owner(ctx, msg).await.is_err() {
        return;
    }

    if args.is_empty() {
        let embed = serenity::builder::CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+blinfo <@membre/ID>`")
            .color(0xED4245);
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

    let info = get_blacklist_info(&pool, bot_id, target)
        .await
        .ok()
        .flatten();
    let Some(info) = info else {
        let embed = serenity::builder::CreateEmbed::new()
            .title("Blacklist")
            .description("Ce membre n'est pas blacklisté.")
            .color(0xFF0000);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let added_at = crate::commands::common::discord_ts(
        Timestamp::from_unix_timestamp(info.added_at.timestamp())
            .unwrap_or_else(|_| Timestamp::now()),
        "F",
    );

    let by = info
        .added_by
        .map(|id| format!("<@{}>", id))
        .unwrap_or_else(|| "Inconnu".to_string());

    let embed = serenity::builder::CreateEmbed::new()
        .title("Informations blacklist")
        .field("Membre", format!("<@{}>", info.user_id), true)
        .field("Ajouté par", by, true)
        .field("Ajouté le", added_at, true)
        .field("Raison", truncate_text(&info.reason, 1024), false)
        .color(0xFF0000);
    send_embed(ctx, msg, embed).await;
}

pub struct BlinfoCommand;
pub static COMMAND_DESCRIPTOR: BlinfoCommand = BlinfoCommand;

impl crate::commands::command_contract::CommandSpec for BlinfoCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "blinfo",
            category: "admin",
            params: "<@membre/ID>",
            summary: "Affiche les details blacklist",
            description: "Affiche les details de blacklist pour un utilisateur donne.",
            examples: &["+blinfo", "+bo", "+help blinfo"],
            default_aliases: &["bli"],
            default_permission: 9,
        }
    }
}
