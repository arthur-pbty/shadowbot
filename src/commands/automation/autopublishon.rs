use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_autopublishon_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::autopublish::handle_autopublishon(ctx, msg, args).await;
}

pub struct AutopublishonCommand;
pub static COMMAND_DESCRIPTOR: AutopublishonCommand = AutopublishonCommand;

impl crate::commands::command_contract::CommandSpec for AutopublishonCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "autopublishon",
            category: "automation",
            params: "[#canal]",
            description: "Active la publication automatique des annonces sur un salon.",
            examples: &["+autopublishon", "+autopublishon #annonces", "+help autopublishon"],
            default_aliases: &["apbon"],
            allow_in_dm: false,
            default_permission: 5,
        }
    }
}
