use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_renew(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_renew(ctx, msg, args).await;
}

pub struct RenewCommand;
pub static COMMAND_DESCRIPTOR: RenewCommand = RenewCommand;

impl crate::commands::command_contract::CommandSpec for RenewCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "renew",
            command: "renew",
            category: "admin",
            params: "[salon]",
            summary: "Recree un salon textuel",
            description: "Supprime puis recree un salon textuel en conservant les options principales.",
            examples: &["+renew", "+renew #general"],
            alias_source_key: "renew",
            default_aliases: &["nuke", "rebuildch"],
        }
    }
}
