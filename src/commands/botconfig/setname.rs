use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_setname_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::set::handle_setname(ctx, msg, args).await;
}

pub struct SetnameCommand;
pub static COMMAND_DESCRIPTOR: SetnameCommand = SetnameCommand;

impl crate::commands::command_contract::CommandSpec for SetnameCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "setname",
            category: "botconfig",
            params: "<nom>",
            description: "Modifie le nom du bot.",
            examples: &["+setname MonBot", "+help setname"],
            default_aliases: &["stn"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
