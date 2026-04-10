use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_webhook_update(
    ctx: &Context,
    guild_id: GuildId,
    belongs_to_channel_id: ChannelId,
) {
    logs_service::on_webhook_update(ctx, guild_id, belongs_to_channel_id).await;
}
