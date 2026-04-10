use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_voicekick(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_voicekick(ctx, msg, args).await;
}

pub struct VoiceKickCommand;
pub static COMMAND_DESCRIPTOR: VoiceKickCommand = VoiceKickCommand;

impl crate::commands::command_contract::CommandSpec for VoiceKickCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "voicekick",
            command: "voicekick",
            category: "admin",
            params: "<membre...>",
            summary: "Deconnecte des membres du vocal",
            description: "Deconnecte un ou plusieurs membres de leur salon vocal actuel.",
            examples: &["+voicekick @User", "+voicekick @U1 @U2"],
            alias_source_key: "voicekick",
            default_aliases: &["vk", "vdisconnect"],
            default_permission: 8,
        }
    }
}
