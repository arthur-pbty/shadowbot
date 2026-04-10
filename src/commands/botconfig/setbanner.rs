use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_setbanner_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::set::handle_setbanner(ctx, msg, args).await;
}

pub struct SetbannerCommand;
pub static COMMAND_DESCRIPTOR: SetbannerCommand = SetbannerCommand;

impl crate::commands::command_contract::CommandSpec for SetbannerCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "setbanner",
            category: "botconfig",
            params: "<url>",
            description: "Modifie la banniere du bot.",
            examples: &["+setbanner https://exemple/banner.png", "+help setbanner"],
            default_aliases: &["stbn"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
