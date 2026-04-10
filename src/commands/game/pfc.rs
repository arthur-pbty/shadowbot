use rand::seq::SliceRandom;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_pfc(ctx: &Context, msg: &Message, args: &[&str]) {
    let choices = ["pierre", "papier", "ciseaux"];

    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Pierre Papier Ciseaux")
            .description("Utilise `+pfc <pierre|papier|ciseaux>` pour jouer.")
            .color(theme_color(ctx).await);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let player = args[0].to_lowercase();
    if !choices.iter().any(|choice| player == *choice) {
        let embed = CreateEmbed::new()
            .title("PFC")
            .description("Choix invalide. Valeurs attendues: pierre, papier ou ciseaux.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let bot = {
        let mut rng = rand::thread_rng();
        choices.choose(&mut rng).copied().unwrap_or("pierre")
    };

    let result = if player == bot {
        "Egalite."
    } else if (player == "pierre" && bot == "ciseaux")
        || (player == "papier" && bot == "pierre")
        || (player == "ciseaux" && bot == "papier")
    {
        "Tu gagnes."
    } else {
        "Tu perds."
    };

    let embed = CreateEmbed::new()
        .title("PFC")
        .description(format!(
            "Ton choix: **{}**\nChoix du bot: **{}**\n\n{}",
            player, bot, result
        ))
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct PfcCommand;
pub static COMMAND_DESCRIPTOR: PfcCommand = PfcCommand;

impl crate::commands::command_contract::CommandSpec for PfcCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "pfc",
            category: "game",
            params: "<pierre|papier|ciseaux>",
            description: "Jouer a pierre-papier-ciseaux.",
            examples: &["+pfc pierre"],
            default_aliases: &["rps"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
