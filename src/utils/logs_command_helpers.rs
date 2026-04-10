use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::parse_channel_id;
use crate::db::DbPoolKey;

pub async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

pub fn parse_target_channel(msg: &Message, args: &[&str], idx: usize) -> Option<ChannelId> {
    args.get(idx)
        .and_then(|raw| parse_channel_id(raw))
        .or(Some(msg.channel_id))
}

pub async fn set_log_channel(
    ctx: &Context,
    guild_id: GuildId,
    log_type: &str,
    channel_id: Option<ChannelId>,
    enabled: bool,
) {
    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let _ = sqlx::query(
        r#"
        INSERT INTO bot_log_channels (bot_id, guild_id, log_type, channel_id, enabled)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (bot_id, guild_id, log_type)
        DO UPDATE SET channel_id = EXCLUDED.channel_id, enabled = EXCLUDED.enabled, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(log_type)
    .bind(channel_id.map(|c| c.get() as i64))
    .bind(enabled)
    .execute(&pool)
    .await;
}
