use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::activity::{RotatingActivityKind, parse_status, start_rotation};
use crate::db::DbPoolKey;

pub async fn restore_presence_from_db(ctx: &Context) {
    let bot_id = ctx.cache.current_user().id;

    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    let Some(pool) = pool else {
        return;
    };

    let status = match crate::db::get_bot_status(&pool, bot_id).await {
        Ok(Some(saved)) => parse_status(&saved),
        _ => OnlineStatus::Online,
    };

    ctx.set_presence(None, status);

    let activity_row = crate::db::get_bot_activity(&pool, bot_id)
        .await
        .ok()
        .flatten();
    if let Some((kind_raw, messages_raw)) = activity_row {
        let Some(kind) = RotatingActivityKind::from_db(&kind_raw) else {
            return;
        };

        let messages: Vec<String> = messages_raw
            .split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        if !messages.is_empty() {
            start_rotation(ctx, kind, messages, status).await;
        }
    }
}
