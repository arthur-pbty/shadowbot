use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_shadowbot(ctx: &Context, msg: &Message, _args: &[&str]) {
    let invite = "https://discord.gg/NbMWHh54bp";
    let color = theme_color(ctx).await;

    let embed = CreateEmbed::new()
        .title("Support Shadowbot")
        .description(format!("Invitation: {}", invite))
        .color(color);

    send_embed(ctx, msg, embed).await;
}

pub struct ShadowbotCommand;
pub static COMMAND_DESCRIPTOR: ShadowbotCommand = ShadowbotCommand;

impl crate::commands::command_contract::CommandSpec for ShadowbotCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "shadowbot",
            category: "bot",
            params: "aucun",
            description: "Affiche les informations globales et letat du bot.",
            examples: &["+shadowbot", "+st", "+help shadowbot"],
            default_aliases: &["sbt"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
