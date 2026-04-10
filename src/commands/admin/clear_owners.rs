use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::ensure_owner;
use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, clear_bot_owners};

pub async fn handle_clear_owners(ctx: &Context, msg: &Message) {
    if ensure_owner(ctx, msg).await.is_err() {
        return;
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

    let count = clear_bot_owners(&pool, bot_id).await.unwrap_or(0);
    let embed = serenity::builder::CreateEmbed::new()
        .title("Owners réinitialisés")
        .description(format!("{} owner(s) supprimé(s).", count))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub struct ClearOwnersCommand;
pub static COMMAND_DESCRIPTOR: ClearOwnersCommand = ClearOwnersCommand;

impl crate::commands::command_contract::CommandSpec for ClearOwnersCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "clear_owners",
            command: "clear owners",
            category: "admin",
            params: "aucun",
            summary: "Vide la liste des owners",
            description: "Supprime tous les owners supplementaires en base de donnees.",
            examples: &["+clear owners", "+cs", "+help clear owners"],
            alias_source_key: "clear_owners",
            default_aliases: &["cro"],
            default_permission: 9,
        }
    }
}
