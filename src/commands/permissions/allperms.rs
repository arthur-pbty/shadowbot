use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::perms_service;

pub async fn handle_allperms(ctx: &Context, msg: &Message, args: &[&str]) {
    perms_service::handle_allperms(ctx, msg, args).await;
}

pub struct AllpermsCommand;
pub static COMMAND_DESCRIPTOR: AllpermsCommand = AllpermsCommand;

impl crate::commands::command_contract::CommandSpec for AllpermsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "allperms",
            command: "allperms",
            category: "permissions",
            params: "[page]",
            summary: "Liste les ACL de toutes commandes",
            description: "Affiche le niveau ACL requis pour chaque commande avec pagination.",
            examples: &["+allperms", "+as", "+help allperms"],
            alias_source_key: "allperms",
            default_aliases: &["apm"],
            default_permission: 8,
        }
    }
}
