use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_set_modlogs(ctx: &Context, msg: &Message, args: &[&str]) {
    logs_service::handle_set_modlogs(ctx, msg, args).await;
}

pub struct SetModlogsCommand;
pub static COMMAND_DESCRIPTOR: SetModlogsCommand = SetModlogsCommand;

impl crate::commands::command_contract::CommandSpec for SetModlogsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "set_modlogs",
            command: "set modlogs",
            category: "admin",
            params: "[event on/off]",
            summary: "Parametre les evenements de modlogs",
            description: "Affiche ou modifie les evenements qui apparaissent dans les logs de moderation.",
            examples: &["+set modlogs", "+set modlogs warn off"],
            alias_source_key: "set_modlogs",
            default_aliases: &["smodlog"],
        }
    }
}
