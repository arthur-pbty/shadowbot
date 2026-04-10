use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::perms_service;

pub async fn handle_mainprefix(ctx: &Context, msg: &Message, args: &[&str]) {
    perms_service::handle_mainprefix(ctx, msg, args).await;
}

pub struct MainprefixCommand;
pub static COMMAND_DESCRIPTOR: MainprefixCommand = MainprefixCommand;

impl crate::commands::command_contract::CommandSpec for MainprefixCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "mainprefix",
            command: "mainprefix",
            category: "permissions",
            params: "<prefix>",
            summary: "Change le prefixe global",
            description: "Definit le prefixe principal utilise par le bot sur tous les serveurs.",
            examples: &["+mainprefix", "+mx", "+help mainprefix"],
            alias_source_key: "mainprefix",
            default_aliases: &["mpx"],
        }
    }
}
