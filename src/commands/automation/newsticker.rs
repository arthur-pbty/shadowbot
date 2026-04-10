use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_newsticker(ctx: &Context, msg: &Message, args: &[&str]) {
    let _ = args;
    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("NewSticker")
            .description("Creation de sticker disponible prochainement (API sticker V2).")
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct NewStickerCommand;
pub static COMMAND_DESCRIPTOR: NewStickerCommand = NewStickerCommand;

impl crate::commands::command_contract::CommandSpec for NewStickerCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "newsticker",
            category: "automation",
            params: "[nom]",
            description: "Cree un nouveau sticker a partir d'un sticker ou fichier repondu.",
            examples: &["+newsticker cool_pack"],
            default_aliases: &["stcreate", "nst"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
