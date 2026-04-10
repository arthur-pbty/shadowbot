use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_unhide(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_hide_unhide(ctx, msg, args, false).await;
}
pub struct UnhideCommand;
pub static COMMAND_DESCRIPTOR: UnhideCommand = UnhideCommand;
impl crate::commands::command_contract::CommandSpec for UnhideCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "unhide",
            command: "unhide",
            category: "admin",
            params: "[salon]",
            summary: "Affiche un salon",
            description: "Rend a nouveau visible un salon.",
            examples: &["+unhide", "+unhide #general"],
            alias_source_key: "unhide",
            default_aliases: &["uhd"],
            default_permission: 8,
        }
    }
}
