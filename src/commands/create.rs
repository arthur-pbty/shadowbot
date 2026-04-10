use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_create(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_create(ctx, msg, args).await;
}

pub struct CreateCommand;
pub static COMMAND_DESCRIPTOR: CreateCommand = CreateCommand;

impl crate::commands::command_contract::CommandSpec for CreateCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "create",
            command: "create",
            category: "admin",
            params: "[emoji/url] [nom]",
            summary: "Cree un emoji custom",
            description: "Cree un emoji custom a partir d'une image, d'un lien ou d'un emoji nitro.",
            examples: &[
                "+create <:blob:123456789012345678> blobcopy",
                "+create https://... logo",
            ],
            alias_source_key: "create",
            default_aliases: &["mkemoji", "ce"],
        }
    }
}
