use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::activity::{RotatingActivityKind, parse_status, start_rotation};
use crate::commands::common::send_embed;
use crate::db::DbPoolKey;

pub async fn handle_watch(ctx: &Context, msg: &Message, args: &[&str]) {
    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+playto|+listen|+watch|+compet|+stream <message>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let Some(kind) = RotatingActivityKind::from_command("+watch") else {
        return;
    };

    let joined = args.join(" ");
    let messages: Vec<String> = joined
        .split(",,")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    if messages.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Aucun message d'activité valide.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let bot_id = ctx.cache.current_user().id;

    let status = {
        let pool = {
            let data = ctx.data.read().await;
            data.get::<DbPoolKey>().cloned()
        };

        if let Some(pool) = pool {
            if let Ok(Some(saved)) = crate::db::get_bot_status(&pool, bot_id).await {
                parse_status(&saved)
            } else {
                OnlineStatus::Online
            }
        } else {
            OnlineStatus::Online
        }
    };

    start_rotation(ctx, kind, messages.clone(), status).await;

    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    if let Some(pool) = pool {
        let _ =
            crate::db::set_bot_activity(&pool, bot_id, kind.as_db(), &messages.join("\n")).await;
    }

    let embed = CreateEmbed::new()
        .title("Activité mise à jour")
        .description(format!("{} message(s) configuré(s).", messages.len()))
        .field(
            "Rotation",
            "Les textes alternent toutes les 30 secondes.",
            false,
        )
        .color(0x57F287);

    send_embed(ctx, msg, embed).await;
}

pub struct WatchCommand;
pub static COMMAND_DESCRIPTOR: WatchCommand = WatchCommand;

impl crate::commands::command_contract::CommandSpec for WatchCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "watch",
            category: "bot",
            params: "<texte[, ,texte2,...]>",
            summary: "Definit une activite watching",
            description: "Configure la rotation des messages d activite en mode watching.",
            examples: &["+watch", "+wh", "+help watch"],
            default_aliases: &["wtc"],
            default_permission: 8,
        }
    }
}
