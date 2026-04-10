use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, set_bot_status};

pub async fn handle_dnd(ctx: &Context, msg: &Message) {
    ctx.dnd();

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };
    if let Some(pool) = pool {
        let _ = set_bot_status(&pool, bot_id, "dnd").await;
    }

    let embed = CreateEmbed::new()
        .title("Statut mis à jour")
        .description("Nouveau statut: dnd")
        .color(0x57F287);

    send_embed(ctx, msg, embed).await;
}

pub struct DndCommand;
pub static COMMAND_DESCRIPTOR: DndCommand = DndCommand;

impl crate::commands::command_contract::CommandSpec for DndCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "dnd",
            category: "botconfig",
            params: "aucun",
            description: "Change le statut du bot en do not disturb et sauvegarde ce statut.",
            examples: &["+dnd", "+dd", "+help dnd"],
            default_aliases: &["dnm"],
            allow_in_dm: false,
            default_permission: 7,
        }
    }
}
