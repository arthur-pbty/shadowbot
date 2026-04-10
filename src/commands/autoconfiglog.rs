use crate::commands::logs_service;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_autoconfiglog(ctx: &Context, msg: &Message) {
    logs_service::handle_autoconfiglog(ctx, msg).await;
}

pub struct AutoconfiglogCommand;
pub static COMMAND_DESCRIPTOR: AutoconfiglogCommand = AutoconfiglogCommand;

impl crate::commands::command_contract::CommandSpec for AutoconfiglogCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "autoconfiglog",
            command: "autoconfiglog",
            category: "admin",
            params: "aucun",
            summary: "Cree tous les salons de logs",
            description: "Cree automatiquement les salons de logs et les configure.",
            examples: &["+autoconfiglog"],
            alias_source_key: "autoconfiglog",
            default_aliases: &["acl"],
        }
    }
}
