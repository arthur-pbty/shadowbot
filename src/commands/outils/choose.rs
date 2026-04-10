use rand::seq::SliceRandom;
use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed, CreateMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_choose(ctx: &Context, msg: &Message, args: &[&str]) {
    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Choose")
            .description("Ouvre un modal pour saisir les options (séparées par `|`).")
            .color(theme_color(ctx).await);
        let components = vec![CreateActionRow::Buttons(vec![
            CreateButton::new(format!("adv:choose:modal:{}", msg.author.id.get()))
                .label("Saisir les options")
                .style(serenity::all::ButtonStyle::Primary),
        ])];

        let _ = msg
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().embed(embed).components(components),
            )
            .await;
        return;
    }

    let merged = args.join(" ");
    let mut options = merged
        .split('|')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    if options.len() < 2 {
        options = args.iter().map(|s| (*s).to_string()).collect();
    }

    if options.len() < 2 {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Choose")
                .description("Donne au moins 2 options.")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let pick = options
        .choose(&mut rand::thread_rng())
        .cloned()
        .unwrap_or_else(|| options[0].clone());

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Tirage")
            .description(format!("Résultat: **{}**", pick))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct ChooseCommand;
pub static COMMAND_DESCRIPTOR: ChooseCommand = ChooseCommand;

impl crate::commands::command_contract::CommandSpec for ChooseCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "choose",
            category: "outils",
            params: "<option1 | option2 | ...>",
            summary: "Tire une option au hasard",
            description: "Lance un tirage au sort instantane parmi les options donnees.",
            examples: &["+choose rouge | bleu | vert"],
            default_aliases: &["pick", "random"],
            default_permission: 8,
        }
    }
}
