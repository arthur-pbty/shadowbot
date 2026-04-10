use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_embed(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_embed_builder(ctx, msg, args).await;
}

pub struct EmbedCommand;
pub static COMMAND_DESCRIPTOR: EmbedCommand = EmbedCommand;

impl crate::commands::command_contract::CommandSpec for EmbedCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "embed",
            command: "embed",
            category: "admin",
            params: "title | description (v1)",
            summary: "Ouvre le generateur d'embed",
            description: "Affiche un generateur d'embed interactif version rapide.",
            examples: &["+embed"],
            alias_source_key: "embed",
            default_aliases: &["emb"],
        }
    }
}
