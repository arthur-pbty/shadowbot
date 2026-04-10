use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_service;

pub async fn handle_guild_create(ctx: &Context, guild: &Guild) {
    admin_service::enforce_blacklist_on_guild(ctx, guild.id).await;
}
