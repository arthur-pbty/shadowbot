use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed, CreateMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::theme_color;

pub async fn handle_rickroll(ctx: &Context, msg: &Message, _args: &[&str]) {
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    let embed = CreateEmbed::new()
        .title("Rickroll")
        .description("Never gonna give you up.")
        .color(theme_color(ctx).await);

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .embed(embed)
                .components(vec![CreateActionRow::Buttons(vec![
                    CreateButton::new_link(url).label("Ouvrir la video"),
                ])]),
        )
        .await;
}

pub struct RickrollCommand;
pub static COMMAND_DESCRIPTOR: RickrollCommand = RickrollCommand;

impl crate::commands::command_contract::CommandSpec for RickrollCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "rickroll",
            category: "game",
            params: "aucun",
            description: "Never gonna give you up.",
            examples: &["+rickroll"],
            default_aliases: &["rr"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
