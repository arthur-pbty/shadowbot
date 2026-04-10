use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_invite_create(ctx: &Context, data: &InviteCreateEvent) {
    logs_service::on_invite_create(ctx, data).await;
}
