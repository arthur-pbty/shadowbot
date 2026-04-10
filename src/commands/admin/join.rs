use crate::commands::logs_service;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_join(ctx: &Context, msg: &Message, args: &[&str]) {
    logs_service::handle_join_leave_settings(ctx, msg, args, "join").await;
}

pub struct JoinCommand;
pub static COMMAND_DESCRIPTOR: JoinCommand = JoinCommand;

impl crate::commands::command_contract::CommandSpec for JoinCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "join",
            command: "join",
            category: "admin",
            params: "settings [on/off] [salon] [message]",
            summary: "Parametre les actions de join",
            description: "Permet de configurer les actions quand un membre rejoint.",
            examples: &[
                "+join settings",
                "+join settings on #welcome Bienvenue {user}",
            ],
            alias_source_key: "join",
            default_aliases: &["jset"],
            default_permission: 8,
        }
    }
}
