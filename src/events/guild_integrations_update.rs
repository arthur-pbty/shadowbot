use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_guild_integrations_update(ctx: &Context, guild_id: GuildId) {
    logs_service::on_guild_integrations_update(ctx, guild_id).await;
}
