use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

fn parse_custom_emoji(input: &str) -> Option<(bool, String)> {
    if !(input.starts_with("<:") || input.starts_with("<a:")) || !input.ends_with('>') {
        return None;
    }

    let animated = input.starts_with("<a:");
    let inner = input.trim_start_matches('<').trim_end_matches('>');
    let parts: Vec<&str> = inner.split(':').collect();
    if parts.len() != 3 {
        return None;
    }

    let id = parts[2].to_string();
    Some((animated, id))
}

fn unicode_emoji_url(input: &str) -> Option<String> {
    if input.is_empty() {
        return None;
    }

    let codepoints = input
        .chars()
        .filter(|c| *c as u32 != 0xFE0F)
        .map(|c| format!("{:x}", c as u32))
        .collect::<Vec<_>>()
        .join("-");

    if codepoints.is_empty() {
        return None;
    }

    Some(format!(
        "https://twemoji.maxcdn.com/v/latest/72x72/{}.png",
        codepoints
    ))
}

pub async fn handle_emoji(ctx: &Context, msg: &Message, args: &[&str]) {
    let color = theme_color(ctx).await;
    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+emoji <émoji>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let input = args.join(" ");

    let url = if let Some((animated, id)) = parse_custom_emoji(&input) {
        let ext = if animated { "gif" } else { "png" };
        format!("https://cdn.discordapp.com/emojis/{}.{}?size=1024", id, ext)
    } else if let Some(url) = unicode_emoji_url(input.trim()) {
        url
    } else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Émoji invalide. Utilise un émoji Unicode ou un émoji custom Discord.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let embed = CreateEmbed::new()
        .title("Image de l'émoji")
        .image(url)
        .color(color);

    send_embed(ctx, msg, embed).await;
}

pub struct EmojiCommand;
pub static COMMAND_DESCRIPTOR: EmojiCommand = EmojiCommand;

impl crate::commands::command_contract::CommandSpec for EmojiCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "emoji",
            command: "emoji",
            category: "general",
            params: "<emoji>",
            summary: "Affiche les infos dun emoji",
            description: "Affiche les details dun emoji fourni.",
            examples: &["+emoji", "+ei", "+help emoji"],
            alias_source_key: "emoji",
            default_aliases: &["emj"],
        }
    }
}
