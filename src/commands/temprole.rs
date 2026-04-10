use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_temprole(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_temprole(ctx, msg, args).await;
}

pub struct TempRoleCommand;
pub static COMMAND_DESCRIPTOR: TempRoleCommand = TempRoleCommand;

impl crate::commands::command_contract::CommandSpec for TempRoleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "temprole",
            command: "temprole",
            category: "admin",
            params: "<membre> <role> <duree>",
            summary: "Ajoute un role temporaire",
            description: "Attribue un role a un membre pour une duree donnee puis le retire automatiquement.",
            examples: &["+temprole @User @VIP 2h"],
            alias_source_key: "temprole",
            default_aliases: &["trole", "tmprole"],
        }
    }
}
