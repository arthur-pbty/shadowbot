use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_uncmute(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_uncmute(ctx, msg, args).await;
}
pub struct UncmuteCommand;
pub static COMMAND_DESCRIPTOR: UncmuteCommand = UncmuteCommand;
impl crate::commands::command_contract::CommandSpec for UncmuteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "uncmute",
            command: "uncmute",
            category: "admin",
            params: "<@membre/ID[,..]>",
            summary: "Retire un cmute",
            description: "Met fin au mute salon.",
            examples: &["+uncmute @User"],
            alias_source_key: "uncmute",
            default_aliases: &["ucm"],
        }
    }
}
