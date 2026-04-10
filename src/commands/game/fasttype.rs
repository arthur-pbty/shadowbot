use rand::seq::SliceRandom;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_fasttype(ctx: &Context, msg: &Message, _args: &[&str]) {
    let challenges = [
        "shadow bot est rapide",
        "rust rend le bot solide",
        "je tape plus vite que mon ombre",
        "discord et serenite",
    ];

    let sentence = {
        let mut rng = rand::thread_rng();
        challenges
            .choose(&mut rng)
            .copied()
            .unwrap_or("shadow bot est rapide")
    };

    let embed = CreateEmbed::new()
        .title("FastType")
        .description(format!(
            "Premier a retaper cette phrase gagne:\n\n`{}`",
            sentence
        ))
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct FasttypeCommand;
pub static COMMAND_DESCRIPTOR: FasttypeCommand = FasttypeCommand;

impl crate::commands::command_contract::CommandSpec for FasttypeCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "fasttype",
            category: "game",
            params: "aucun",
            description: "Jouer a un jeu de vitesse de frappe.",
            examples: &["+fasttype"],
            default_aliases: &["type"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
