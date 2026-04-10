use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::perms_service;

pub async fn handle_del(ctx: &Context, msg: &Message, args: &[&str]) {
    perms_service::handle_del_perm(ctx, msg, args).await;
}

pub struct DelCommand;
pub static COMMAND_DESCRIPTOR: DelCommand = DelCommand;

impl crate::commands::command_contract::CommandSpec for DelCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "del",
            command: "del",
            category: "permissions",
            params: "perm <@&rôle/@membre/ID>",
            summary: "Supprime des permissions scope",
            description: "Supprime les permissions ACL associees a un role ou utilisateur.",
            examples: &["+del", "+dl", "+help del"],
            alias_source_key: "del",
            default_aliases: &["dlp"],
        }
    }
}
