use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed, CreateMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_button(ctx: &Context, msg: &Message, args: &[&str]) {
    if args.len() < 2 {
        return;
    }

    let action = args[0].to_lowercase();
    let link = args[1];

    if action == "add" {
        let _ = msg
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new()
                    .content("Bouton personnalisé")
                    .components(vec![CreateActionRow::Buttons(vec![
                        CreateButton::new_link(link).label("Ouvrir"),
                    ])]),
            )
            .await;
    } else if action == "del" {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Button")
                .description("Suppression: supprime simplement le message contenant le bouton.")
                .color(theme_color(ctx).await),
        )
        .await;
    }
}

pub struct ButtonCommand;
pub static COMMAND_DESCRIPTOR: ButtonCommand = ButtonCommand;

impl crate::commands::command_contract::CommandSpec for ButtonCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "button",
            category: "automation",
            params: "<add/del> <lien>",
            description: "Ajoute ou supprime un bouton de decoration personnalise sur un message du bot.",
            examples: &[
                "+button add https://example.com",
                "+button del https://example.com",
            ],
            default_aliases: &["btn"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
