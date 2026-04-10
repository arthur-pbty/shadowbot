use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_channel_create(ctx: &Context, channel: &GuildChannel) {
    logs_service::on_channel_create(ctx, channel).await;
}
