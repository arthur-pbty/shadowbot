use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_addrole(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_add_del_role(ctx, msg, args, true).await;
}

pub struct AddroleCommand;
pub static COMMAND_DESCRIPTOR: AddroleCommand = AddroleCommand;

impl crate::commands::command_contract::CommandSpec for AddroleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "addrole",
            command: "addrole",
            category: "admin",
            params: "<@membre/ID[,..]> <@role/ID>",
            summary: "Ajoute un role",
            description: "Ajoute un role a un ou plusieurs membres.",
            examples: &["+addrole @User @Membre"],
            alias_source_key: "addrole",
            default_aliases: &["ar"],
            default_permission: 8,
        }
    }
}
