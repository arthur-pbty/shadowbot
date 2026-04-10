use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_autoreact(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_autoreact(ctx, msg, args).await;
}

pub struct AutoReactCommand;
pub static COMMAND_DESCRIPTOR: AutoReactCommand = AutoReactCommand;

impl crate::commands::command_contract::CommandSpec for AutoReactCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "autoreact",
            command: "autoreact",
            category: "admin",
            params: "<add/del> <salon> <emoji> | list",
            summary: "Configure les reactions automatiques",
            description: "Ajoute, retire et liste les reactions automatiquement appliquees aux messages d'un salon.",
            examples: &["+autoreact add #general 😀", "+autoreact list"],
            alias_source_key: "autoreact",
            default_aliases: &["ar", "reactauto"],
            default_permission: 8,
        }
    }
}
