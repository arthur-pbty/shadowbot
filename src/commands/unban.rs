use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_unban(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_unban(ctx, msg, args).await;
}
pub struct UnbanCommand;
pub static COMMAND_DESCRIPTOR: UnbanCommand = UnbanCommand;
impl crate::commands::command_contract::CommandSpec for UnbanCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "unban",
            command: "unban",
            category: "admin",
            params: "<@membre/ID[,..]>",
            summary: "Retire un ban",
            description: "Unban un ou plusieurs membres.",
            examples: &["+unban @User"],
            alias_source_key: "unban",
            default_aliases: &["ub"],
        }
    }
}
