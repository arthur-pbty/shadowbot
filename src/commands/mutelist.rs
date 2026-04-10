use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_mutelist(ctx: &Context, msg: &Message) {
    moderation_tools::handle_mutelist(ctx, msg).await;
}
pub struct MutelistCommand;
pub static COMMAND_DESCRIPTOR: MutelistCommand = MutelistCommand;
impl crate::commands::command_contract::CommandSpec for MutelistCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "mutelist",
            command: "mutelist",
            category: "admin",
            params: "aucun",
            summary: "Liste les mutes",
            description: "Affiche tous les mutes en cours.",
            examples: &["+mutelist"],
            alias_source_key: "mutelist",
            default_aliases: &["ml"],
        }
    }
}
