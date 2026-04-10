use crate::commands::logs_service;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_modlog(ctx: &Context, msg: &Message, args: &[&str]) {
    logs_service::handle_log_toggle(ctx, msg, args, "moderation", "ModLog").await;
}

pub struct ModlogCommand;
pub static COMMAND_DESCRIPTOR: ModlogCommand = ModlogCommand;

impl crate::commands::command_contract::CommandSpec for ModlogCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "modlog",
            command: "modlog",
            category: "admin",
            params: "<on [salon]|off>",
            summary: "Active les logs de moderation",
            description: "Active ou desactive les logs de moderation dans un salon cible.",
            examples: &["+modlog on #logs", "+modlog off"],
            alias_source_key: "modlog",
            default_aliases: &["mlog"],
        }
    }
}
