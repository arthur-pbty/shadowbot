use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_button(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_button(ctx, msg, args).await;
}

pub struct ButtonCommand;
pub static COMMAND_DESCRIPTOR: ButtonCommand = ButtonCommand;

impl crate::commands::command_contract::CommandSpec for ButtonCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "button",
            command: "button",
            category: "admin",
            params: "<add/del> <lien>",
            summary: "Gere des boutons decoratifs",
            description: "Ajoute ou supprime un bouton de decoration personnalise sur un message du bot.",
            examples: &[
                "+button add https://example.com",
                "+button del https://example.com",
            ],
            alias_source_key: "button",
            default_aliases: &["btn"],
        }
    }
}
