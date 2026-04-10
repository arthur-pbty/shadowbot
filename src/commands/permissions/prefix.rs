use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::perms_service;

pub async fn handle_prefix(ctx: &Context, msg: &Message, args: &[&str]) {
    perms_service::handle_prefix(ctx, msg, args).await;
}

pub struct PrefixCommand;
pub static COMMAND_DESCRIPTOR: PrefixCommand = PrefixCommand;

impl crate::commands::command_contract::CommandSpec for PrefixCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "prefix",
            command: "prefix",
            category: "permissions",
            params: "<prefix>",
            summary: "Change le prefixe serveur",
            description: "Definit le prefixe du serveur courant.",
            examples: &["+prefix", "+px", "+help prefix"],
            alias_source_key: "prefix",
            default_aliases: &["pfx"],
            default_permission: 8,
        }
    }
}
