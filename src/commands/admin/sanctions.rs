use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::moderation_tools;

pub async fn handle_sanctions(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_sanctions(ctx, msg, args).await;
}

pub struct SanctionsCommand;
pub static COMMAND_DESCRIPTOR: SanctionsCommand = SanctionsCommand;

impl crate::commands::command_contract::CommandSpec for SanctionsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "sanctions",
            command: "sanctions",
            category: "admin",
            params: "<@membre/ID>",
            summary: "Affiche les sanctions d un membre",
            description: "Liste l historique des sanctions d un membre.",
            examples: &["+sanctions @User"],
            alias_source_key: "sanctions",
            default_aliases: &["sanct"],
            default_permission: 8,
        }
    }
}
