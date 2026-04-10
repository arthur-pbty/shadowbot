use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::botconfig_common;

pub async fn handle_stream(ctx: &Context, msg: &Message, args: &[&str]) {
    botconfig_common::handle_activity(ctx, msg, "+stream", args).await;
}

pub struct StreamCommand;
pub static COMMAND_DESCRIPTOR: StreamCommand = StreamCommand;

impl crate::commands::command_contract::CommandSpec for StreamCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "stream",
            command: "stream",
            category: "profile",
            params: "<texte[, ,texte2,...]>",
            summary: "Definit une activite streaming",
            description: "Configure la rotation des messages d activite en mode streaming.",
            examples: &["+stream", "+sm", "+help stream"],
            alias_source_key: "stream",
            default_aliases: &["stm"],
        }
    }
}
