use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_kick(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_kick(ctx, msg, args).await;
}
pub struct KickCommand;
pub static COMMAND_DESCRIPTOR: KickCommand = KickCommand;
impl crate::commands::command_contract::CommandSpec for KickCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "kick",
            command: "kick",
            category: "admin",
            params: "<@membre/ID[,..]> [raison]",
            summary: "Expulse un membre",
            description: "Kick un ou plusieurs membres.",
            examples: &["+kick @User"],
            alias_source_key: "kick",
            default_aliases: &["k"],
            default_permission: 8,
        }
    }
}
