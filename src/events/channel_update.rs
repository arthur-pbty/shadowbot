use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_channel_update(
    ctx: &Context,
    old_data_if_available: Option<GuildChannel>,
    new: &GuildChannel,
) {
    logs_service::on_channel_update(ctx, old_data_if_available, new).await;
}
