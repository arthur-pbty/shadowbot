use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::botconfig_common;

pub async fn handle_watch(ctx: &Context, msg: &Message, args: &[&str]) {
    botconfig_common::handle_activity(ctx, msg, "+watch", args).await;
}

pub struct WatchCommand;
pub static COMMAND_DESCRIPTOR: WatchCommand = WatchCommand;

impl crate::commands::command_contract::CommandSpec for WatchCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "watch",
            command: "watch",
            category: "profile",
            params: "<texte[, ,texte2,...]>",
            summary: "Definit une activite watching",
            description: "Configure la rotation des messages d activite en mode watching.",
            examples: &["+watch", "+wh", "+help watch"],
            alias_source_key: "watch",
            default_aliases: &["wtc"],
            default_permission: 8,
        }
    }
}
