use crate::commands::logs_service;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_raidlog(ctx: &Context, msg: &Message, args: &[&str]) {
    logs_service::handle_raidlog(ctx, msg, args).await;
}

pub struct RaidlogCommand;
pub static COMMAND_DESCRIPTOR: RaidlogCommand = RaidlogCommand;

impl crate::commands::command_contract::CommandSpec for RaidlogCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "raidlog",
            command: "raidlog",
            category: "admin",
            params: "[salon]|off",
            summary: "Active les logs antiraid",
            description: "Active les logs antiraid dans un salon ou les desactive.",
            examples: &["+raidlog #logs", "+raidlog off"],
            alias_source_key: "raidlog",
            default_aliases: &["rdlog"],
        }
    }
}
