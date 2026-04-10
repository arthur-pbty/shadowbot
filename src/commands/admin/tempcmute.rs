use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_tempcmute(ctx: &Context, msg: &Message, args: &[&str]) {
    moderation_tools::handle_cmute(ctx, msg, args, true).await;
}
pub struct TempcmuteCommand;
pub static COMMAND_DESCRIPTOR: TempcmuteCommand = TempcmuteCommand;
impl crate::commands::command_contract::CommandSpec for TempcmuteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "tempcmute",
            command: "tempcmute",
            category: "admin",
            params: "<@membre/ID[,..]> <duree> [raison]",
            summary: "Mute salon temporaire",
            description: "Mute temporaire sur le salon courant.",
            examples: &["+tempcmute @User 5m"],
            alias_source_key: "tempcmute",
            default_aliases: &["tcm"],
            default_permission: 8,
        }
    }
}
