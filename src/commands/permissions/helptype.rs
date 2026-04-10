use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, get_help_type, set_help_type};

pub async fn handle_helptype(ctx: &Context, msg: &Message, args: &[&str]) {
    let bot_id = ctx.cache.current_user().id;
    let Some(pool) = pool(ctx).await else {
        let embed = serenity::builder::CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if args.is_empty() {
        let current = get_help_type(&pool, bot_id)
            .await
            .ok()
            .flatten()
            .unwrap_or_else(|| "button".to_string());
        let embed = serenity::builder::CreateEmbed::new()
            .title("Mode help")
            .description(format!(
                "Mode actuel: `{}`\nValeurs: `button`, `select`, `hybrid`",
                current
            ))
            .color(0x5865F2);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let normalized = match args[0].to_lowercase().as_str() {
        "button" => "button",
        "select" => "select",
        "hybrid" => "hybrid",
        _ => {
            let embed = serenity::builder::CreateEmbed::new()
                .title("Erreur")
                .description("Usage: `+helptype <button/select/hybrid>`")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        }
    };

    let _ = set_help_type(&pool, bot_id, normalized).await;
    let embed = serenity::builder::CreateEmbed::new()
        .title("Mode help mis à jour")
        .description(format!("Nouveau mode: `{}`", normalized))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}
pub struct HelptypeCommand;
pub static COMMAND_DESCRIPTOR: HelptypeCommand = HelptypeCommand;

impl crate::commands::command_contract::CommandSpec for HelptypeCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "helptype",
            category: "permissions",
            params: "<button|select|hybrid>",
            summary: "Change le mode daffichage help",
            description: "Definit le mode daffichage de laide entre button, select et hybrid.",
            examples: &["+helptype", "+he", "+help helptype"],
            default_aliases: &["htp"],
            default_permission: 0,
        }
    }
}
