use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::activity::{RotatingActivityKind, parse_status, start_rotation, stop_rotation};
use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, set_bot_status};

pub fn parse_color(value: &str) -> Option<u32> {
    let v = value.trim().to_lowercase();
    match v.as_str() {
        "red" | "rouge" => Some(0xED4245),
        "green" | "vert" => Some(0x57F287),
        "blue" | "bleu" => Some(0x5865F2),
        "yellow" | "jaune" => Some(0xFEE75C),
        "orange" => Some(0xFAA61A),
        "purple" | "violet" => Some(0x9B59B6),
        "pink" | "rose" => Some(0xEB459E),
        "white" | "blanc" => Some(0xFFFFFF),
        "black" | "noir" => Some(0x000000),
        _ => {
            let hex = v.trim_start_matches('#').trim_start_matches("0x");
            u32::from_str_radix(hex, 16).ok()
        }
    }
}

pub async fn save_status_if_db(ctx: &Context, status: &str) {
    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    if let Some(pool) = pool {
        let _ = set_bot_status(&pool, bot_id, status).await;
    }
}

pub async fn handle_activity(ctx: &Context, msg: &Message, command: &str, args: &[&str]) {
    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+playto|+listen|+watch|+compet|+stream <message>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let Some(kind) = RotatingActivityKind::from_command(command) else {
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

pub async fn handle_remove_activity(ctx: &Context, msg: &Message) {
    stop_rotation(ctx).await;
    ctx.set_activity(None);

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    if let Some(pool) = pool {
        let _ = crate::db::clear_bot_activity(&pool, bot_id).await;
    }

    let embed = CreateEmbed::new()
        .title("Activité supprimée")
        .description("L'activité du bot a été retirée.")
        .color(0x57F287);

    send_embed(ctx, msg, embed).await;
}

pub async fn handle_status(ctx: &Context, msg: &Message, command: &str) {
    let status_name = match command {
        "+online" => {
            ctx.online();
            "online"
        }
        "+idle" => {
            ctx.idle();
            "idle"
        }
        "+dnd" => {
            ctx.dnd();
            "dnd"
        }
        "+invisible" => {
            ctx.invisible();
            "invisible"
        }
        _ => return,
    };

    save_status_if_db(ctx, status_name).await;

    let embed = CreateEmbed::new()
        .title("Statut mis à jour")
        .description(format!("Nouveau statut: {}", status_name))
        .color(0x57F287);

    send_embed(ctx, msg, embed).await;
}
