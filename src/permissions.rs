use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::BTreeSet;
use std::env;

use crate::commands::common::send_embed;
use crate::db::{
    DbPoolKey, get_command_permission, get_guild_prefix, get_main_prefix, has_command_access,
    has_perm_level_access, is_bot_owner,
};

const EXTRA_COMMAND_KEYS: &[&str] = &[
    "ticket_settings",
    "ticket_add",
    "ticket_remove",
    "ticket_close",
    "suggestion_settings",
    "setperm",
    "delperm",
    "changereset",
    "serverlist",
    "endgiveaway",
    "mpsettings",
];

fn first_arg_is(args: &[&str], expected: &str) -> bool {
    args.first()
        .map(|value| value.eq_ignore_ascii_case(expected))
        .unwrap_or(false)
}

pub fn command_key(command: &str, args: &[&str]) -> String {
    let normalized = command.to_lowercase();

    match normalized.as_str() {
        "ticket" => "ticket_settings".to_string(),
        "add" => "ticket_add".to_string(),
        "del" => "ticket_remove".to_string(),
        "close" => "ticket_close".to_string(),
        "clear" => "clearmessages".to_string(),
        "suggestion" => {
            if first_arg_is(args, "settings") {
                "suggestion_settings".to_string()
            } else {
                "suggestion_create".to_string()
            }
        }
        "change" => {
            if first_arg_is(args, "reset") {
                "changereset".to_string()
            } else {
                "change".to_string()
            }
        }
        "server" => {
            if first_arg_is(args, "list") {
                "serverlist".to_string()
            } else {
                "server".to_string()
            }
        }
        "end" => {
            if first_arg_is(args, "giveaway") {
                "endgiveaway".to_string()
            } else {
                "end".to_string()
            }
        }
        "mp" => {
            if first_arg_is(args, "settings") {
                "mpsettings".to_string()
            } else {
                "mp".to_string()
            }
        }
        "mpsent" | "mpdelete" | "mpdel" => "mp".to_string(),
        "helpetting" => "helpsetting".to_string(),
        _ => normalized,
    }
}

pub fn all_command_keys() -> Vec<String> {
    let mut keys = BTreeSet::new();

    for meta in crate::commands::all_command_metadata() {
        keys.insert(command_key(meta.name, &[]));
    }

    for key in EXTRA_COMMAND_KEYS {
        keys.insert((*key).to_string());
    }

    keys.into_iter().collect()
}

fn metadata_key_for_permission(command_key: &str) -> &str {
    match command_key {
        "ticket_settings" => "ticket",
        "ticket_add" | "ticket_remove" => "add",
        "ticket_close" => "close",
        "suggestion_create" | "suggestion_settings" => "suggestion",
        "setperm" => "set",
        "delperm" => "del",
        "changereset" => "change",
        "serverlist" => "server",
        "endgiveaway" => "end",
        "mpsettings" => "mp",
        _ => command_key,
    }
}

pub fn default_permission(command_key: &str) -> u8 {
    let metadata_key = metadata_key_for_permission(command_key);
    crate::commands::command_metadata_by_key(metadata_key)
        .map(|meta| meta.default_permission)
        .unwrap_or(0)
}

fn is_forced_owner_from_env(user_id: UserId) -> bool {
    // IDs acceptes: listes CSV ou valeur simple dans plusieurs noms d'ENV.
    let keys = ["FORCE_OWNER_IDS", "OWNER_IDS", "OWNER_ID", "BOT_OWNER_ID"];

    for key in keys {
        let Ok(raw) = env::var(key) else {
            continue;
        };

        for part in raw.split([',', ';', ' ', '\n', '\t']) {
            let token = part.trim();
            if token.is_empty() {
                continue;
            }

            if let Ok(id) = token.parse::<u64>() {
                if id == user_id.get() {
                    return true;
                }
            }
        }
    }

    false
}

async fn is_owner(ctx: &Context, user_id: UserId) -> bool {
    if is_forced_owner_from_env(user_id) {
        return true;
    }

    if let Ok(info) = ctx.http.get_current_application_info().await {
        if let Some(owner) = info.owner {
            if owner.id == user_id {
                return true;
            }
        }
    }

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    if let Some(pool) = pool {
        return is_bot_owner(&pool, bot_id, user_id).await.unwrap_or(false);
    }

    false
}

pub async fn resolve_prefix(ctx: &Context, guild_id: Option<GuildId>) -> String {
    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    if let Some(pool) = pool {
        if let Some(gid) = guild_id {
            if let Ok(Some(p)) = get_guild_prefix(&pool, bot_id, gid).await {
                if !p.is_empty() {
                    return p;
                }
            }
        }

        if let Ok(Some(p)) = get_main_prefix(&pool, bot_id).await {
            if !p.is_empty() {
                return p;
            }
        }
    }

    "+".to_string()
}

pub async fn command_required_permission(ctx: &Context, command_key: &str) -> u8 {
    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    if let Some(pool) = pool {
        if let Ok(Some(level)) = get_command_permission(&pool, bot_id, command_key).await {
            return level;
        }
    }

    default_permission(command_key)
}

pub async fn can_use_command(ctx: &Context, msg: &Message, command_key: &str) -> bool {
    if is_owner(ctx, msg.author.id).await {
        return true;
    }

    let required = command_required_permission(ctx, command_key).await;
    if required == 0 {
        return true;
    }

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    let Some(pool) = pool else {
        return false;
    };

    let role_ids: Vec<RoleId> = if let Some(guild_id) = msg.guild_id {
        guild_id
            .member(&ctx.http, msg.author.id)
            .await
            .map(|m| m.roles)
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    if has_command_access(&pool, bot_id, msg.author.id, &role_ids, command_key)
        .await
        .unwrap_or(false)
    {
        return true;
    }

    has_perm_level_access(&pool, bot_id, msg.author.id, &role_ids, required)
        .await
        .unwrap_or(false)
}

pub async fn deny_permission(ctx: &Context, msg: &Message, command_key: &str, required: u8) {
    let embed = CreateEmbed::new()
        .title("Accès refusé")
        .description(format!(
            "Permission insuffisante pour `{}`. Niveau requis: `{}`.",
            command_key, required
        ))
        .color(0xED4245);
    send_embed(ctx, msg, embed).await;
}

pub async fn is_owner_user(ctx: &Context, user_id: UserId) -> bool {
    is_owner(ctx, user_id).await
}
