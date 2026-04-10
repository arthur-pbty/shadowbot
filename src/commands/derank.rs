use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_derank(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_derank(ctx, msg, args).await;
}

pub struct DerankCommand;
pub static COMMAND_DESCRIPTOR: DerankCommand = DerankCommand;

impl crate::commands::command_contract::CommandSpec for DerankCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "derank",
            command: "derank",
            category: "admin",
            params: "<@membre/ID[,..]>",
            summary: "Retire tous les roles",
            description: "Retire tous les roles gerables d un membre.",
            examples: &["+derank @User"],
            alias_source_key: "derank",
            default_aliases: &["drk"],
        }
    }
}
