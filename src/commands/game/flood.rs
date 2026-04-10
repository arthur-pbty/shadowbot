use rand::Rng;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_flood(ctx: &Context, msg: &Message, _args: &[&str]) {
    let palette = ['R', 'V', 'B', 'J', 'M', 'C'];
    let mut rows = Vec::new();

    {
        let mut rng = rand::thread_rng();
        for _ in 0..6 {
            let mut line = String::new();
            for _ in 0..6 {
                let color = palette[rng.gen_range(0..palette.len())];
                line.push(color);
                line.push(' ');
            }
            rows.push(line.trim_end().to_string());
        }
    }

    let embed = CreateEmbed::new()
        .title("Flood")
        .description(format!(
            "Objectif: inonder la grille avec une seule couleur en un minimum de coups.\n\n```\n{}\n```",
            rows.join("\n")
        ))
        .field("Couleurs", "R=rouge V=vert B=bleu J=jaune M=magenta C=cyan", false)
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct FloodCommand;
pub static COMMAND_DESCRIPTOR: FloodCommand = FloodCommand;

impl crate::commands::command_contract::CommandSpec for FloodCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "flood",
            category: "game",
            params: "aucun",
            description: "Jouer au jeu Flood.",
            examples: &["+flood"],
            default_aliases: &["floodit"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
