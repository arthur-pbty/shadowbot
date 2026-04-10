use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::db::{DbPoolKey, is_blacklisted, list_blacklisted_ids};

pub async fn enforce_blacklist_on_message(ctx: &Context, msg: &Message) -> bool {
    if msg.author.bot {
        return false;
    }

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    let Some(pool) = pool else {
        return false;
    };

    let blacklisted = is_blacklisted(&pool, bot_id, msg.author.id)
        .await
        .unwrap_or(false);
    if !blacklisted {
        return false;
    }

    if let Some(guild_id) = msg.guild_id {
        let _ = guild_id
            .ban_with_reason(&ctx.http, msg.author.id, 0, "Blacklist globale du bot")
            .await;
    }

    true
}

pub async fn enforce_blacklist_on_guild(ctx: &Context, guild_id: GuildId) {
    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    let Some(pool) = pool else {
        return;
    };

    let users = list_blacklisted_ids(&pool, bot_id)
        .await
        .unwrap_or_default();
    for uid in users {
        let _ = guild_id
            .ban_with_reason(&ctx.http, uid, 0, "Blacklist globale du bot")
            .await;
    }
}
