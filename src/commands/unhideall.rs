use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_unhideall(ctx: &Context, msg: &Message) {
    moderation_tools::handle_hideall_unhideall(ctx, msg, false).await;
}

pub struct UnhideallCommand;
pub static COMMAND_DESCRIPTOR: UnhideallCommand = UnhideallCommand;

impl crate::commands::command_contract::CommandSpec for UnhideallCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "unhideall",
            command: "unhideall",
            category: "admin",
            params: "aucun",
            summary: "Affiche tous les salons",
            description: "Rend visibles tous les salons du serveur.",
            examples: &["+unhideall"],
            alias_source_key: "unhideall",
            default_aliases: &["uhda"],
        }
    }
}
