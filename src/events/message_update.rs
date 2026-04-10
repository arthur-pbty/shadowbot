use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;
use crate::db::{DbPoolKey, get_observed_message, upsert_message_observed_partial};

pub async fn handle_message_update(
    ctx: &Context,
    old_if_available: Option<Message>,
    new: Option<Message>,
    event: &MessageUpdateEvent,
) {
    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    let db_fallback = if let Some(pool) = pool.as_ref() {
        get_observed_message(pool, bot_id, event.id)
            .await
            .ok()
            .flatten()
    } else {
        None
    };

    let db_author_id = db_fallback
        .as_ref()
        .and_then(|(author_id, _)| author_id.clone());
    let db_content = db_fallback.as_ref().map(|(_, content)| content.clone());

    let before = old_if_available
        .as_ref()
        .map(|m| m.content.clone())
        .or_else(|| db_content.clone());
    let after = new
        .as_ref()
        .map(|m| m.content.clone())
        .or_else(|| event.content.clone());
    let author_id = old_if_available
        .as_ref()
        .map(|m| m.author.id)
        .or_else(|| new.as_ref().map(|m| m.author.id))
        .or_else(|| event.author.as_ref().map(|u| u.id))
        .or(db_author_id);

    if let (Some(pool), Some(after_content)) = (pool.as_ref(), after.as_ref()) {
        let _ = upsert_message_observed_partial(
            pool,
            bot_id,
            event.id,
            event.guild_id,
            event.channel_id,
            author_id,
            after_content,
        )
        .await;
    }

    logs_service::on_message_edited(
        ctx,
        event.guild_id,
        event.channel_id,
        author_id,
        before,
        after,
    )
    .await;
}
