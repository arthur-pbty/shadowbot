use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Duration;

use crate::commands::admin_common::parse_user_id;
use crate::commands::common::{parse_role, send_embed, theme_color};
use crate::db::DbPoolKey;

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

pub async fn handle_temprole(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    if args.len() < 3 {
        return;
    }

    let Some(user_id) = parse_user_id(args[0]) else {
        return;
    };

    let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await else {
        return;
    };

    let Some(role) = parse_role(&guild, args[1]) else {
        return;
    };

    let Some(duration) = duration_from_input(args[2]) else {
        return;
    };

    let expires_at = Utc::now() + chrono::Duration::seconds(duration.as_secs() as i64);

    if let Ok(member) = guild_id.member(&ctx.http, user_id).await {
        let _ = member.add_role(&ctx.http, role.id).await;
    }

    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };
    if let Some(pool) = pool {
        let bot_id = ctx.cache.current_user().id;
        let _ = sqlx::query(
            r#"
            INSERT INTO bot_temproles (bot_id, guild_id, user_id, role_id, expires_at, active, added_by)
            VALUES ($1, $2, $3, $4, $5, TRUE, $6)
            ON CONFLICT (bot_id, guild_id, user_id, role_id)
            DO UPDATE SET expires_at = EXCLUDED.expires_at, active = TRUE, added_by = EXCLUDED.added_by;
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(user_id.get() as i64)
        .bind(role.id.get() as i64)
        .bind(expires_at)
        .bind(msg.author.id.get() as i64)
        .execute(&pool)
        .await;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("TempRole")
            .description(format!(
                "Rôle <@&{}> ajouté à <@{}> jusqu'à <t:{}:F>.",
                role.id.get(),
                user_id.get(),
                expires_at.timestamp()
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct TempRoleCommand;
pub static COMMAND_DESCRIPTOR: TempRoleCommand = TempRoleCommand;

impl crate::commands::command_contract::CommandSpec for TempRoleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "temprole",
            category: "roles",
            params: "<membre> <role> <duree>",
            description: "Attribue un role a un membre pour une duree donnee puis le retire automatiquement.",
            examples: &["+temprole @User @VIP 2h"],
            default_aliases: &["trole", "tmprole"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
