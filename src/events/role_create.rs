use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_role_create(ctx: &Context, new: &Role) {
    logs_service::on_role_create(ctx, new.guild_id, new).await;
}
