use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_message_update(
    ctx: &Context,
    old_if_available: Option<Message>,
    new: Option<Message>,
    event: &MessageUpdateEvent,
) {
    let before = old_if_available.as_ref().map(|m| m.content.clone());
    let after = new
        .as_ref()
        .map(|m| m.content.clone())
        .or_else(|| event.content.clone());
    let author_id = old_if_available
        .as_ref()
        .map(|m| m.author.id)
        .or_else(|| new.as_ref().map(|m| m.author.id));

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
