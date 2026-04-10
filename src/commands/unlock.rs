use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_unlock(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_lock_unlock(ctx, msg, args, false).await;
}
pub struct UnlockCommand;
pub static COMMAND_DESCRIPTOR: UnlockCommand = UnlockCommand;
impl crate::commands::command_contract::CommandSpec for UnlockCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "unlock",
            command: "unlock",
            category: "admin",
            params: "[salon]",
            summary: "Ouvre un salon",
            description: "Deverrouille un salon texte ou vocal.",
            examples: &["+unlock", "+unlock #general"],
            alias_source_key: "unlock",
            default_aliases: &["ulk"],
        }
    }
}
