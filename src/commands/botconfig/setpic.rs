use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_setpic_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::set::handle_setpic(ctx, msg, args).await;
}

pub struct SetpicCommand;
pub static COMMAND_DESCRIPTOR: SetpicCommand = SetpicCommand;

impl crate::commands::command_contract::CommandSpec for SetpicCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "setpic",
            category: "botconfig",
            params: "<url>",
            description: "Modifie l'avatar du bot.",
            examples: &["+setpic https://exemple/image.png", "+help setpic"],
            default_aliases: &["stpic"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
