use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_role_delete(
    ctx: &Context,
    guild_id: GuildId,
    removed_role_id: RoleId,
    removed_role_data_if_available: Option<Role>,
) {
    logs_service::on_role_delete(
        ctx,
        guild_id,
        removed_role_id,
        removed_role_data_if_available.as_ref(),
    )
    .await;
}
