use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_unlockall(ctx: &Context, msg: &Message) {
    moderation_tools::handle_lockall_unlockall(ctx, msg, false).await;
}
pub struct UnlockallCommand;
pub static COMMAND_DESCRIPTOR: UnlockallCommand = UnlockallCommand;
impl crate::commands::command_contract::CommandSpec for UnlockallCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "unlockall",
            command: "unlockall",
            category: "admin",
            params: "aucun",
            summary: "Ouvre tous les salons",
            description: "Deverrouille tous les salons du serveur.",
            examples: &["+unlockall"],
            alias_source_key: "unlockall",
            default_aliases: &["ulka"],
            default_permission: 8,
        }
    }
}
