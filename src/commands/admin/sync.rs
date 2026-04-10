use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_sync(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_sync(ctx, msg, args).await;
}

pub struct SyncCommand;
pub static COMMAND_DESCRIPTOR: SyncCommand = SyncCommand;

impl crate::commands::command_contract::CommandSpec for SyncCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "sync",
            command: "sync",
            category: "admin",
            params: "<salon/categorie/all>",
            summary: "Synchronise les permissions",
            description: "Synchronise les permissions d'un salon avec sa categorie, ou tous les salons avec all.",
            examples: &["+sync all", "+sync #general"],
            alias_source_key: "sync",
            default_aliases: &["chsync"],
            default_permission: 8,
        }
    }
}
