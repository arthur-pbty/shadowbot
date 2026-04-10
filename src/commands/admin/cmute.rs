use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_cmute(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_cmute(ctx, msg, args, false).await;
}
pub struct CmuteCommand;
pub static COMMAND_DESCRIPTOR: CmuteCommand = CmuteCommand;
impl crate::commands::command_contract::CommandSpec for CmuteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "cmute",
            command: "cmute",
            category: "admin",
            params: "<@membre/ID[,..]> [raison]",
            summary: "Mute salon",
            description: "Mute un membre sur le salon courant.",
            examples: &["+cmute @User"],
            alias_source_key: "cmute",
            default_aliases: &["cm"],
            default_permission: 8,
        }
    }
}
