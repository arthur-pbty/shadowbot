use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::perms_service;

pub async fn handle_perms(ctx: &Context, msg: &Message, args: &[&str]) {
    perms_service::handle_perms(ctx, msg, args).await;
}

pub struct PermsCommand;
pub static COMMAND_DESCRIPTOR: PermsCommand = PermsCommand;

impl crate::commands::command_contract::CommandSpec for PermsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "perms",
            command: "perms",
            category: "permissions",
            params: "aucun",
            summary: "Affiche les permissions ACL",
            description: "Affiche les permissions ACL configurees par role ou scope.",
            examples: &["+perms", "+ps", "+help perms"],
            alias_source_key: "perms",
            default_aliases: &["prm"],
            default_permission: 8,
        }
    }
}
