use rand::seq::SliceRandom;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_reroll(ctx: &Context, msg: &Message, _args: &[&str]) {
    let Some(referenced) = msg.referenced_message.as_ref() else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Reroll")
                .description("Réponds à un message giveaway pour reroll.")
                .color(0xED4245),
        )
        .await;
        return;
    };

    let mut candidates = referenced.mentions.iter().map(|u| u.id).collect::<Vec<_>>();
    candidates.sort_by_key(|u| u.get());
    candidates.dedup();

    if candidates.is_empty() {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Reroll")
                .description("Aucun participant détecté.")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let winner = candidates
        .choose(&mut rand::thread_rng())
        .copied()
        .unwrap_or(candidates[0]);

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Reroll")
            .description(format!("Nouveau gagnant: <@{}>", winner.get()))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct RerollCommand;
pub static COMMAND_DESCRIPTOR: RerollCommand = RerollCommand;

impl crate::commands::command_contract::CommandSpec for RerollCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "reroll",
            category: "admin",
            params: "aucun (en reponse a un message)",
            summary: "Relance un tirage giveaway",
            description: "Choisit un nouveau gagnant depuis le message cible.",
            examples: &["+reroll"],
            default_aliases: &["rro", "greroll"],
            default_permission: 8,
        }
    }
}
