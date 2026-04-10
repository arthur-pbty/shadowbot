use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_channel_delete(ctx: &Context, channel: &GuildChannel) {
    logs_service::on_channel_delete(ctx, channel).await;
}
