use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::moderation_tools;

pub async fn handle_del_sanction(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_del_sanction(ctx, msg, args).await;
}

pub struct DelSanctionCommand;
pub static COMMAND_DESCRIPTOR: DelSanctionCommand = DelSanctionCommand;

impl crate::commands::command_contract::CommandSpec for DelSanctionCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "del_sanction",
            command: "del sanction",
            category: "admin",
            params: "<@membre/ID> <nombre>",
            summary: "Supprime une sanction d un membre",
            description: "Supprime une sanction specifique dans l historique d un membre.",
            examples: &["+del sanction @User 1"],
            alias_source_key: "del_sanction",
            default_aliases: &["delsanction"],
        }
    }
}
