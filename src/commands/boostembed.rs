use crate::commands::logs_service;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_boostembed(ctx: &Context, msg: &Message, args: &[&str]) {
    logs_service::handle_boostembed(ctx, msg, args).await;
}

pub struct BoostembedCommand;
pub static COMMAND_DESCRIPTOR: BoostembedCommand = BoostembedCommand;

impl crate::commands::command_contract::CommandSpec for BoostembedCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "boostembed",
            command: "boostembed",
            category: "admin",
            params: "<on|off|test>",
            summary: "Active, coupe ou teste l embed boost",
            description: "Controle l embed de boost et permet un test rapide.",
            examples: &["+boostembed on", "+boostembed test"],
            alias_source_key: "boostembed",
            default_aliases: &["bembed"],
        }
    }
}
