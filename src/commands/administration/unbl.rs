use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::{ensure_owner, parse_user_id};
use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, remove_from_blacklist};

pub async fn handle_unbl(ctx: &Context, msg: &Message, args: &[&str]) {
    if ensure_owner(ctx, msg).await.is_err() {
        return;
    }

    if args.is_empty() {
        let embed = serenity::builder::CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+unbl <@membre/ID>`")
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

    let count = remove_from_blacklist(&pool, bot_id, target)
        .await
        .unwrap_or(0);
    let desc = if count > 0 {
        format!("<@{}> retiré de la blacklist.", target.get())
    } else {
        format!("<@{}> n'était pas blacklisté.", target.get())
    };

    let embed = serenity::builder::CreateEmbed::new()
        .title("Unblacklist")
        .description(desc)
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub struct UnblCommand;
pub static COMMAND_DESCRIPTOR: UnblCommand = UnblCommand;

impl crate::commands::command_contract::CommandSpec for UnblCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "unbl",
            category: "administration",
            params: "<@membre/ID>",
            summary: "Retire un utilisateur blacklist",
            description: "Retire un utilisateur de la blacklist globale du bot.",
            examples: &["+unbl", "+ul", "+help unbl"],
            default_aliases: &["unb"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
