use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_thread_member_update(ctx: &Context, thread_member: &ThreadMember) {
    logs_service::on_thread_member_update(ctx, thread_member).await;
}
