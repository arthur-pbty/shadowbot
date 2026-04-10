use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;
use sqlx::PgPool;

use crate::commands::common::send_embed;
use crate::db::DbPoolKey;
use crate::permissions::is_owner_user;

pub fn parse_user_or_role(input: &str) -> Option<(&'static str, u64)> {
    let trimmed = input.trim();
    if trimmed.starts_with("<@&") && trimmed.ends_with('>') {
        return trimmed
            .trim_start_matches("<@&")
            .trim_end_matches('>')
            .parse::<u64>()
            .ok()
            .map(|id| ("role", id));
    }

    if (trimmed.starts_with("<@") && trimmed.ends_with('>')) || trimmed.parse::<u64>().is_ok() {
        let cleaned = trimmed
            .trim_start_matches('<')
            .trim_end_matches('>')
            .trim_start_matches('@')
            .trim_start_matches('!');
        if let Ok(id) = cleaned.parse::<u64>() {
            return Some(("user", id));
        }
    }

    None
}

pub fn normalize_command_name(input: &str) -> String {
    input
        .trim_start_matches('+')
        .replace(' ', "_")
        .to_lowercase()
}

pub async fn ensure_owner(ctx: &Context, msg: &Message) -> bool {
    if is_owner_user(ctx, msg.author.id).await {
        true
    } else {
        let embed = CreateEmbed::new()
            .title("Acces refuse")
            .description("Commande reservee aux owners.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        false
    }
}

pub async fn get_pool(ctx: &Context) -> Option<PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}
