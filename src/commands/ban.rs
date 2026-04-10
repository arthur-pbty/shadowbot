use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_ban(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_ban(ctx, msg, args, false).await;
}
pub struct BanCommand;
pub static COMMAND_DESCRIPTOR: BanCommand = BanCommand;
impl crate::commands::command_contract::CommandSpec for BanCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "ban",
            command: "ban",
            category: "admin",
            params: "<@membre/ID[,..]> [raison]",
            summary: "Bannit un membre",
            description: "Ban un ou plusieurs membres.",
            examples: &["+ban @User"],
            alias_source_key: "ban",
            default_aliases: &["b"],
        }
    }
}
