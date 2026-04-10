use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::botconfig_common;

pub async fn handle_dnd(ctx: &Context, msg: &Message) {
    botconfig_common::handle_status(ctx, msg, "+dnd").await;
}

pub struct DndCommand;
pub static COMMAND_DESCRIPTOR: DndCommand = DndCommand;

impl crate::commands::command_contract::CommandSpec for DndCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "dnd",
            command: "dnd",
            category: "profile",
            params: "aucun",
            summary: "Passe le bot en dnd",
            description: "Change le statut du bot en do not disturb et sauvegarde ce statut.",
            examples: &["+dnd", "+dd", "+help dnd"],
            alias_source_key: "dnd",
            default_aliases: &["dnm"],
        }
    }
}
