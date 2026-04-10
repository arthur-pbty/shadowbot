use rand::seq::SliceRandom;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_snake(ctx: &Context, msg: &Message, _args: &[&str]) {
    let directions = ["haut", "bas", "gauche", "droite"];
    let foods = ["pomme", "banane", "fraise", "citron"];

    let (direction, food) = {
        let mut rng = rand::thread_rng();
        (
            directions.choose(&mut rng).copied().unwrap_or("haut"),
            foods.choose(&mut rng).copied().unwrap_or("pomme"),
        )
    };

    let embed = CreateEmbed::new()
        .title("Snake")
        .description(format!(
            "Partie lancee. Direction conseillee: **{}**.\nObjectif courant: attraper une **{}**.",
            direction, food
        ))
        .field(
            "Commande rapide",
            "Relance `+snake` pour un nouveau round.",
            false,
        )
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct SnakeCommand;
pub static COMMAND_DESCRIPTOR: SnakeCommand = SnakeCommand;

impl crate::commands::command_contract::CommandSpec for SnakeCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "snake",
            category: "game",
            params: "aucun",
            description: "Lancer une partie de snake.",
            examples: &["+snake"],
            default_aliases: &["snk"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
