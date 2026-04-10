use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_tempban(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_ban(ctx, msg, args, true).await;
}
pub struct TempbanCommand;
pub static COMMAND_DESCRIPTOR: TempbanCommand = TempbanCommand;
impl crate::commands::command_contract::CommandSpec for TempbanCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "tempban",
            command: "tempban",
            category: "admin",
            params: "<@membre/ID[,..]> <duree> [raison]",
            summary: "Ban temporaire",
            description: "Ban temporairement un ou plusieurs membres.",
            examples: &["+tempban @User 1d"],
            alias_source_key: "tempban",
            default_aliases: &["tb"],
            default_permission: 8,
        }
    }
}
