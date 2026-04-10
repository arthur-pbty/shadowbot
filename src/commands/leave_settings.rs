use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_leave_settings(ctx: &Context, msg: &Message, args: &[&str]) {
    logs_service::handle_join_leave_settings(ctx, msg, args, "leave").await;
}

pub struct LeaveSettingsCommand;
pub static COMMAND_DESCRIPTOR: LeaveSettingsCommand = LeaveSettingsCommand;

impl crate::commands::command_contract::CommandSpec for LeaveSettingsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "leave_settings",
            command: "leave settings",
            category: "admin",
            params: "settings [on/off] [salon] [message]",
            summary: "Parametre les actions de leave",
            description: "Configure les actions a executer quand un membre quitte le serveur.",
            examples: &[
                "+leave settings",
                "+leave settings on #logs {user} a quitte",
            ],
            alias_source_key: "leave_settings",
            default_aliases: &["lset"],
        }
    }
}
