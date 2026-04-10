use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::perms_service;

pub async fn handle_change(ctx: &Context, msg: &Message, args: &[&str]) {
    perms_service::handle_change(ctx, msg, args).await;
}

pub struct ChangeCommand;
pub static COMMAND_DESCRIPTOR: ChangeCommand = ChangeCommand;

impl crate::commands::command_contract::CommandSpec for ChangeCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "change",
            command: "change",
            category: "permissions",
            params: "<commande> <niveau 0-9> | reset",
            summary: "Change un niveau de permission",
            description: "Definit le niveau ACL requis pour une commande ou reinitialise les overrides.",
            examples: &["+change", "+ce", "+help change"],
            alias_source_key: "change",
            default_aliases: &["chg"],
            default_permission: 9,
        }
    }
}
