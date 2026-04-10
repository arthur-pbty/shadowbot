use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, set_bot_status};

pub async fn handle_invisible(ctx: &Context, msg: &Message) {
    ctx.invisible();

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };
    if let Some(pool) = pool {
        let _ = set_bot_status(&pool, bot_id, "invisible").await;
    }

    let embed = CreateEmbed::new()
        .title("Statut mis à jour")
        .description("Nouveau statut: invisible")
        .color(0x57F287);

    send_embed(ctx, msg, embed).await;
}

pub struct InvisibleCommand;
pub static COMMAND_DESCRIPTOR: InvisibleCommand = InvisibleCommand;

impl crate::commands::command_contract::CommandSpec for InvisibleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "invisible",
            category: "profile",
            params: "aucun",
            summary: "Passe le bot en invisible",
            description: "Change le statut du bot en invisible et sauvegarde ce statut.",
            examples: &["+invisible", "+ie", "+help invisible"],
            default_aliases: &["ivs"],
            default_permission: 8,
        }
    }
}
