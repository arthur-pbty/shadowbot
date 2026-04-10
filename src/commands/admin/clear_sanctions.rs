use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::moderation_tools;

pub async fn handle_clear_sanctions(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_clear_sanctions(ctx, msg, args).await;
}

pub struct ClearSanctionsCommand;
pub static COMMAND_DESCRIPTOR: ClearSanctionsCommand = ClearSanctionsCommand;

impl crate::commands::command_contract::CommandSpec for ClearSanctionsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "clear_sanctions",
            command: "clear sanctions",
            category: "admin",
            params: "<@membre/ID>",
            summary: "Supprime toutes les sanctions d un membre",
            description: "Efface completement les sanctions d un membre cible.",
            examples: &["+clear sanctions @User"],
            alias_source_key: "clear_sanctions",
            default_aliases: &["csanctions"],
            default_permission: 8,
        }
    }
}
