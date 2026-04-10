use rand::seq::SliceRandom;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_pendu(ctx: &Context, msg: &Message, _args: &[&str]) {
    let words = [
        "discord",
        "shadowbot",
        "rust",
        "moderation",
        "serenity",
        "serveur",
    ];

    let word = {
        let mut rng = rand::thread_rng();
        words.choose(&mut rng).copied().unwrap_or("rust")
    };
    let mut chars = word.chars().collect::<Vec<_>>();

    if chars.len() > 2 {
        for index in 1..chars.len() - 1 {
            chars[index] = '_';
        }
    }

    let masked = chars
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    let embed = CreateEmbed::new()
        .title("Pendu")
        .description(format!(
            "Mot mystere: **{}**\nIndice: {} lettres.",
            masked,
            word.chars().count()
        ))
        .field(
            "Astuce",
            "Tu peux jouer en discutant les propositions dans le salon.",
            false,
        )
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct PenduCommand;
pub static COMMAND_DESCRIPTOR: PenduCommand = PenduCommand;

impl crate::commands::command_contract::CommandSpec for PenduCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "pendu",
            category: "game",
            params: "aucun",
            description: "Jouer au jeu du pendu.",
            examples: &["+pendu"],
            default_aliases: &["hangman"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
