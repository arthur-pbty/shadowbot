use crate::commands::logs_service;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_rolelog(ctx: &Context, msg: &Message, args: &[&str]) {
    logs_service::handle_log_toggle(ctx, msg, args, "role", "RoleLog").await;
}

pub struct RolelogCommand;
pub static COMMAND_DESCRIPTOR: RolelogCommand = RolelogCommand;

impl crate::commands::command_contract::CommandSpec for RolelogCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "rolelog",
            command: "rolelog",
            category: "admin",
            params: "<on [salon]|off>",
            summary: "Active les logs de roles",
            description: "Active ou desactive les logs des roles.",
            examples: &["+rolelog on #logs", "+rolelog off"],
            alias_source_key: "rolelog",
            default_aliases: &["rlog"],
        }
    }
}
