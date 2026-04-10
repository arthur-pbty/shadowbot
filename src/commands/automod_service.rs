use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::commands::moderation_sanction_helpers::{add_sanction, handle_timeout};
use crate::db::{
    self, DbPoolKey, ModerationSettings, PunishRule, count_member_strikes_in_window,
    ensure_default_punish_rules, get_last_punish_triggered_at, upsert_last_punish_triggered_at,
};
use crate::permissions;

static SPAM_TRACKER: OnceLock<Mutex<HashMap<(u64, u64, u64), VecDeque<Instant>>>> = OnceLock::new();

pub async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

pub fn parse_on_off(input: &str) -> Option<bool> {
    match input.trim().to_lowercase().as_str() {
        "on" | "enable" | "enabled" | "true" | "1" => Some(true),
        "off" | "disable" | "disabled" | "false" | "0" => Some(false),
        _ => None,
    }
}

pub fn parse_duration_to_seconds(input: &str) -> Option<i64> {
    let raw = input.trim().to_lowercase();
    if raw.is_empty() {
        return None;
    }

    let mut digits = String::new();
    let mut suffix = String::new();

    for ch in raw.chars() {
        if ch.is_ascii_digit() {
            if !suffix.is_empty() {
                return None;
            }
            digits.push(ch);
        } else if !ch.is_whitespace() {
            suffix.push(ch);
        }
    }

    let value = digits.parse::<i64>().ok()?;
    if value <= 0 {
        return None;
    }

    let unit = if suffix.is_empty() { "s" } else { &suffix };
    let seconds = match unit {
        "s" | "sec" | "secs" | "seconde" | "secondes" => value,
        "m" | "min" | "mins" | "minute" | "minutes" => value.checked_mul(60)?,
        "h" | "heure" | "heures" => value.checked_mul(3_600)?,
        "j" | "d" | "jour" | "jours" => value.checked_mul(86_400)?,
        "w" | "sem" | "semaine" | "semaines" => value.checked_mul(604_800)?,
        _ => return None,
    };

    Some(seconds.max(1))
}

pub fn format_duration(mut seconds: i64) -> String {
    seconds = seconds.max(1);
    let days = seconds / 86_400;
    seconds %= 86_400;
    let hours = seconds / 3_600;
    seconds %= 3_600;
    let minutes = seconds / 60;
    seconds %= 60;

    let mut out = Vec::new();
    if days > 0 {
        out.push(format!("{}j", days));
    }
    if hours > 0 {
        out.push(format!("{}h", hours));
    }
    if minutes > 0 {
        out.push(format!("{}m", minutes));
    }
    if seconds > 0 || out.is_empty() {
        out.push(format!("{}s", seconds));
    }

    out.join(" ")
}

pub fn parse_rate_limit(input: &str) -> Option<(i32, i32)> {
    let mut parts = input.splitn(2, '/');
    let limit = parts.next()?.trim().parse::<i32>().ok()?.max(1);
    let duration = parse_duration_to_seconds(parts.next()?.trim())?;
    if duration > i32::MAX as i64 {
        return None;
    }

    Some((limit, duration as i32))
}

pub fn parse_trigger(input: &str) -> Option<&'static str> {
    match input.trim().to_lowercase().as_str() {
        "spam" | "antispam" => Some("spam"),
        "link" | "antilink" => Some("link"),
        "massmention" | "antimassmention" | "mention" | "mentions" => Some("massmention"),
        "badword" | "badwords" | "mauvaismot" | "motinterdit" => Some("badword"),
        _ => None,
    }
}

pub fn parse_profile(input: Option<&str>) -> Option<&'static str> {
    let raw = input?.trim().to_lowercase();
    match raw.as_str() {
        "ancien" | "old" => Some("old"),
        "nouveau" | "new" => Some("new"),
        _ => None,
    }
}

pub fn parse_sanction(input: &str) -> Option<&'static str> {
    match input.trim().to_lowercase().as_str() {
        "warn" | "avert" => Some("warn"),
        "mute" | "timeout" => Some("mute"),
        "kick" => Some("kick"),
        "ban" => Some("ban"),
        _ => None,
    }
}

pub fn apply_channel_override(global_enabled: bool, override_mode: Option<&str>) -> bool {
    match override_mode {
        Some(mode) if mode.eq_ignore_ascii_case("allow") => false,
        Some(mode) if mode.eq_ignore_ascii_case("deny") => true,
        _ => global_enabled,
    }
}

fn contains_invite_link(content: &str) -> bool {
    let lower = content.to_lowercase();
    lower.contains("discord.gg/")
        || lower.contains("discord.com/invite/")
        || lower.contains("discordapp.com/invite/")
}

fn contains_any_link(content: &str) -> bool {
    let lower = content.to_lowercase();
    lower.contains("http://")
        || lower.contains("https://")
        || lower.contains("www.")
        || lower.contains("discord.gg/")
}

fn spam_hit(bot_id: u64, guild_id: u64, user_id: u64, limit: i32, window_seconds: i32) -> bool {
    let lock = SPAM_TRACKER.get_or_init(|| Mutex::new(HashMap::new()));
    let mut tracker = lock.lock().expect("spam tracker lock poisoned");

    let key = (bot_id, guild_id, user_id);
    let now = Instant::now();
    let window = Duration::from_secs(window_seconds.max(1) as u64);
    let queue = tracker.entry(key).or_insert_with(VecDeque::new);

    queue.push_back(now);

    while let Some(oldest) = queue.front() {
        if now.duration_since(*oldest) > window {
            let _ = queue.pop_front();
        } else {
            break;
        }
    }

    queue.len() > limit.max(1) as usize
}

async fn user_profile(
    ctx: &Context,
    pool: &sqlx::PgPool,
    bot_id: i64,
    guild_id: GuildId,
    user_id: UserId,
) -> &'static str {
    let Ok(old_settings) =
        db::get_or_create_old_member_settings(pool, bot_id, guild_id.get() as i64).await
    else {
        return "new";
    };

    if !old_settings.enabled {
        return "new";
    }

    let Some(role_id_raw) = old_settings.role_id else {
        return "new";
    };

    let Ok(member) = guild_id.member(&ctx.http, user_id).await else {
        return "new";
    };

    if member
        .roles
        .iter()
        .any(|role_id| role_id.get() as i64 == role_id_raw)
    {
        "old"
    } else {
        "new"
    }
}

async fn execute_rule(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
    rule: &PunishRule,
    settings: &ModerationSettings,
) -> String {
    let sanction = rule.sanction.to_lowercase();
    let bot_user_id = ctx.cache.current_user().id;

    if sanction == "warn" {
        add_sanction(
            ctx,
            guild_id,
            user_id,
            bot_user_id,
            "warn",
            "AutoMod: seuil de strikes atteint.",
            None,
            None,
        )
        .await;
        return "warn".to_string();
    }

    if sanction == "mute" || sanction == "timeout" {
        let duration = rule
            .sanction_seconds
            .unwrap_or(3_600)
            .clamp(1, 28 * 24 * 3_600);
        let expires = Some(Utc::now() + chrono::Duration::seconds(duration));
        let _ = handle_timeout(ctx, guild_id, &[user_id], expires).await;
        add_sanction(
            ctx,
            guild_id,
            user_id,
            bot_user_id,
            "tempmute",
            "AutoMod: seuil de strikes atteint.",
            None,
            expires,
        )
        .await;
        if settings.use_timeout {
            return format!("timeout {}", format_duration(duration));
        }
        return format!("mute role {}", format_duration(duration));
    }

    if sanction == "kick" {
        let result = guild_id
            .kick_with_reason(&ctx.http, user_id, "AutoMod: seuil de strikes atteint")
            .await;

        if result.is_ok() {
            add_sanction(
                ctx,
                guild_id,
                user_id,
                bot_user_id,
                "kick",
                "AutoMod: seuil de strikes atteint.",
                None,
                None,
            )
            .await;
            return "kick".to_string();
        }

        return "kick (echec)".to_string();
    }

    if sanction == "ban" {
        let result = guild_id
            .ban_with_reason(&ctx.http, user_id, 0, "AutoMod: seuil de strikes atteint")
            .await;

        if result.is_ok() {
            add_sanction(
                ctx,
                guild_id,
                user_id,
                bot_user_id,
                "ban",
                "AutoMod: seuil de strikes atteint.",
                None,
                None,
            )
            .await;
            return "ban".to_string();
        }

        return "ban (echec)".to_string();
    }

    "aucune".to_string()
}

async fn apply_violation(
    ctx: &Context,
    msg: &Message,
    pool: &sqlx::PgPool,
    settings: &ModerationSettings,
    trigger: &str,
    reason: &str,
) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let _ = msg.delete(&ctx.http).await;

    let bot_id = settings.bot_id;
    let guild_id_raw = settings.guild_id;
    let user_id = msg.author.id;
    let profile = user_profile(ctx, pool, bot_id, guild_id, user_id).await;

    let strikes = db::get_strike_rule(pool, bot_id, guild_id_raw, trigger, profile)
        .await
        .ok()
        .flatten()
        .unwrap_or(1)
        .max(0);

    if strikes > 0 {
        let _ = db::add_member_strike_event(
            pool,
            bot_id,
            guild_id_raw,
            user_id.get() as i64,
            trigger,
            strikes,
        )
        .await;
    }

    let _ = ensure_default_punish_rules(pool, bot_id, guild_id_raw).await;
    let rules = db::list_punish_rules(pool, bot_id, guild_id_raw)
        .await
        .unwrap_or_default();

    let mut action = String::from("aucune");
    for rule in rules.iter().rev() {
        let Ok(total) = count_member_strikes_in_window(
            pool,
            bot_id,
            guild_id_raw,
            user_id.get() as i64,
            rule.window_seconds,
        )
        .await
        else {
            continue;
        };

        if total < rule.threshold as i64 {
            continue;
        }

        let recent_trigger =
            get_last_punish_triggered_at(pool, bot_id, guild_id_raw, user_id.get() as i64, rule.id)
                .await
                .ok()
                .flatten()
                .map(|at| Utc::now() - at < chrono::Duration::seconds(rule.window_seconds))
                .unwrap_or(false);

        if recent_trigger {
            continue;
        }

        action = execute_rule(ctx, guild_id, user_id, rule, settings).await;
        let _ = upsert_last_punish_triggered_at(
            pool,
            bot_id,
            guild_id_raw,
            user_id.get() as i64,
            rule.id,
        )
        .await;
        break;
    }

    let embed = CreateEmbed::new()
        .title("AutoMod")
        .description(format!(
            "{}\nMembre: <@{}>\nTrigger: `{}` · Profil: `{}` · Strikes: `+{}`\nAction: `{}`",
            reason,
            user_id.get(),
            trigger,
            profile,
            strikes,
            action
        ))
        .color(0xED4245);
    send_embed(ctx, msg, embed).await;
}

pub async fn enforce_automod_message(ctx: &Context, msg: &Message) -> bool {
    let Some(guild_id) = msg.guild_id else {
        return false;
    };

    let Some(pool) = pool(ctx).await else {
        return false;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let settings =
        match db::get_or_create_moderation_settings(&pool, bot_id, guild_id.get() as i64).await {
            Ok(settings) => settings,
            Err(_) => return false,
        };

    let channel_id = msg.channel_id.get() as i64;
    let spam_override = db::get_moderation_channel_override(
        &pool,
        bot_id,
        guild_id.get() as i64,
        channel_id,
        "spam",
    )
    .await
    .ok()
    .flatten();
    let link_override = db::get_moderation_channel_override(
        &pool,
        bot_id,
        guild_id.get() as i64,
        channel_id,
        "link",
    )
    .await
    .ok()
    .flatten();

    let antispam_enabled =
        apply_channel_override(settings.antispam_enabled, spam_override.as_deref());
    let antilink_enabled =
        apply_channel_override(settings.antilink_enabled, link_override.as_deref());

    if settings.badwords_enabled {
        let content = msg.content.to_lowercase();
        let badwords = db::list_badwords(&pool, bot_id, guild_id.get() as i64)
            .await
            .unwrap_or_default();
        if badwords
            .iter()
            .any(|word| !word.is_empty() && content.contains(word))
        {
            apply_violation(
                ctx,
                msg,
                &pool,
                &settings,
                "badword",
                "Message supprime: mot interdit detecte.",
            )
            .await;
            return true;
        }
    }

    if settings.antimassmention_enabled {
        let mention_count = msg.mentions.len() + msg.mention_roles.len();
        if mention_count >= settings.antimassmention_limit.max(1) as usize {
            apply_violation(
                ctx,
                msg,
                &pool,
                &settings,
                "massmention",
                "Message supprime: spam de mentions detecte.",
            )
            .await;
            return true;
        }
    }

    if antilink_enabled {
        let link_hit = if settings.antilink_mode.eq_ignore_ascii_case("all") {
            contains_any_link(&msg.content)
        } else {
            contains_invite_link(&msg.content)
        };

        if link_hit {
            apply_violation(
                ctx,
                msg,
                &pool,
                &settings,
                "link",
                "Message supprime: lien interdit detecte.",
            )
            .await;
            return true;
        }
    }

    if antispam_enabled {
        let hit = spam_hit(
            ctx.cache.current_user().id.get(),
            guild_id.get(),
            msg.author.id.get(),
            settings.antispam_limit.max(1),
            settings.antispam_window_seconds.max(1),
        );

        if hit {
            apply_violation(
                ctx,
                msg,
                &pool,
                &settings,
                "spam",
                "Message supprime: spam detecte.",
            )
            .await;
            return true;
        }
    }

    false
}

pub async fn public_command_allowed(
    ctx: &Context,
    msg: &Message,
    command_key: &str,
    required_permission: u8,
) -> bool {
    if permissions::is_owner_user(ctx, msg.author.id).await {
        return true;
    }

    if required_permission > 0 {
        return true;
    }

    let Some(guild_id) = msg.guild_id else {
        return true;
    };

    let Some(pool) = pool(ctx).await else {
        return true;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let settings =
        match db::get_or_create_moderation_settings(&pool, bot_id, guild_id.get() as i64).await {
            Ok(settings) => settings,
            Err(_) => return true,
        };

    let override_mode = db::get_moderation_channel_override(
        &pool,
        bot_id,
        guild_id.get() as i64,
        msg.channel_id.get() as i64,
        "public",
    )
    .await
    .ok()
    .flatten();

    let allowed = match override_mode.as_deref() {
        Some(mode) if mode.eq_ignore_ascii_case("allow") => true,
        Some(mode) if mode.eq_ignore_ascii_case("deny") => false,
        _ => settings.public_commands_enabled,
    };
    if allowed {
        return true;
    }

    let embed = CreateEmbed::new()
        .title("Commandes publiques desactivees")
        .description(format!(
            "La commande `{}` est desactivee dans ce salon.",
            command_key.replace('_', " ")
        ))
        .color(0xED4245);
    send_embed(ctx, msg, embed).await;
    false
}
