use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_reroll(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_reroll(ctx, msg, args).await;
}

pub struct RerollCommand;
pub static COMMAND_DESCRIPTOR: RerollCommand = RerollCommand;

impl crate::commands::command_contract::CommandSpec for RerollCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "reroll",
            command: "reroll",
            category: "admin",
            params: "aucun (en reponse a un message)",
            summary: "Relance un tirage giveaway",
            description: "Choisit un nouveau gagnant depuis le message cible.",
            examples: &["+reroll"],
            alias_source_key: "reroll",
            default_aliases: &["rro", "greroll"],
            default_permission: 8,
        }
    }
}
