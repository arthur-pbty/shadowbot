use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_tempmute(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_mute(ctx, msg, args, true).await;
}
pub struct TempmuteCommand;
pub static COMMAND_DESCRIPTOR: TempmuteCommand = TempmuteCommand;
impl crate::commands::command_contract::CommandSpec for TempmuteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "tempmute",
            command: "tempmute",
            category: "admin",
            params: "<@membre/ID[,..]> <duree> [raison]",
            summary: "Mute temporaire",
            description: "Mute un ou plusieurs membres pour une duree donnee.",
            examples: &["+tempmute @User 10m"],
            alias_source_key: "tempmute",
            default_aliases: &["tm"],
            default_permission: 8,
        }
    }
}
