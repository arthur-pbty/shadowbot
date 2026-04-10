use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_guild_ban_removal(ctx: &Context, guild_id: GuildId, unbanned_user: &User) {
    logs_service::on_guild_ban_removal(ctx, guild_id, unbanned_user).await;
}
