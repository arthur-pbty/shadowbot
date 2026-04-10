use crate::commands::logs_service;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_boostlog(ctx: &Context, msg: &Message, args: &[&str]) {
    logs_service::handle_log_toggle(ctx, msg, args, "boost", "BoostLog").await;
}

pub struct BoostlogCommand;
pub static COMMAND_DESCRIPTOR: BoostlogCommand = BoostlogCommand;

impl crate::commands::command_contract::CommandSpec for BoostlogCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "boostlog",
            command: "boostlog",
            category: "admin",
            params: "<on [salon]|off>",
            summary: "Active les logs de boosts",
            description: "Active ou desactive les logs de boosts.",
            examples: &["+boostlog on #logs", "+boostlog off"],
            alias_source_key: "boostlog",
            default_aliases: &["blog"],
        }
    }
}
