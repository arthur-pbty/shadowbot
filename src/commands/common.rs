use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::db::{DbPoolKey, get_bot_theme};

pub fn parse_limit(args: &[&str], default: usize, max: usize) -> usize {
    args.iter()
        .find_map(|arg| arg.parse::<usize>().ok())
        .map(|value| value.clamp(1, max))
        .unwrap_or(default)
}

pub fn has_flag(args: &[&str], names: &[&str]) -> bool {
    args.iter()
        .any(|arg| names.iter().any(|name| arg.eq_ignore_ascii_case(name)))
}

pub fn truncate_text(input: &str, max_len: usize) -> String {
    if input.chars().count() <= max_len {
        return input.to_string();
    }

    let mut out = input
        .chars()
        .take(max_len.saturating_sub(1))
        .collect::<String>();
    out.push('…');
    out
}

pub fn add_list_fields(mut embed: CreateEmbed, lines: &[String], base_name: &str) -> CreateEmbed {
    if lines.is_empty() {
        return embed.field(base_name, "Aucun résultat.", false);
    }

    let max_fields = 3;
    let chunk_size = 12;

    for (index, chunk) in lines.chunks(chunk_size).take(max_fields).enumerate() {
        let field_name = if index == 0 {
            base_name.to_string()
        } else {
            format!("{} (suite {})", base_name, index + 1)
        };

        let value = truncate_text(&chunk.join("\n"), 1024);
        embed = embed.field(field_name, value, false);
    }

    let shown = (chunk_size * max_fields).min(lines.len());
    if lines.len() > shown {
        embed = embed.field(
            "Affichage",
            format!("{} éléments affichés sur {}.", shown, lines.len()),
            false,
        );
    }

    embed
}

pub fn mention_user(user_id: UserId) -> String {
    format!("<@{}>", user_id.get())
}

pub fn discord_ts(timestamp: Timestamp, style: &str) -> String {
    format!("<t:{}:{}>", timestamp.unix_timestamp(), style)
}

pub async fn send_embed(ctx: &Context, msg: &Message, embed: CreateEmbed) {
    let color = theme_color(ctx).await;
    let embed = embed.color(color);

    let _ = msg
        .channel_id
        .send_message(&ctx.http, CreateMessage::new().embed(embed))
        .await;
}

pub async fn theme_color(ctx: &Context) -> u32 {
    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    if let Some(pool) = pool {
        if let Ok(Some(color)) = get_bot_theme(&pool, bot_id).await {
            return color;
        }
    }

    0xFF0000
}

pub fn parse_role(guild: &PartialGuild, input: &str) -> Option<Role> {
    // Essayer de parser comme mention <@&id>
    if let Ok(id) = input
        .trim_start_matches("<@&")
        .trim_end_matches('>')
        .parse::<u64>()
    {
        if let Some(role) = guild.roles.get(&RoleId::new(id)) {
            return Some(role.clone());
        }
    }

    // Essayer de parser comme ID brut
    if let Ok(id) = input.parse::<u64>() {
        if let Some(role) = guild.roles.get(&RoleId::new(id)) {
            return Some(role.clone());
        }
    }

    // Chercher par nom (case-insensitive)
    let search = input.to_lowercase();
    guild
        .roles
        .values()
        .find(|r| r.name.to_lowercase().contains(&search))
        .cloned()
}

pub fn parse_channel_id(input: &str) -> Option<ChannelId> {
    // Essayer de parser comme mention <#id>
    if let Ok(id) = input
        .trim_start_matches("<#")
        .trim_end_matches('>')
        .parse::<u64>()
    {
        return Some(ChannelId::new(id));
    }

    // Essayer de parser comme ID brut
    if let Ok(id) = input.parse::<u64>() {
        return Some(ChannelId::new(id));
    }

    None
}
