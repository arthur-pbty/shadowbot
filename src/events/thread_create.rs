use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_thread_create(ctx: &Context, thread: &GuildChannel) {
    logs_service::on_thread_create(ctx, thread).await;
}
