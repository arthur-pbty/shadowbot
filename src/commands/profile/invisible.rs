use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::botconfig_common;

pub async fn handle_invisible(ctx: &Context, msg: &Message) {
    botconfig_common::handle_status(ctx, msg, "+invisible").await;
}

pub struct InvisibleCommand;
pub static COMMAND_DESCRIPTOR: InvisibleCommand = InvisibleCommand;

impl crate::commands::command_contract::CommandSpec for InvisibleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "invisible",
            command: "invisible",
            category: "profile",
            params: "aucun",
            summary: "Passe le bot en invisible",
            description: "Change le statut du bot en invisible et sauvegarde ce statut.",
            examples: &["+invisible", "+ie", "+help invisible"],
            alias_source_key: "invisible",
            default_aliases: &["ivs"],
            default_permission: 8,
        }
    }
}
