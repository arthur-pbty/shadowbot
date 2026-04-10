use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_autopublishoff_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::autopublish::handle_autopublishoff(ctx, msg, args).await;
}

pub struct AutopublishoffCommand;
pub static COMMAND_DESCRIPTOR: AutopublishoffCommand = AutopublishoffCommand;

impl crate::commands::command_contract::CommandSpec for AutopublishoffCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "autopublishoff",
            category: "automation",
            params: "[#canal]",
            description: "Desactive la publication automatique des annonces sur un salon.",
            examples: &["+autopublishoff", "+autopublishoff #annonces", "+help autopublishoff"],
            default_aliases: &["apboff"],
            allow_in_dm: false,
            default_permission: 5,
        }
    }
}
