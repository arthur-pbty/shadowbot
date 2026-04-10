use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_hide(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_hide_unhide(ctx, msg, args, true).await;
}
pub struct HideCommand;
pub static COMMAND_DESCRIPTOR: HideCommand = HideCommand;
impl crate::commands::command_contract::CommandSpec for HideCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "hide",
            command: "hide",
            category: "admin",
            params: "[salon]",
            summary: "Cache un salon",
            description: "Retire la visibilite d un salon.",
            examples: &["+hide", "+hide #general"],
            alias_source_key: "hide",
            default_aliases: &["hd"],
            default_permission: 8,
        }
    }
}
