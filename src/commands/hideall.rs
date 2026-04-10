use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_hideall(ctx: &Context, msg: &Message) {
    moderation_tools::handle_hideall_unhideall(ctx, msg, true).await;
}

pub struct HideallCommand;
pub static COMMAND_DESCRIPTOR: HideallCommand = HideallCommand;

impl crate::commands::command_contract::CommandSpec for HideallCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "hideall",
            command: "hideall",
            category: "admin",
            params: "aucun",
            summary: "Cache tous les salons",
            description: "Retire la visibilite de tous les salons.",
            examples: &["+hideall"],
            alias_source_key: "hideall",
            default_aliases: &["hda"],
        }
    }
}
