use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_autobackup(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_autobackup(ctx, msg, args).await;
}

pub struct AutoBackupCommand;
pub static COMMAND_DESCRIPTOR: AutoBackupCommand = AutoBackupCommand;

impl crate::commands::command_contract::CommandSpec for AutoBackupCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "autobackup",
            command: "autobackup",
            category: "admin",
            params: "<serveur/emoji> <jours>",
            summary: "Configure les backups automatiques",
            description: "Definit l'intervalle en jours des backups automatiques.",
            examples: &["+autobackup serveur 3", "+autobackup emoji 7"],
            alias_source_key: "autobackup",
            default_aliases: &["abkp"],
        }
    }
}
