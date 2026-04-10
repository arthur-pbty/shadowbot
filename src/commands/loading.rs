use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_loading(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_loading(ctx, msg, args).await;
}

pub struct LoadingCommand;
pub static COMMAND_DESCRIPTOR: LoadingCommand = LoadingCommand;

impl crate::commands::command_contract::CommandSpec for LoadingCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "loading",
            command: "loading",
            category: "general",
            params: "<duree> <message>",
            summary: "Affiche une barre de chargement",
            description: "Anime une barre de progression avec la duree et le texte fournis.",
            examples: &["+loading 10s Traitement en cours"],
            alias_source_key: "loading",
            default_aliases: &["loadbar", "bar"],
        }
    }
}
