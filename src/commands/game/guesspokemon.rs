use rand::seq::SliceRandom;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_guesspokemon(ctx: &Context, msg: &Message, _args: &[&str]) {
    let pokemons = [
        "pikachu",
        "bulbizarre",
        "salameche",
        "carapuce",
        "evoli",
        "dracaufeu",
    ];

    let name = {
        let mut rng = rand::thread_rng();
        pokemons.choose(&mut rng).copied().unwrap_or("pikachu")
    };
    let hint = format!(
        "{}{}",
        &name[0..1],
        "_".repeat(name.chars().count().saturating_sub(1))
    );

    let embed = CreateEmbed::new()
        .title("GuessPokemon")
        .description(format!(
            "Qui est ce pokemon ?\nIndice: **{}** ({} lettres)",
            hint,
            name.chars().count()
        ))
        .field("Reponse", "Le premier a donner le bon nom gagne.", false)
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct GuesspokemonCommand;
pub static COMMAND_DESCRIPTOR: GuesspokemonCommand = GuesspokemonCommand;

impl crate::commands::command_contract::CommandSpec for GuesspokemonCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "guesspokemon",
            category: "game",
            params: "aucun",
            description: "Jouer au jeu trouver le pokemon.",
            examples: &["+guesspokemon"],
            default_aliases: &["pokemon"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
