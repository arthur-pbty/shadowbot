use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::ensure_owner;
use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, clear_blacklist};

pub async fn handle_clear_bl(ctx: &Context, msg: &Message) {
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

    let count = clear_blacklist(&pool, bot_id).await.unwrap_or(0);
    let embed = serenity::builder::CreateEmbed::new()
        .title("Blacklist réinitialisée")
        .description(format!("{} membre(s) retiré(s) de la blacklist.", count))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub struct ClearBlCommand;
pub static COMMAND_DESCRIPTOR: ClearBlCommand = ClearBlCommand;

impl crate::commands::command_contract::CommandSpec for ClearBlCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "clear_bl",
            category: "owner",
            params: "aucun",
            description: "Supprime toutes les entrees de la blacklist globale.",
            examples: &["+clearbl", "+cl", "+help clearbl"],
            default_aliases: &["cbl"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
