use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_newsticker(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_new_sticker(ctx, msg, args).await;
}

pub struct NewStickerCommand;
pub static COMMAND_DESCRIPTOR: NewStickerCommand = NewStickerCommand;

impl crate::commands::command_contract::CommandSpec for NewStickerCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "newsticker",
            command: "newsticker",
            category: "admin",
            params: "[nom]",
            summary: "Cree un sticker serveur",
            description: "Cree un nouveau sticker a partir d'un sticker ou fichier repondu.",
            examples: &["+newsticker cool_pack"],
            alias_source_key: "newsticker",
            default_aliases: &["stcreate", "nst"],
            default_permission: 8,
        }
    }
}
