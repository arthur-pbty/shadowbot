use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::{app_owner_id, ensure_owner, parse_user_id};
use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, remove_bot_owner};

pub async fn handle_unowner(ctx: &Context, msg: &Message, args: &[&str]) {
    if ensure_owner(ctx, msg).await.is_err() {
        return;
    }

    if args.is_empty() {
        let embed = serenity::builder::CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+unowner <@membre/ID>`")
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

    if let Some(app_owner) = app_owner_id(ctx).await {
        if app_owner == target {
            let embed = serenity::builder::CreateEmbed::new()
                .title("Refusé")
                .description("Impossible de retirer l'owner principal de l'application.")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        }
    }

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

    let removed = remove_bot_owner(&pool, bot_id, target).await.unwrap_or(0);
    let desc = if removed > 0 {
        format!("<@{}> n'est plus owner.", target.get())
    } else {
        format!("<@{}> n'était pas owner.", target.get())
    };

    let embed = serenity::builder::CreateEmbed::new()
        .title("Unowner")
        .description(desc)
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub struct UnownerCommand;
pub static COMMAND_DESCRIPTOR: UnownerCommand = UnownerCommand;

impl crate::commands::command_contract::CommandSpec for UnownerCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "unowner",
            category: "administration",
            params: "<@membre/ID>",
            summary: "Retire un owner du bot",
            description: "Retire un utilisateur de la liste des owners supplementaires du bot.",
            examples: &["+unowner", "+ur", "+help unowner"],
            default_aliases: &["uow"],
            default_permission: 9,
        }
    }
}
