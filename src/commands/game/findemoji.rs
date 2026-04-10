use rand::seq::SliceRandom;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_findemoji(ctx: &Context, msg: &Message, _args: &[&str]) {
    let packs = [
        (":cat:", [":cat:", ":dog:", ":fox:", ":bear:"]),
        (":star:", [":moon:", ":sunny:", ":star:", ":zap:"]),
        (":pizza:", [":hamburger:", ":pizza:", ":fries:", ":hotdog:"]),
    ];

    let (target, options) = {
        let mut rng = rand::thread_rng();
        packs
            .choose(&mut rng)
            .copied()
            .unwrap_or((":cat:", [":cat:", ":dog:", ":fox:", ":bear:"]))
    };

    let embed = CreateEmbed::new()
        .title("FindEmoji")
        .description(format!(
            "Trouve l emoji cible: **{}**\nOptions: {}",
            target,
            options.join("  ")
        ))
        .field(
            "Regle",
            "Le premier qui identifie le bon emoji gagne le round.",
            false,
        )
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct FindemojiCommand;
pub static COMMAND_DESCRIPTOR: FindemojiCommand = FindemojiCommand;

impl crate::commands::command_contract::CommandSpec for FindemojiCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "findemoji",
            category: "game",
            params: "aucun",
            description: "Jouer au jeu Trouver l Emoji.",
            examples: &["+findemoji"],
            default_aliases: &["emojihunt"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
