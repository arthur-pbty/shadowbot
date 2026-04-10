use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::botconfig_common;

pub async fn handle_listen(ctx: &Context, msg: &Message, args: &[&str]) {
    botconfig_common::handle_activity(ctx, msg, "+listen", args).await;
}

pub struct ListenCommand;
pub static COMMAND_DESCRIPTOR: ListenCommand = ListenCommand;

impl crate::commands::command_contract::CommandSpec for ListenCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "listen",
            command: "listen",
            category: "profile",
            params: "<texte[, ,texte2,...]>",
            summary: "Definit une activite listening",
            description: "Configure la rotation des messages d activite en mode listening.",
            examples: &["+listen", "+ln", "+help listen"],
            alias_source_key: "listen",
            default_aliases: &["lsn"],
            default_permission: 8,
        }
    }
}
