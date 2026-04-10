use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_invite_delete(ctx: &Context, data: &InviteDeleteEvent) {
    logs_service::on_invite_delete(ctx, data).await;
}
