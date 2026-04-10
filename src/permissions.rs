use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

use crate::commands::common::send_embed;
use crate::db::{
    DbPoolKey, get_command_permission, get_guild_prefix, get_main_prefix, has_command_access,
    has_perm_level_access, is_bot_owner,
};

pub fn command_key(command: &str, args: &[&str]) -> String {
    match command {
        "ticket" => "ticket_settings".to_string(),
        "claim" => "claim".to_string(),
        "rename" => "rename".to_string(),
        "add" => "ticket_add".to_string(),
        "del" => {
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("perm"))
                .unwrap_or(false)
            {
                "del_perm".to_string()
            } else if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("sanction"))
                .unwrap_or(false)
            {
                "del_sanction".to_string()
            } else {
                "ticket_remove".to_string()
            }
        }
        "close" => "ticket_close".to_string(),
        "tickets" => "tickets".to_string(),
        "show" => {
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("pics"))
                .unwrap_or(false)
            {
                "show_pics".to_string()
            } else {
                "show".to_string()
            }
        }
        "showpics" => "show_pics".to_string(),
        "suggestion" => {
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("settings"))
                .unwrap_or(false)
            {
                "suggestion_settings".to_string()
            } else {
                "suggestion_create".to_string()
            }
        }
        "autopublish" => "autopublish".to_string(),
        "tempvoc" => {
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("cmd"))
                .unwrap_or(false)
            {
                "tempvoc_cmd".to_string()
            } else {
                "tempvoc".to_string()
            }
        }
        "clear" => {
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("owners"))
                .unwrap_or(false)
            {
                "clear_owners".to_string()
            } else if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("bl"))
                .unwrap_or(false)
            {
                "clear_bl".to_string()
            } else if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("perms"))
                .unwrap_or(false)
            {
                "clear_perms".to_string()
            } else if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("sanctions"))
                .unwrap_or(false)
            {
                "clear_sanctions".to_string()
            } else if args.len() >= 2
                && args[0].eq_ignore_ascii_case("all")
                && args[1].eq_ignore_ascii_case("sanctions")
            {
                "clear_all_sanctions".to_string()
            } else {
                "clear_messages".to_string()
            }
        }
        "change" => {
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("reset"))
                .unwrap_or(false)
            {
                "change_reset".to_string()
            } else {
                "change".to_string()
            }
        }
        "remove" => {
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("activity"))
                .unwrap_or(false)
            {
                "remove_activity".to_string()
            } else {
                "remove".to_string()
            }
        }
        "set" => {
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("perm"))
                .unwrap_or(false)
            {
                "set_perm".to_string()
            } else if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("modlogs"))
                .unwrap_or(false)
            {
                "set_modlogs".to_string()
            } else if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("boostembed"))
                .unwrap_or(false)
            {
                "set_boostembed".to_string()
            } else {
                "set".to_string()
            }
        }
        "mp" => {
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("settings"))
                .unwrap_or(false)
            {
                "mp_settings".to_string()
            } else if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("sent"))
                .unwrap_or(false)
            {
                "mp".to_string()
            } else if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("delete") || s.eq_ignore_ascii_case("del"))
                .unwrap_or(false)
            {
                "mp".to_string()
            } else {
                "mp".to_string()
            }
        }
        "server" => {
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("list"))
                .unwrap_or(false)
            {
                "server_list".to_string()
            } else {
                "server".to_string()
            }
        }
        "end" => {
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("giveaway"))
                .unwrap_or(false)
            {
                "end_giveaway".to_string()
            } else {
                "end".to_string()
            }
        }
        "help" => "help".to_string(),
        "helptype" => "helptype".to_string(),
        "helpalias" => "helpalias".to_string(),
        "alias" => "alias".to_string(),
        "modlog" => "modlog".to_string(),
        "messagelog" => "messagelog".to_string(),
        "voicelog" => "voicelog".to_string(),
        "boostlog" => "boostlog".to_string(),
        "rolelog" => "rolelog".to_string(),
        "raidlog" => "raidlog".to_string(),
        "autoconfiglog" => "autoconfiglog".to_string(),
        "join" => "join".to_string(),
        "boostembed" => "boostembed".to_string(),
        "nolog" => "nolog".to_string(),
        "invite" => "invite".to_string(),
        "leave" => {
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("settings"))
                .unwrap_or(false)
            {
                "leave_settings".to_string()
            } else {
                "leave".to_string()
            }
        }
        "discussion" => "discussion".to_string(),
        other => other.to_string(),
    }
}

pub fn all_command_keys() -> Vec<String> {
    vec![
        "ping",
        "allbots",
        "alladmins",
        "botadmins",
        "boosters",
        "rolemembers",
        "serverinfo",
        "vocinfo",
        "role",
        "channel",
        "user",
        "member",
        "pic",
        "banner",
        "server",
        "snipe",
        "emoji",
        "giveaway",
        "end",
        "end_giveaway",
        "sanctions",
        "del_sanction",
        "clear_sanctions",
        "clear_all_sanctions",
        "clear_messages",
        "warn",
        "mute",
        "tempmute",
        "unmute",
        "cmute",
        "tempcmute",
        "uncmute",
        "mutelist",
        "unmuteall",
        "kick",
        "ban",
        "tempban",
        "unban",
        "banlist",
        "lock",
        "unlock",
        "lockall",
        "unlockall",
        "hide",
        "unhide",
        "hideall",
        "unhideall",
        "addrole",
        "delrole",
        "derank",
        "reroll",
        "choose",
        "embed",
        "backup",
        "ticket_settings",
        "claim",
        "rename",
        "ticket_add",
        "ticket_remove",
        "ticket_close",
        "tickets",
        "show_pics",
        "suggestion_create",
        "suggestion_settings",
        "autopublish",
        "tempvoc",
        "tempvoc_cmd",
        "autobackup",
        "loading",
        "create",
        "newsticker",
        "massiverole",
        "unmassiverole",
        "voicemove",
        "voicekick",
        "cleanup",
        "bringall",
        "renew",
        "unbanall",
        "temprole",
        "untemprole",
        "sync",
        "button",
        "autoreact",
        "calc",
        "shadowbot",
        "set",
        "set_modlogs",
        "set_boostembed",
        "theme",
        "playto",
        "listen",
        "watch",
        "compet",
        "stream",
        "remove_activity",
        "online",
        "idle",
        "dnd",
        "invisible",
        "owner",
        "unowner",
        "clear_owners",
        "bl",
        "unbl",
        "blinfo",
        "clear_bl",
        "say",
        "change",
        "changeall",
        "change_reset",
        "mainprefix",
        "prefix",
        "perms",
        "set_perm",
        "del_perm",
        "clear_perms",
        "allperms",
        "help",
        "helptype",
        "helpalias",
        "alias",
        "modlog",
        "messagelog",
        "voicelog",
        "boostlog",
        "rolelog",
        "raidlog",
        "autoconfiglog",
        "join",
        "leave_settings",
        "boostembed",
        "nolog",
        "mp",
        "mp_settings",
        "server_list",
        "invite",
        "leave",
        "discussion",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

pub fn default_permission(command_key: &str) -> u8 {
    match command_key {
        "ticket_settings" | "suggestion_settings" | "autopublish" | "tempvoc" => 8,
        "claim" | "rename" | "ticket_add" | "ticket_remove" | "ticket_close" | "tickets" => 2,
        "show_pics" | "suggestion_create" | "tempvoc_cmd" => 0,
        "owner" | "unowner" | "clear_owners" => 9,
        "bl" | "unbl" | "blinfo" | "clear_bl" => 9,
        "change" | "changeall" | "change_reset" | "mainprefix" | "set_perm" | "del_perm"
        | "clear_perms" => 9,
        "set_modlogs" | "set_boostembed" => 8,
        "prefix" | "perms" | "allperms" => 8,
        "help" | "server_list" => 0,
        "helptype" | "helpalias" | "alias" | "leave" => 9,
        "mp_settings" => 9,
        "mp" | "mp_sent" | "mp_delete" | "invite" | "discussion" => 8,
        "set"
        | "theme"
        | "playto"
        | "listen"
        | "watch"
        | "compet"
        | "stream"
        | "remove_activity"
        | "online"
        | "idle"
        | "dnd"
        | "invisible"
        | "say"
        | "giveaway"
        | "end_giveaway"
        | "reroll"
        | "choose"
        | "embed"
        | "backup"
        | "autobackup"
        | "loading"
        | "create"
        | "newsticker"
        | "massiverole"
        | "unmassiverole"
        | "voicemove"
        | "voicekick"
        | "cleanup"
        | "bringall"
        | "renew"
        | "unbanall"
        | "temprole"
        | "untemprole"
        | "sync"
        | "button"
        | "autoreact"
        | "sanctions"
        | "del_sanction"
        | "clear_sanctions"
        | "clear_all_sanctions"
        | "clear_messages"
        | "warn"
        | "mute"
        | "tempmute"
        | "unmute"
        | "cmute"
        | "tempcmute"
        | "uncmute"
        | "mutelist"
        | "unmuteall"
        | "kick"
        | "ban"
        | "tempban"
        | "unban"
        | "banlist"
        | "lock"
        | "unlock"
        | "lockall"
        | "unlockall"
        | "hide"
        | "unhide"
        | "hideall"
        | "unhideall"
        | "addrole"
        | "delrole"
        | "derank"
        | "modlog"
        | "messagelog"
        | "voicelog"
        | "boostlog"
        | "rolelog"
        | "raidlog"
        | "autoconfiglog"
        | "join"
        | "boostembed"
        | "leave_settings"
        | "nolog" => 8,
        _ => 0,
    }
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
