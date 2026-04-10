use crate::commands::logs_service;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_nolog(ctx: &Context, msg: &Message, args: &[&str]) {
    logs_service::handle_nolog(ctx, msg, args).await;
}

pub struct NologCommand;
pub static COMMAND_DESCRIPTOR: NologCommand = NologCommand;

impl crate::commands::command_contract::CommandSpec for NologCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "nolog",
            command: "nolog",
            category: "admin",
            params: "<add/del> [salon] [message|voice|all]",
            summary: "Exclut des salons des logs",
            description: "Desactive ou reactive les logs message/voice pour certains salons.",
            examples: &["+nolog add #secret all", "+nolog del #secret message"],
            alias_source_key: "nolog",
            default_aliases: &["nlg"],
            default_permission: 8,
        }
    }
}
