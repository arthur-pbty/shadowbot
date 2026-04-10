use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use chrono::Utc;
use serenity::builder::EditMember;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::db::DbPoolKey;

static MODERATION_TICK: OnceLock<Mutex<Instant>> = OnceLock::new();

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

async fn handle_timeout(
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

async fn channel_mute_users(
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

pub async fn maybe_run_maintenance(ctx: &Context, guild_id: Option<GuildId>) {
    let Some(guild_id) = guild_id else {
        return;
    };

    let now = Instant::now();
    let lock = MODERATION_TICK.get_or_init(|| Mutex::new(Instant::now() - Duration::from_secs(60)));
    {
        let mut last = lock.lock().expect("moderation tick lock poisoned");
        if now.duration_since(*last) < Duration::from_secs(30) {
            return;
        }
        *last = now;
    }

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;
    let now_dt = Utc::now();

    let rows = sqlx::query_as::<_, (i64, i64, String, Option<i64>)>(
        r#"
        SELECT id, user_id, kind, channel_id
        FROM bot_sanctions
        WHERE bot_id = $1 AND guild_id = $2 AND active = TRUE AND expires_at IS NOT NULL AND expires_at <= $3;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(now_dt)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    for (id, uid, kind, channel_id) in &rows {
        let user_id = UserId::new(*uid as u64);
        if kind == "tempmute" {
            let _ = handle_timeout(ctx, guild_id, &[user_id], None).await;
        } else if kind == "tempcmute" {
            if let Some(cid) = channel_id {
                let _ =
                    channel_mute_users(ctx, ChannelId::new(*cid as u64), &[user_id], false).await;
            }
        } else if kind == "tempban" {
            let _ = guild_id.unban(&ctx.http, user_id).await;
        }

        let _ = sqlx::query("UPDATE bot_sanctions SET active = FALSE WHERE id = $1")
            .bind(*id)
            .execute(&pool)
            .await;
    }
}
