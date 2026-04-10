use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_setperm_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::set::handle_setperm(ctx, msg, args).await;
}

pub struct SetpermCommand;
pub static COMMAND_DESCRIPTOR: SetpermCommand = SetpermCommand;

impl crate::commands::command_contract::CommandSpec for SetpermCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "setperm",
            category: "botconfig",
            params: "<permission/commande> <role/membre>",
            description: "Attribue un niveau ACL ou un acces commande a un role ou membre.",
            examples: &["+setperm 6 @Moderateur", "+setperm mute @Role", "+help setperm"],
            default_aliases: &["stp"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
