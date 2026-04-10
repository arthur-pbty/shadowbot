use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::commands::perms_helpers::{ensure_owner, get_pool};
use crate::db::set_main_prefix;

pub async fn handle_mainprefix(ctx: &Context, msg: &Message, args: &[&str]) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `mainprefix <prefixe>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let prefix = args[0].trim();
    if prefix.is_empty() || prefix.len() > 5 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Prefixe invalide (1 a 5 caracteres).")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let bot_id = ctx.cache.current_user().id;
    if let Some(pool) = get_pool(ctx).await {
        let _ = set_main_prefix(&pool, bot_id, prefix).await;
    }

    let embed = CreateEmbed::new()
        .title("Prefixe principal mis a jour")
        .description(format!("Nouveau prefixe principal: `{}`", prefix))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub struct MainprefixCommand;
pub static COMMAND_DESCRIPTOR: MainprefixCommand = MainprefixCommand;

impl crate::commands::command_contract::CommandSpec for MainprefixCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "mainprefix",
            category: "botconfig",
            params: "<prefix>",
            description: "Definit le prefixe principal utilise par le bot sur tous les serveurs.",
            examples: &["+mainprefix", "+mx", "+help mainprefix"],
            default_aliases: &["mpx"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
