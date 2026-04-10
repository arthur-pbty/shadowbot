use rand::Rng;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_epicgamer(ctx: &Context, msg: &Message, _args: &[&str]) {
    let percent = {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..=100)
    };

    let rank = if percent >= 90 {
        "Legendary"
    } else if percent >= 70 {
        "Epic"
    } else if percent >= 40 {
        "Casual"
    } else {
        "Noob"
    };

    let embed = CreateEmbed::new()
        .title("Epic Gamer")
        .description(format!(
            "Ton pourcentage de gamer epique est **{}%**.\nRang: **{}**.",
            percent, rank
        ))
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct EpicgamerCommand;
pub static COMMAND_DESCRIPTOR: EpicgamerCommand = EpicgamerCommand;

impl crate::commands::command_contract::CommandSpec for EpicgamerCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "epicgamer",
            category: "game",
            params: "aucun",
            description: "Evaluez votre pourcentage de gamer epique.",
            examples: &["+epicgamer"],
            default_aliases: &["gamer"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
