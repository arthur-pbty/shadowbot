use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_thread_delete(
    ctx: &Context,
    thread: &PartialGuildChannel,
    full_thread_data: Option<&GuildChannel>,
) {
    logs_service::on_thread_delete(ctx, thread, full_thread_data).await;
}
