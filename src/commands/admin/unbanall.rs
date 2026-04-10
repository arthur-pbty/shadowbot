use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_unbanall(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_unbanall(ctx, msg, args).await;
}

pub struct UnbanAllCommand;
pub static COMMAND_DESCRIPTOR: UnbanAllCommand = UnbanAllCommand;

impl crate::commands::command_contract::CommandSpec for UnbanAllCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "unbanall",
            command: "unbanall",
            category: "admin",
            params: "aucun",
            summary: "Retire tous les bannissements",
            description: "Supprime tous les bans du serveur cible.",
            examples: &["+unbanall"],
            alias_source_key: "unbanall",
            default_aliases: &["uball", "clearbans"],
            default_permission: 8,
        }
    }
}
