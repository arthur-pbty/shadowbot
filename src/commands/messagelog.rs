use crate::commands::logs_service;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_messagelog(ctx: &Context, msg: &Message, args: &[&str]) {
    logs_service::handle_log_toggle(ctx, msg, args, "message", "MessageLog").await;
}

pub struct MessagelogCommand;
pub static COMMAND_DESCRIPTOR: MessagelogCommand = MessagelogCommand;

impl crate::commands::command_contract::CommandSpec for MessagelogCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "messagelog",
            command: "messagelog",
            category: "admin",
            params: "<on [salon]|off>",
            summary: "Active les logs de messages",
            description: "Active ou desactive les logs des messages supprimes et edites.",
            examples: &["+messagelog on #logs", "+messagelog off"],
            alias_source_key: "messagelog",
            default_aliases: &["msglog"],
        }
    }
}
