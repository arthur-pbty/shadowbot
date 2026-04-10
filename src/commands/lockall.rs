use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_lockall(ctx: &Context, msg: &Message) {
    moderation_tools::handle_lockall_unlockall(ctx, msg, true).await;
}
pub struct LockallCommand;
pub static COMMAND_DESCRIPTOR: LockallCommand = LockallCommand;
impl crate::commands::command_contract::CommandSpec for LockallCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "lockall",
            command: "lockall",
            category: "admin",
            params: "aucun",
            summary: "Ferme tous les salons",
            description: "Verrouille tous les salons du serveur.",
            examples: &["+lockall"],
            alias_source_key: "lockall",
            default_aliases: &["lka"],
        }
    }
}
