use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, set_bot_status};

pub async fn handle_online(ctx: &Context, msg: &Message) {
    ctx.online();

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };
    if let Some(pool) = pool {
        let _ = set_bot_status(&pool, bot_id, "online").await;
    }

    let embed = CreateEmbed::new()
        .title("Statut mis à jour")
        .description("Nouveau statut: online")
        .color(0x57F287);

    send_embed(ctx, msg, embed).await;
}

pub struct OnlineCommand;
pub static COMMAND_DESCRIPTOR: OnlineCommand = OnlineCommand;

impl crate::commands::command_contract::CommandSpec for OnlineCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "online",
            category: "bot",
            params: "aucun",
            summary: "Passe le bot en online",
            description: "Change le statut du bot en online et sauvegarde ce statut.",
            examples: &["+online", "+oe", "+help online"],
            default_aliases: &["onl"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
