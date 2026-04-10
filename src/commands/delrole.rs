use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_delrole(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_add_del_role(ctx, msg, args, false).await;
}

pub struct DelroleCommand;
pub static COMMAND_DESCRIPTOR: DelroleCommand = DelroleCommand;

impl crate::commands::command_contract::CommandSpec for DelroleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "delrole",
            command: "delrole",
            category: "admin",
            params: "<@membre/ID[,..]> <@role/ID>",
            summary: "Retire un role",
            description: "Retire un role a un ou plusieurs membres.",
            examples: &["+delrole @User @Membre"],
            alias_source_key: "delrole",
            default_aliases: &["dr"],
        }
    }
}
