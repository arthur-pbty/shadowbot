use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::moderation_tools;

pub async fn handle_clear_messages(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_clear_messages(ctx, msg, args).await;
}

pub struct ClearMessagesCommand;
pub static COMMAND_DESCRIPTOR: ClearMessagesCommand = ClearMessagesCommand;

impl crate::commands::command_contract::CommandSpec for ClearMessagesCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "clear_messages",
            command: "clear",
            category: "admin",
            params: "<nombre> [@membre/ID]",
            summary: "Supprime des messages dans le salon",
            description: "Supprime un nombre de messages, optionnellement filtres par membre.",
            examples: &["+clear 20", "+clear 20 @User"],
            alias_source_key: "clear_messages",
            default_aliases: &["purge"],
        }
    }
}
