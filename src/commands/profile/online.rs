use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::botconfig_common;

pub async fn handle_online(ctx: &Context, msg: &Message) {
    botconfig_common::handle_status(ctx, msg, "+online").await;
}

pub struct OnlineCommand;
pub static COMMAND_DESCRIPTOR: OnlineCommand = OnlineCommand;

impl crate::commands::command_contract::CommandSpec for OnlineCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "online",
            command: "online",
            category: "profile",
            params: "aucun",
            summary: "Passe le bot en online",
            description: "Change le statut du bot en online et sauvegarde ce statut.",
            examples: &["+online", "+oe", "+help online"],
            alias_source_key: "online",
            default_aliases: &["onl"],
            default_permission: 8,
        }
    }
}
