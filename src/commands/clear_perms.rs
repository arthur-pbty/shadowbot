use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::perms_service;

pub async fn handle_clear_perms(ctx: &Context, msg: &Message) {
    perms_service::handle_clear_perms(ctx, msg).await;
}

pub struct ClearPermsCommand;
pub static COMMAND_DESCRIPTOR: ClearPermsCommand = ClearPermsCommand;

impl crate::commands::command_contract::CommandSpec for ClearPermsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "clear_perms",
            command: "clear perms",
            category: "permissions",
            params: "aucun",
            summary: "Vide toutes les permissions scope",
            description: "Supprime toutes les permissions ACL configurees en base.",
            examples: &["+clear perms", "+cs", "+help clear perms"],
            alias_source_key: "clear_perms",
            default_aliases: &["cpm"],
        }
    }
}
