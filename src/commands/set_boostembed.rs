use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_set_boostembed(ctx: &Context, msg: &Message, args: &[&str]) {
    logs_service::handle_set_boostembed(ctx, msg, args).await;
}

pub struct SetBoostembedCommand;
pub static COMMAND_DESCRIPTOR: SetBoostembedCommand = SetBoostembedCommand;

impl crate::commands::command_contract::CommandSpec for SetBoostembedCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "set_boostembed",
            command: "set boostembed",
            category: "admin",
            params: "<title|description|color> <valeur>",
            summary: "Parametre l embed de boost",
            description: "Configure le titre, la description et la couleur de l embed boost.",
            examples: &[
                "+set boostembed title Merci",
                "+set boostembed color #FF66CC",
            ],
            alias_source_key: "set_boostembed",
            default_aliases: &["sboostembed"],
        }
    }
}
