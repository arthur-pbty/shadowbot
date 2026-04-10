use crate::commands::logs_service;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_voicelog(ctx: &Context, msg: &Message, args: &[&str]) {
    logs_service::handle_log_toggle(ctx, msg, args, "voice", "VoiceLog").await;
}

pub struct VoicelogCommand;
pub static COMMAND_DESCRIPTOR: VoicelogCommand = VoicelogCommand;

impl crate::commands::command_contract::CommandSpec for VoicelogCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "voicelog",
            command: "voicelog",
            category: "admin",
            params: "<on [salon]|off>",
            summary: "Active les logs vocaux",
            description: "Active ou desactive les logs de l activite vocale.",
            examples: &["+voicelog on #logs", "+voicelog off"],
            alias_source_key: "voicelog",
            default_aliases: &["vlog"],
            default_permission: 8,
        }
    }
}
