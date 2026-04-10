use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_mute(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_mute(ctx, msg, args, false).await;
}
pub struct MuteCommand;
pub static COMMAND_DESCRIPTOR: MuteCommand = MuteCommand;
impl crate::commands::command_contract::CommandSpec for MuteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "mute",
            command: "mute",
            category: "admin",
            params: "<@membre/ID[,..]> [raison]",
            summary: "Mute un membre",
            description: "Applique un mute a un ou plusieurs membres.",
            examples: &["+mute @User abus"],
            alias_source_key: "mute",
            default_aliases: &["tmute"],
        }
    }
}
