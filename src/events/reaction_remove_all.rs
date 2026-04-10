use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_reaction_remove_all(
    ctx: &Context,
    channel_id: ChannelId,
    removed_from_message_id: MessageId,
) {
    logs_service::on_reaction_remove_all(ctx, channel_id, removed_from_message_id).await;
}
