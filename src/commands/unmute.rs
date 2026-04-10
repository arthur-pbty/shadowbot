use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_unmute(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_unmute(ctx, msg, args).await;
}
pub struct UnmuteCommand;
pub static COMMAND_DESCRIPTOR: UnmuteCommand = UnmuteCommand;
impl crate::commands::command_contract::CommandSpec for UnmuteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "unmute",
            command: "unmute",
            category: "admin",
            params: "<@membre/ID[,..]>",
            summary: "Retire un mute",
            description: "Met fin au mute d un ou plusieurs membres.",
            examples: &["+unmute @User"],
            alias_source_key: "unmute",
            default_aliases: &["um"],
        }
    }
}
