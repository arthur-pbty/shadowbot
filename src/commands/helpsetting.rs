use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::db::{
    DbPoolKey, get_help_aliases_enabled, get_help_perms_enabled, get_help_type,
    set_help_aliases_enabled, set_help_perms_enabled, set_help_type,
};

pub async fn handle_helpsetting(ctx: &Context, msg: &Message, args: &[&str]) {
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
        let help_type = get_help_type(&pool, bot_id)
            .await
            .ok()
            .flatten()
            .unwrap_or_else(|| "button".to_string());
        let help_aliases = get_help_aliases_enabled(&pool, bot_id)
            .await
            .ok()
            .flatten()
            .unwrap_or(true);
        let help_perms = get_help_perms_enabled(&pool, bot_id)
            .await
            .ok()
            .flatten()
            .unwrap_or(true);

        let embed = serenity::builder::CreateEmbed::new()
            .title("Configuration de l'aide")
            .description("Paramètres actuels:")
            .field("Mode d'affichage", format!("`{}`", help_type), true)
            .field("Aliases", format!("`{}`", if help_aliases { "on" } else { "off" }), true)
            .field(
                "Permissions",
                format!("`{}`", if help_perms { "on" } else { "off" }),
                true,
            )
            .color(0x5865F2);
        send_embed(ctx, msg, embed).await;
        return;
    }

    match args[0].to_lowercase().as_str() {
        "type" | "mode" => {
            if args.len() < 2 {
                let embed = serenity::builder::CreateEmbed::new()
                    .title("Erreur")
                    .description("Usage: `+helpsetting type <button|select|hybrid>`")
                    .color(0xED4245);
                send_embed(ctx, msg, embed).await;
                return;
            }

            let normalized = match args[1].to_lowercase().as_str() {
                "button" => "button",
                "select" => "select",
                "hybrid" => "hybrid",
                _ => {
                    let embed = serenity::builder::CreateEmbed::new()
                        .title("Erreur")
                        .description("Valeurs valides: `button`, `select`, `hybrid`")
                        .color(0xED4245);
                    send_embed(ctx, msg, embed).await;
                    return;
                }
            };

            let _ = set_help_type(&pool, bot_id, normalized).await;
            let embed = serenity::builder::CreateEmbed::new()
                .title("Mode de help mis à jour")
                .description(format!("Nouveau mode: `{}`", normalized))
                .color(0x57F287);
            send_embed(ctx, msg, embed).await;
        }
        "aliases" | "alias" => {
            if args.len() < 2 {
                let embed = serenity::builder::CreateEmbed::new()
                    .title("Erreur")
                    .description("Usage: `+helpsetting aliases <on|off>`")
                    .color(0xED4245);
                send_embed(ctx, msg, embed).await;
                return;
            }

            let enabled = match args[1].to_lowercase().as_str() {
                "on" | "true" | "yes" => true,
                "off" | "false" | "no" => false,
                _ => {
                    let embed = serenity::builder::CreateEmbed::new()
                        .title("Erreur")
                        .description("Valeurs valides: `on`, `off`")
                        .color(0xED4245);
                    send_embed(ctx, msg, embed).await;
                    return;
                }
            };

            let _ = set_help_aliases_enabled(&pool, bot_id, enabled).await;
            let embed = serenity::builder::CreateEmbed::new()
                .title("Aliases de help mis à jour")
                .description(format!("Aliases: `{}`", if enabled { "on" } else { "off" }))
                .color(0x57F287);
            send_embed(ctx, msg, embed).await;
        }
        "perms" | "permissions" => {
            if args.len() < 2 {
                let embed = serenity::builder::CreateEmbed::new()
                    .title("Erreur")
                    .description("Usage: `+helpsetting perms <on|off>`")
                    .color(0xED4245);
                send_embed(ctx, msg, embed).await;
                return;
            }

            let enabled = match args[1].to_lowercase().as_str() {
                "on" | "true" | "yes" => true,
                "off" | "false" | "no" => false,
                _ => {
                    let embed = serenity::builder::CreateEmbed::new()
                        .title("Erreur")
                        .description("Valeurs valides: `on`, `off`")
                        .color(0xED4245);
                    send_embed(ctx, msg, embed).await;
                    return;
                }
            };

            let _ = set_help_perms_enabled(&pool, bot_id, enabled).await;
            let embed = serenity::builder::CreateEmbed::new()
                .title("Affichage des permissions mis à jour")
                .description(format!(
                    "Permissions: `{}`",
                    if enabled { "on" } else { "off" }
                ))
                .color(0x57F287);
            send_embed(ctx, msg, embed).await;
        }
        _ => {
            let embed = serenity::builder::CreateEmbed::new()
                .title("Erreur")
                .description("Sous-commandes: `type`, `aliases`, `perms`")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
        }
    }
}

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

pub struct HelpsettingCommand;
pub static COMMAND_DESCRIPTOR: HelpsettingCommand = HelpsettingCommand;

impl crate::commands::command_contract::CommandSpec for HelpsettingCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "helpsetting",
            command: "helpsetting",
            category: "permissions",
            params: "<type|aliases|perms> [value]",
            summary: "Configure l'affichage du système d'aide",
            description: "Permet de configurer le mode d'affichage, l'affichage des alias et l'affichage des permissions du système d'aide.",
            examples: &["+helpsetting", "+helpsetting type hybrid", "+helpsetting perms off"],
            alias_source_key: "helpsetting",
            default_aliases: &["hs"],
        }
    }
}
