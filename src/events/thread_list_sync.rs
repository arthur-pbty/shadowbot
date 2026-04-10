use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_thread_list_sync(ctx: &Context, thread_list_sync: &ThreadListSyncEvent) {
    logs_service::on_thread_list_sync(ctx, thread_list_sync).await;
}
