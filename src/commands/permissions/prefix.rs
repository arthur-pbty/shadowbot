use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::commands::perms_helpers::{ensure_owner, get_pool};
use crate::db::set_guild_prefix;

pub async fn handle_prefix(ctx: &Context, msg: &Message, args: &[&str]) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    let Some(guild_id) = msg.guild_id else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Commande disponible uniquement sur un serveur.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `prefix <prefixe>`")
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
        let _ = set_guild_prefix(&pool, bot_id, guild_id, prefix).await;
    }

    let embed = CreateEmbed::new()
        .title("Prefixe serveur mis a jour")
        .description(format!("Nouveau prefixe ici: `{}`", prefix))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub struct PrefixCommand;
pub static COMMAND_DESCRIPTOR: PrefixCommand = PrefixCommand;

impl crate::commands::command_contract::CommandSpec for PrefixCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "prefix",
            category: "permissions",
            params: "<prefix>",
            summary: "Change le prefixe serveur",
            description: "Definit le prefixe du serveur courant.",
            examples: &["+prefix", "+px", "+help prefix"],
            default_aliases: &["pfx"],
            default_permission: 8,
        }
    }
}
