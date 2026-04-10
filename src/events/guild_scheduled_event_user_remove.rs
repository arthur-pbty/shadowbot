use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_guild_scheduled_event_user_remove(
    ctx: &Context,
    unsubscribed: &GuildScheduledEventUserRemoveEvent,
) {
    logs_service::on_guild_scheduled_event_user_remove(ctx, unsubscribed).await;
}
