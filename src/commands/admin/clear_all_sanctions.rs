use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::moderation_tools;

pub async fn handle_clear_all_sanctions(ctx: &Context, msg: &Message) {
    moderation_tools::handle_clear_all_sanctions(ctx, msg).await;
}

pub struct ClearAllSanctionsCommand;
pub static COMMAND_DESCRIPTOR: ClearAllSanctionsCommand = ClearAllSanctionsCommand;

impl crate::commands::command_contract::CommandSpec for ClearAllSanctionsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "clear_all_sanctions",
            command: "clear all sanctions",
            category: "admin",
            params: "aucun",
            summary: "Supprime toutes les sanctions du serveur",
            description: "Efface toutes les sanctions de tous les membres du serveur.",
            examples: &["+clear all sanctions"],
            alias_source_key: "clear_all_sanctions",
            default_aliases: &["casanctions"],
            default_permission: 8,
        }
    }
}
