use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_end(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_end(ctx, msg, args).await;
}

pub struct EndCommand;
pub static COMMAND_DESCRIPTOR: EndCommand = EndCommand;

impl crate::commands::command_contract::CommandSpec for EndCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "end",
            command: "end",
            category: "admin",
            params: "giveaway <id_message>",
            summary: "Termine un giveaway par ID",
            description: "Permet de stopper instantanement un giveaway avec l'identifiant du message.",
            examples: &["+end giveaway 123456789012345678"],
            alias_source_key: "end",
            default_aliases: &["gend"],
        }
    }
}
