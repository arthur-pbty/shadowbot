use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_unmuteall(ctx: &Context, msg: &Message) {
    moderation_tools::handle_unmuteall(ctx, msg).await;
}
pub struct UnmuteallCommand;
pub static COMMAND_DESCRIPTOR: UnmuteallCommand = UnmuteallCommand;
impl crate::commands::command_contract::CommandSpec for UnmuteallCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "unmuteall",
            command: "unmuteall",
            category: "admin",
            params: "aucun",
            summary: "Retire tous les mutes",
            description: "Supprime tous les mutes en cours.",
            examples: &["+unmuteall"],
            alias_source_key: "unmuteall",
            default_aliases: &["uma"],
        }
    }
}
