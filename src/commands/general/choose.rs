use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_choose(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_choose(ctx, msg, args).await;
}

pub struct ChooseCommand;
pub static COMMAND_DESCRIPTOR: ChooseCommand = ChooseCommand;

impl crate::commands::command_contract::CommandSpec for ChooseCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "choose",
            command: "choose",
            category: "general",
            params: "<option1 | option2 | ...>",
            summary: "Tire une option au hasard",
            description: "Lance un tirage au sort instantane parmi les options donnees.",
            examples: &["+choose rouge | bleu | vert"],
            alias_source_key: "choose",
            default_aliases: &["pick", "random"],
            default_permission: 8,
        }
    }
}
