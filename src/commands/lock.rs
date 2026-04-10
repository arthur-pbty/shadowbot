use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_lock(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_lock_unlock(ctx, msg, args, true).await;
}
pub struct LockCommand;
pub static COMMAND_DESCRIPTOR: LockCommand = LockCommand;
impl crate::commands::command_contract::CommandSpec for LockCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "lock",
            command: "lock",
            category: "admin",
            params: "[salon]",
            summary: "Ferme un salon",
            description: "Verrouille un salon texte ou vocal.",
            examples: &["+lock", "+lock #general"],
            alias_source_key: "lock",
            default_aliases: &["lk"],
        }
    }
}
