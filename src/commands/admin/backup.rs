use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_backup(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_backup(ctx, msg, args).await;
}

pub struct BackupCommand;
pub static COMMAND_DESCRIPTOR: BackupCommand = BackupCommand;

impl crate::commands::command_contract::CommandSpec for BackupCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "backup",
            command: "backup",
            category: "admin",
            params: "<serveur/emoji> <nom> | list/delete/load",
            summary: "Gere les backups serveur et emojis",
            description: "Cree, liste, supprime et recharge des backups serveur ou emojis.",
            examples: &[
                "+backup serveur prod_1",
                "+backup list serveur",
                "+backup load emoji nightly",
            ],
            alias_source_key: "backup",
            default_aliases: &["bkp"],
            default_permission: 8,
        }
    }
}
