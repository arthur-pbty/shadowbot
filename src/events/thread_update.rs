use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_thread_update(ctx: &Context, old: Option<GuildChannel>, new: &GuildChannel) {
    logs_service::on_thread_update(ctx, old, new).await;
}
