use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed, CreateMessage, EditMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Duration;

use crate::commands::common::theme_color;

fn duration_from_input(input: &str) -> Option<Duration> {
    let raw = input.trim().to_lowercase();
    if raw.is_empty() {
        return None;
    }

    let mut number = String::new();
    let mut suffix = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_digit() {
            if !suffix.is_empty() {
                return None;
            }
            number.push(ch);
        } else if !ch.is_whitespace() {
            suffix.push(ch);
        }
    }

    let value = number.parse::<u64>().ok()?;
    let secs = match suffix.as_str() {
        "s" | "sec" | "secs" | "seconde" | "secondes" => value,
        "m" | "min" | "mins" | "minute" | "minutes" => value * 60,
        "h" | "heure" | "heures" => value * 3600,
        "j" | "d" | "jour" | "jours" => value * 86400,
        _ => return None,
    };

    Some(Duration::from_secs(secs.max(1)))
}

pub async fn handle_loading(ctx: &Context, msg: &Message, args: &[&str]) {
    if args.len() < 2 {
        let embed = CreateEmbed::new()
            .title("Loading")
            .description("Ouvre un modal pour saisir la durée et le message.")
            .color(theme_color(ctx).await);

        let components = vec![CreateActionRow::Buttons(vec![
            CreateButton::new(format!("adv:loading:modal:{}", msg.author.id.get()))
                .label("Configurer")
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

    let Some(duration) = duration_from_input(args[0]) else {
        return;
    };

    let total_secs = duration.as_secs().clamp(1, 120);
    let text = args[1..].join(" ");

    let mut sent = match msg
        .channel_id
        .send_message(&ctx.http, CreateMessage::new().content("[----------] 0%"))
        .await
    {
        Ok(m) => m,
        Err(_) => return,
    };

    for i in 0..=10_u64 {
        let done = "#".repeat(i as usize);
        let todo = "-".repeat((10 - i) as usize);
        let percent = i * 10;

        let _ = sent
            .edit(
                &ctx.http,
                EditMessage::new().content(format!("{} [{}{}] {}%", text, done, todo, percent)),
            )
            .await;

        if i < 10 {
            tokio::time::sleep(Duration::from_secs((total_secs / 10).max(1))).await;
        }
    }
}

pub struct LoadingCommand;
pub static COMMAND_DESCRIPTOR: LoadingCommand = LoadingCommand;

impl crate::commands::command_contract::CommandSpec for LoadingCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "loading",
            category: "fun",
            params: "<duree> <message>",
            description: "Anime une barre de progression avec la duree et le texte fournis.",
            examples: &["+loading 10s Traitement en cours"],
            default_aliases: &["loadbar", "bar"],
            allow_in_dm: false,
            default_permission: 0,
        }
    }
}
