use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::botconfig_common;

pub async fn handle_playto(ctx: &Context, msg: &Message, args: &[&str]) {
    botconfig_common::handle_activity(ctx, msg, "+playto", args).await;
}

pub struct PlaytoCommand;
pub static COMMAND_DESCRIPTOR: PlaytoCommand = PlaytoCommand;

impl crate::commands::command_contract::CommandSpec for PlaytoCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "playto",
            command: "playto",
            category: "profile",
            params: "<texte[, ,texte2,...]>",
            summary: "Definit une activite playing",
            description: "Configure la rotation des messages d activite en mode playing.",
            examples: &["+playto", "+po", "+help playto"],
            alias_source_key: "playto",
            default_aliases: &["ply"],
            default_permission: 8,
        }
    }
}
