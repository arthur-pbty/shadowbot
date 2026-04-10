use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use chrono::Utc;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::moderation_sanction_helpers::{channel_mute_users, handle_timeout};
use crate::db::DbPoolKey;

static MODERATION_TICK: OnceLock<Mutex<Instant>> = OnceLock::new();

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
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
