use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_warn(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_warn(ctx, msg, args).await;
}
pub struct WarnCommand;
pub static COMMAND_DESCRIPTOR: WarnCommand = WarnCommand;
impl crate::commands::command_contract::CommandSpec for WarnCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "warn",
            command: "warn",
            category: "admin",
            params: "<@membre/ID[,..]> [raison]",
            summary: "Donne un warn",
            description: "Ajoute un warn a un ou plusieurs membres.",
            examples: &["+warn @User spam"],
            alias_source_key: "warn",
            default_aliases: &["avert"],
        }
    }
}
