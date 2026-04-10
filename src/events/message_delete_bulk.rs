use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_message_delete_bulk(
    ctx: &Context,
    channel_id: ChannelId,
    multiple_deleted_messages_ids: &[MessageId],
    guild_id: Option<GuildId>,
) {
    logs_service::on_message_delete_bulk(ctx, channel_id, multiple_deleted_messages_ids, guild_id)
        .await;
}
