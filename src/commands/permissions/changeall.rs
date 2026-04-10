use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::perms_service;

pub async fn handle_changeall(ctx: &Context, msg: &Message, args: &[&str]) {
    perms_service::handle_changeall(ctx, msg, args).await;
}

pub struct ChangeallCommand;
pub static COMMAND_DESCRIPTOR: ChangeallCommand = ChangeallCommand;

impl crate::commands::command_contract::CommandSpec for ChangeallCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "changeall",
            command: "changeall",
            category: "permissions",
            params: "<niveau_source 0-9> <niveau_cible 0-9>",
            summary: "Change des permissions en masse",
            description: "Remplace en masse un niveau ACL source par un niveau ACL cible.",
            examples: &["+changeall", "+cl", "+help changeall"],
            alias_source_key: "changeall",
            default_aliases: &["cga"],
            default_permission: 9,
        }
    }
}
