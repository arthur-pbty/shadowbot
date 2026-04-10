use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::{admin_service, botconfig_service, help};

pub async fn handle_ready(ctx: &Context, ready: &Ready) {
    botconfig_service::restore_presence_from_db(ctx).await;
    help::register_slash_help(ctx).await;

    for guild_id in ctx.cache.guilds() {
        admin_service::enforce_blacklist_on_guild(ctx, guild_id).await;
    }
    println!("Connecté en tant que {}", ready.user.name);
}
