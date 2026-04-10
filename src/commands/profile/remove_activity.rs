use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::botconfig_common;

pub async fn handle_remove_activity(ctx: &Context, msg: &Message) {
    botconfig_common::handle_remove_activity(ctx, msg).await;
}

pub struct RemoveActivityCommand;
pub static COMMAND_DESCRIPTOR: RemoveActivityCommand = RemoveActivityCommand;

impl crate::commands::command_contract::CommandSpec for RemoveActivityCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "remove_activity",
            command: "remove activity",
            category: "profile",
            params: "aucun",
            summary: "Supprime lactivite du bot",
            description: "Arrete la rotation d activite et retire lactivite courante du bot.",
            examples: &["+remove activity", "+ry", "+help remove activity"],
            alias_source_key: "remove_activity",
            default_aliases: &["rma"],
            default_permission: 8,
        }
    }
}
