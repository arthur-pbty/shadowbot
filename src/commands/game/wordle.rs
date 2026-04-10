use rand::seq::SliceRandom;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_wordle(ctx: &Context, msg: &Message, _args: &[&str]) {
    let words = ["pomme", "ombre", "salon", "banjo", "pixel", "vocal"];

    let secret = {
        let mut rng = rand::thread_rng();
        words.choose(&mut rng).copied().unwrap_or("pomme")
    };
    let hint = format!("{}{}", &secret[0..1], "_ _ _ _");

    let embed = CreateEmbed::new()
        .title("Wordle")
        .description(format!(
            "Mot secret de 5 lettres initialise.\nIndice: **{}**\n\nPropose un mot avec `+wordle <mot>` (version libre).",
            hint
        ))
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct WordleCommand;
pub static COMMAND_DESCRIPTOR: WordleCommand = WordleCommand;

impl crate::commands::command_contract::CommandSpec for WordleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "wordle",
            category: "game",
            params: "aucun",
            description: "Jouer a Wordle.",
            examples: &["+wordle"],
            default_aliases: &["wd"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
