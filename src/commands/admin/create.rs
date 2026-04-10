use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

fn emoji_url_from_source(msg: &Message, source: &str) -> String {
    if source.starts_with("http://") || source.starts_with("https://") {
        return source.to_string();
    }

    if source.starts_with("<:") || source.starts_with("<a:") {
        let cleaned = source.trim_matches(|c| c == '<' || c == '>');
        let parts = cleaned.split(':').collect::<Vec<_>>();
        if parts.len() == 3 {
            let animated = parts[0] == "a";
            return format!(
                "https://cdn.discordapp.com/emojis/{}.{}",
                parts[2],
                if animated { "gif" } else { "png" }
            );
        }
    }

    if let Some(att) = msg.attachments.first() {
        return att.url.clone();
    }

    String::new()
}

pub async fn handle_create(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    if args.len() < 2 {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Create Emoji")
                .description("Usage: +create <emoji/url> <nom>")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let image_url = emoji_url_from_source(msg, args[0]);
    if image_url.is_empty() {
        return;
    }

    let response = match reqwest::get(&image_url).await {
        Ok(r) => r,
        Err(_) => return,
    };
    let bytes = match response.bytes().await {
        Ok(b) => b,
        Err(_) => return,
    };

    let data_uri = format!("data:image/png;base64,{}", {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(bytes)
    });

    let result = guild_id.create_emoji(&ctx.http, args[1], &data_uri).await;

    let embed = if let Ok(emoji) = result {
        CreateEmbed::new()
            .title("Emoji")
            .description(format!("Emoji cree: {}", emoji))
            .color(theme_color(ctx).await)
    } else {
        CreateEmbed::new()
            .title("Emoji")
            .description("Impossible de creer l'emoji.")
            .color(0xED4245)
    };

    send_embed(ctx, msg, embed).await;
}

pub struct CreateCommand;
pub static COMMAND_DESCRIPTOR: CreateCommand = CreateCommand;

impl crate::commands::command_contract::CommandSpec for CreateCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "create",
            category: "admin",
            params: "[emoji/url] [nom]",
            summary: "Cree un emoji custom",
            description: "Cree un emoji custom a partir d'une image, d'un lien ou d'un emoji nitro.",
            examples: &[
                "+create <:blob:123456789012345678> blobcopy",
                "+create https://... logo",
            ],
            default_aliases: &["mkemoji", "ce"],
            default_permission: 8,
        }
    }
}
