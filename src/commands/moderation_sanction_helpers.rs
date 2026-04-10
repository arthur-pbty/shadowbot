use std::time::Duration;

use chrono::Utc;
use serenity::builder::EditMember;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::parse_user_id;
use crate::db::DbPoolKey;

pub fn duration_from_input(input: &str) -> Option<Duration> {
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

pub async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

pub async fn add_sanction(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
    moderator_id: UserId,
    kind: &str,
    reason: &str,
    channel_id: Option<ChannelId>,
    expires_at: Option<chrono::DateTime<Utc>>,
) {
    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id;
    let _ = sqlx::query(
        r#"
        INSERT INTO bot_sanctions
            (bot_id, guild_id, user_id, moderator_id, kind, reason, channel_id, expires_at, active)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, TRUE);
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(user_id.get() as i64)
    .bind(moderator_id.get() as i64)
    .bind(kind)
    .bind(reason)
    .bind(channel_id.map(|c| c.get() as i64))
    .bind(expires_at)
    .execute(&pool)
    .await;
}

pub async fn parse_targets(raw: &str) -> Vec<UserId> {
    let mut out = Vec::new();
    for token in raw.split(',') {
        if let Some(uid) = parse_user_id(token.trim()) {
            out.push(uid);
        }
    }
    out
}

pub async fn handle_timeout(
    ctx: &Context,
    guild_id: GuildId,
    users: &[UserId],
    expires: Option<chrono::DateTime<Utc>>,
) -> usize {
    let mut done = 0usize;
    for user_id in users {
        if let Ok(mut member) = guild_id.member(&ctx.http, *user_id).await {
            let mut builder = EditMember::new();
            if let Some(ts) = expires {
                if let Ok(discord_ts) = Timestamp::from_unix_timestamp(ts.timestamp()) {
                    builder = builder.disable_communication_until_datetime(discord_ts);
                }
            } else {
                builder = builder.enable_communication();
            }

            if member.edit(&ctx.http, builder).await.is_ok() {
                done += 1;
            }
        }
    }
    done
}

pub async fn channel_mute_users(
    ctx: &Context,
    channel_id: ChannelId,
    users: &[UserId],
    mute: bool,
) -> usize {
    let mut done = 0usize;
    for user_id in users {
        let result = if mute {
            channel_id
                .create_permission(
                    &ctx.http,
                    PermissionOverwrite {
                        allow: Permissions::empty(),
                        deny: Permissions::SEND_MESSAGES
                            | Permissions::ADD_REACTIONS
                            | Permissions::SPEAK,
                        kind: PermissionOverwriteType::Member(*user_id),
                    },
                )
                .await
        } else {
            channel_id
                .delete_permission(&ctx.http, PermissionOverwriteType::Member(*user_id))
                .await
        };

        if result.is_ok() {
            done += 1;
        }
    }
    done
}
