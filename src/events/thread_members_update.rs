use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_thread_members_update(ctx: &Context, event: &ThreadMembersUpdateEvent) {
    logs_service::on_thread_members_update(ctx, event).await;
}
