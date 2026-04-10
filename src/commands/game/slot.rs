use rand::seq::SliceRandom;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_slot(ctx: &Context, msg: &Message, _args: &[&str]) {
    let symbols = ["7", "BAR", "STAR", "BELL", "CHERRY"];
    let (a, b, c) = {
        let mut rng = rand::thread_rng();
        (
            symbols.choose(&mut rng).copied().unwrap_or("7"),
            symbols.choose(&mut rng).copied().unwrap_or("7"),
            symbols.choose(&mut rng).copied().unwrap_or("7"),
        )
    };

    let result = if a == b && b == c {
        "Jackpot"
    } else if a == b || b == c || a == c {
        "Presque"
    } else {
        "Perdu"
    };

    let embed = CreateEmbed::new()
        .title("Slot")
        .description(format!(
            "[ {} | {} | {} ]\nResultat: **{}**",
            a, b, c, result
        ))
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct SlotCommand;
pub static COMMAND_DESCRIPTOR: SlotCommand = SlotCommand;

impl crate::commands::command_contract::CommandSpec for SlotCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "slot",
            category: "game",
            params: "aucun",
            description: "Jouer au jeu Slot.",
            examples: &["+slot"],
            default_aliases: &["machine"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
