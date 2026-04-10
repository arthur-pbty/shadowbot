use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;
use crate::db::{
    DbPoolKey, get_observed_message, mark_message_deleted, mark_sent_mp_deleted_by_message,
};

pub async fn handle_message_delete(
    ctx: &Context,
    channel_id: ChannelId,
    deleted_message_id: MessageId,
    guild_id: Option<GuildId>,
) {
    let bot_id = ctx.cache.current_user().id;

    let (cache_author_id, cache_content) =
        if let Some(cached) = ctx.cache.message(channel_id, deleted_message_id) {
            (Some(cached.author.id), Some(cached.content.clone()))
        } else {
            (None, None)
        };

    let mut resolved_author_id = cache_author_id;
    let mut resolved_content = cache_content;

    if let Some(pool) = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    } {
        if let Ok(Some((db_author_id, db_content))) =
            get_observed_message(&pool, bot_id, deleted_message_id).await
        {
            if resolved_author_id.is_none() {
                resolved_author_id = db_author_id;
            }
            if resolved_content.is_none() {
                resolved_content = Some(db_content);
            }
        }

        let _ =
            mark_sent_mp_deleted_by_message(&pool, bot_id, channel_id, deleted_message_id).await;

        let _ = mark_message_deleted(
            &pool,
            bot_id,
            guild_id,
            channel_id,
            deleted_message_id,
            resolved_author_id,
            resolved_content.clone(),
        )
        .await;
    }

    logs_service::on_message_deleted(
        ctx,
        guild_id,
        channel_id,
        deleted_message_id,
        resolved_author_id,
        resolved_content,
    )
    .await;
}
