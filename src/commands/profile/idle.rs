use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, set_bot_status};

pub async fn handle_idle(ctx: &Context, msg: &Message) {
    ctx.idle();

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };
    if let Some(pool) = pool {
        let _ = set_bot_status(&pool, bot_id, "idle").await;
    }

    let embed = CreateEmbed::new()
        .title("Statut mis à jour")
        .description("Nouveau statut: idle")
        .color(0x57F287);

    send_embed(ctx, msg, embed).await;
}

pub struct IdleCommand;
pub static COMMAND_DESCRIPTOR: IdleCommand = IdleCommand;

impl crate::commands::command_contract::CommandSpec for IdleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "idle",
            category: "profile",
            params: "aucun",
            summary: "Passe le bot en idle",
            description: "Change le statut du bot en idle et sauvegarde ce statut.",
            examples: &["+idle", "+ie", "+help idle"],
            default_aliases: &["idl"],
            default_permission: 8,
        }
    }
}
