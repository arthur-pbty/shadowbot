use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_voice_channel_status_update(
    ctx: &Context,
    old: Option<String>,
    status: Option<String>,
    id: ChannelId,
    guild_id: GuildId,
) {
    logs_service::on_voice_channel_status_update(ctx, old, status, id, guild_id).await;
}
