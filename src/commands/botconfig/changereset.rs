use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_changereset_command(ctx: &Context, msg: &Message, _args: &[&str]) {
    crate::commands::change::handle_changereset(ctx, msg).await;
}

pub struct ChangeresetCommand;
pub static COMMAND_DESCRIPTOR: ChangeresetCommand = ChangeresetCommand;

impl crate::commands::command_contract::CommandSpec for ChangeresetCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "changereset",
            category: "botconfig",
            params: "aucun",
            description: "Reinitialise tous les overrides ACL des commandes.",
            examples: &["+changereset", "+help changereset"],
            default_aliases: &["chgr"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
