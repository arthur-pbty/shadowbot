use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::botconfig_common;

pub async fn handle_idle(ctx: &Context, msg: &Message) {
    botconfig_common::handle_status(ctx, msg, "+idle").await;
}

pub struct IdleCommand;
pub static COMMAND_DESCRIPTOR: IdleCommand = IdleCommand;

impl crate::commands::command_contract::CommandSpec for IdleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "idle",
            command: "idle",
            category: "profile",
            params: "aucun",
            summary: "Passe le bot en idle",
            description: "Change le statut du bot en idle et sauvegarde ce statut.",
            examples: &["+idle", "+ie", "+help idle"],
            alias_source_key: "idle",
            default_aliases: &["idl"],
            default_permission: 8,
        }
    }
}
