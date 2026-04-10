use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::commands::perms_helpers::{ensure_owner, get_pool};
use crate::db::set_command_permission;
use crate::permissions::{all_command_keys, command_required_permission};

pub async fn handle_changeall(ctx: &Context, msg: &Message, args: &[&str]) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    if args.len() < 2 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `changeall <permission> <permission>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let Ok(from) = args[0].parse::<u8>() else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Permission source invalide.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let Ok(to) = args[1].parse::<u8>() else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Permission cible invalide.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if from > 9 || to > 9 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Permissions valides: 0..9")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
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

    let mut updated = 0usize;
    for cmd in all_command_keys() {
        let current = command_required_permission(ctx, &cmd).await;
        if current == from {
            let _ = set_command_permission(&pool, bot_id, &cmd, to).await;
            updated += 1;
        }
    }

    let embed = CreateEmbed::new()
        .title("Changeall applique")
        .description(format!("{} commande(s): {} -> {}", updated, from, to))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub struct ChangeallCommand;
pub static COMMAND_DESCRIPTOR: ChangeallCommand = ChangeallCommand;

impl crate::commands::command_contract::CommandSpec for ChangeallCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "changeall",
            category: "botconfig",
            params: "<niveau_source 0-9> <niveau_cible 0-9>",
            description: "Remplace en masse un niveau ACL source par un niveau ACL cible.",
            examples: &["+changeall", "+cl", "+help changeall"],
            default_aliases: &["cga"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
