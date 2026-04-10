use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_voicemove(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_voicemove(ctx, msg, args).await;
}

pub struct VoiceMoveCommand;
pub static COMMAND_DESCRIPTOR: VoiceMoveCommand = VoiceMoveCommand;

impl crate::commands::command_contract::CommandSpec for VoiceMoveCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "voicemove",
            command: "voicemove",
            category: "admin",
            params: "<salon_source> <salon_destination>",
            summary: "Deplace les membres vocaux",
            description: "Deplace tous les membres d'un salon vocal vers un autre salon.",
            examples: &["+voicemove #General #Event"],
            alias_source_key: "voicemove",
            default_aliases: &["vmove", "vmoveall"],
        }
    }
}
