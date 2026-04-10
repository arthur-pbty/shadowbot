use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_setprofil_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::set::handle_setprofil(ctx, msg, args).await;
}

pub struct SetprofilCommand;
pub static COMMAND_DESCRIPTOR: SetprofilCommand = SetprofilCommand;

impl crate::commands::command_contract::CommandSpec for SetprofilCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "setprofil",
            category: "botconfig",
            params: "<nom> ;; <url_pic> ;; <url_banner>",
            description: "Met a jour en une commande le nom, l'avatar et la banniere du bot.",
            examples: &[
                "+setprofil Shadow ;; https://img/a.png ;; https://img/b.png",
                "+help setprofil",
            ],
            default_aliases: &["stpr"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
