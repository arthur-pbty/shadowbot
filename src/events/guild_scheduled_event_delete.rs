use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_guild_scheduled_event_delete(ctx: &Context, event: &ScheduledEvent) {
    logs_service::on_guild_scheduled_event_delete(ctx, event).await;
}
