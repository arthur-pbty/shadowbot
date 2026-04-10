use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_role_update(ctx: &Context, old_data_if_available: Option<Role>, new: &Role) {
    logs_service::on_role_update(ctx, new.guild_id, old_data_if_available.as_ref(), new).await;
}
