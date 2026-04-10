use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::botconfig_common;

pub async fn handle_compet(ctx: &Context, msg: &Message, args: &[&str]) {
    botconfig_common::handle_activity(ctx, msg, "+compet", args).await;
}

pub struct CompetCommand;
pub static COMMAND_DESCRIPTOR: CompetCommand = CompetCommand;

impl crate::commands::command_contract::CommandSpec for CompetCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "compet",
            command: "compet",
            category: "profile",
            params: "<texte[, ,texte2,...]>",
            summary: "Definit une activite competing",
            description: "Configure la rotation des messages d activite en mode competing.",
            examples: &["+compet", "+ct", "+help compet"],
            alias_source_key: "compet",
            default_aliases: &["cpt"],
        }
    }
}
