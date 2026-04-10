use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_guild_ban_addition(ctx: &Context, guild_id: GuildId, banned_user: &User) {
    logs_service::on_guild_ban_addition(ctx, guild_id, banned_user).await;
}
