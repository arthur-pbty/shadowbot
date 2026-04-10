use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::commands::perms_helpers::{ensure_owner, get_pool, normalize_command_name};
use crate::db::{reset_command_permissions, set_command_permission};

pub async fn handle_changereset(ctx: &Context, msg: &Message) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    let bot_id = ctx.cache.current_user().id;
    let Some(pool) = get_pool(ctx).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let removed = reset_command_permissions(&pool, bot_id).await.unwrap_or(0);
    let embed = CreateEmbed::new()
        .title("Permissions reinitialisees")
        .description(format!("Overrides supprimes: {}", removed))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub async fn handle_change(ctx: &Context, msg: &Message, args: &[&str]) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    let bot_id = ctx.cache.current_user().id;
    let Some(pool) = get_pool(ctx).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if args.len() < 2 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `change <commande> <permission>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let command = normalize_command_name(args[0]);
    let Ok(level) = args[1].parse::<u8>() else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Permission invalide (0..9).")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if level > 9 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Permission invalide (0..9).")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let _ = set_command_permission(&pool, bot_id, &command, level).await;
    let embed = CreateEmbed::new()
        .title("Permission modifiee")
        .description(format!("`{}` -> niveau `{}`", command, level))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub struct ChangeCommand;
pub static COMMAND_DESCRIPTOR: ChangeCommand = ChangeCommand;

impl crate::commands::command_contract::CommandSpec for ChangeCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "change",
            category: "botconfig",
            params: "<commande> <niveau 0-9>",
            description: "Definit le niveau ACL requis pour une commande cible.",
            examples: &["+change", "+ce", "+help change"],
            default_aliases: &["chg"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
