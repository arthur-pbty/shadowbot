use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_member_removal(ctx: &Context, guild_id: GuildId, user: &User) {
    logs_service::on_member_leave(ctx, guild_id, user).await;
}
