use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::parse_user_id;
use crate::commands::common::{parse_channel_id, parse_role, send_embed, theme_color};
use crate::db::DbPoolKey;

static MODERATION_TICK: OnceLock<Mutex<Instant>> = OnceLock::new();

fn duration_from_input(input: &str) -> Option<Duration> {
    let raw = input.trim().to_lowercase();
    if raw.is_empty() {
        return None;
    }

    let mut number = String::new();
    let mut suffix = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_digit() {
            if !suffix.is_empty() {
                return None;
            }
            number.push(ch);
        } else if !ch.is_whitespace() {
            suffix.push(ch);
        }
    }

    let value = number.parse::<u64>().ok()?;
    let secs = match suffix.as_str() {
        "s" | "sec" | "secs" | "seconde" | "secondes" => value,
        "m" | "min" | "mins" | "minute" | "minutes" => value * 60,
        "h" | "heure" | "heures" => value * 3600,
        "j" | "d" | "jour" | "jours" => value * 86400,
        _ => return None,
    };

    Some(Duration::from_secs(secs.max(1)))
}

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

async fn add_sanction(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
    moderator_id: UserId,
    kind: &str,
    reason: &str,
    channel_id: Option<ChannelId>,
    expires_at: Option<chrono::DateTime<Utc>>,
) {
    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id;
    let _ = sqlx::query(
        r#"
        INSERT INTO bot_sanctions
            (bot_id, guild_id, user_id, moderator_id, kind, reason, channel_id, expires_at, active)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, TRUE);
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(user_id.get() as i64)
    .bind(moderator_id.get() as i64)
    .bind(kind)
    .bind(reason)
    .bind(channel_id.map(|c| c.get() as i64))
    .bind(expires_at)
    .execute(&pool)
    .await;
}

async fn parse_targets(raw: &str) -> Vec<UserId> {
    let mut out = Vec::new();
    for token in raw.split(',') {
        if let Some(uid) = parse_user_id(token.trim()) {
            out.push(uid);
        }
    }
    out
}

async fn handle_timeout(
    ctx: &Context,
    guild_id: GuildId,
    users: &[UserId],
    expires: Option<chrono::DateTime<Utc>>,
) -> usize {
    let mut done = 0usize;
    for user_id in users {
        if let Ok(mut member) = guild_id.member(&ctx.http, *user_id).await {
            let mut builder = serenity::builder::EditMember::new();
            if let Some(ts) = expires {
                if let Ok(discord_ts) = Timestamp::from_unix_timestamp(ts.timestamp()) {
                    builder = builder.disable_communication_until_datetime(discord_ts);
                }
            } else {
                builder = builder.enable_communication();
            }

            if member.edit(&ctx.http, builder).await.is_ok() {
                done += 1;
            }
        }
    }
    done
}

async fn channel_mute_users(
    ctx: &Context,
    channel_id: ChannelId,
    users: &[UserId],
    mute: bool,
) -> usize {
    let mut done = 0usize;
    for user_id in users {
        let result = if mute {
            channel_id
                .create_permission(
                    &ctx.http,
                    PermissionOverwrite {
                        allow: Permissions::empty(),
                        deny: Permissions::SEND_MESSAGES
                            | Permissions::ADD_REACTIONS
                            | Permissions::SPEAK,
                        kind: PermissionOverwriteType::Member(*user_id),
                    },
                )
                .await
        } else {
            channel_id
                .delete_permission(&ctx.http, PermissionOverwriteType::Member(*user_id))
                .await
        };

        if result.is_ok() {
            done += 1;
        }
    }
    done
}

pub async fn handle_sanctions(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    let Some(target_raw) = args.first() else {
        let _ = send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Sanctions")
                .description("Usage: +sanctions <membre>")
                .color(0xED4245),
        )
        .await;
        return;
    };
    let Some(target) = parse_user_id(target_raw) else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let rows = sqlx::query_as::<
        _,
        (
            i64,
            String,
            String,
            chrono::DateTime<Utc>,
            Option<chrono::DateTime<Utc>>,
            bool,
        ),
    >(
        r#"
        SELECT id, kind, reason, created_at, expires_at, active
        FROM bot_sanctions
        WHERE bot_id = $1 AND guild_id = $2 AND user_id = $3
        ORDER BY created_at DESC
        LIMIT 30;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(target.get() as i64)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let desc = if rows.is_empty() {
        "Aucune sanction.".to_string()
    } else {
        rows.into_iter()
            .map(|(id, kind, reason, created_at, expires_at, active)| {
                let until = expires_at
                    .map(|d| format!(" · jusqu'à <t:{}:R>", d.timestamp()))
                    .unwrap_or_default();
                format!(
                    "`#{}` `{}` {} · <t:{}:R>{} · {}",
                    id,
                    kind,
                    if active { "(active)" } else { "(inactive)" },
                    created_at.timestamp(),
                    until,
                    reason
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title(format!("Sanctions de <@{}>", target.get()))
            .description(desc)
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_del_sanction(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.len() < 3 {
        return;
    }

    let Some(target) = parse_user_id(args[1]) else {
        return;
    };
    let Ok(index) = args[2].parse::<usize>() else {
        return;
    };
    if index == 0 {
        return;
    }

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let rows = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT id
        FROM bot_sanctions
        WHERE bot_id = $1 AND guild_id = $2 AND user_id = $3
        ORDER BY created_at DESC;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(target.get() as i64)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let Some((sanction_id,)) = rows.get(index - 1).copied() else {
        return;
    };

    let _ = sqlx::query(
        r#"
        DELETE FROM bot_sanctions
        WHERE id = $1 AND bot_id = $2 AND guild_id = $3;
        "#,
    )
    .bind(sanction_id)
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .execute(&pool)
    .await;

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Sanctions")
            .description(format!(
                "Sanction #{} supprimée pour <@{}>.",
                sanction_id,
                target.get()
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_clear_sanctions(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.len() < 2 {
        return;
    }

    let Some(target) = parse_user_id(args[1]) else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let removed = sqlx::query(
        r#"
        DELETE FROM bot_sanctions
        WHERE bot_id = $1 AND guild_id = $2 AND user_id = $3;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(target.get() as i64)
    .execute(&pool)
    .await
    .ok()
    .map(|r| r.rows_affected())
    .unwrap_or(0);

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Sanctions")
            .description(format!(
                "{} sanction(s) supprimée(s) pour <@{}>.",
                removed,
                target.get()
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_clear_all_sanctions(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let removed = sqlx::query(
        r#"
        DELETE FROM bot_sanctions
        WHERE bot_id = $1 AND guild_id = $2;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .execute(&pool)
    .await
    .ok()
    .map(|r| r.rows_affected())
    .unwrap_or(0);

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Sanctions")
            .description(format!(
                "{} sanction(s) supprimée(s) sur le serveur.",
                removed
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_clear_messages(ctx: &Context, msg: &Message, args: &[&str]) {
    let Ok(mut amount) = args.first().unwrap_or(&"0").parse::<u64>() else {
        return;
    };
    if amount == 0 {
        return;
    }
    amount = amount.clamp(1, 100);

    let filter_user = args.get(1).and_then(|raw| parse_user_id(raw));

    let mut deleted = 0usize;
    if let Ok(messages) = msg
        .channel_id
        .messages(
            &ctx.http,
            serenity::builder::GetMessages::new().limit(amount as u8 + 1),
        )
        .await
    {
        for m in messages {
            if m.id == msg.id {
                continue;
            }
            if let Some(user_id) = filter_user {
                if m.author.id != user_id {
                    continue;
                }
            }
            if msg.channel_id.delete_message(&ctx.http, m.id).await.is_ok() {
                deleted += 1;
            }
        }
    }

    let _ = send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Clear")
            .description(format!("{} message(s) supprimé(s).", deleted))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_warn(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.is_empty() {
        return;
    }

    let targets = parse_targets(args[0]).await;
    if targets.is_empty() {
        return;
    }
    let reason = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Aucune raison".to_string()
    };

    for uid in &targets {
        add_sanction(
            ctx,
            guild_id,
            *uid,
            msg.author.id,
            "warn",
            &reason,
            None,
            None,
        )
        .await;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Warn")
            .description(format!("{} membre(s) warn.", targets.len()))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_mute(ctx: &Context, msg: &Message, args: &[&str], temporary: bool) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.is_empty() {
        return;
    }

    let targets = parse_targets(args[0]).await;
    if targets.is_empty() {
        return;
    }

    let (expires_at, reason_start_idx) = if temporary {
        let Some(duration_raw) = args.get(1) else {
            return;
        };
        let Some(duration) = duration_from_input(duration_raw) else {
            return;
        };
        (
            Some(Utc::now() + chrono::Duration::seconds(duration.as_secs() as i64)),
            2,
        )
    } else {
        // "indéfini" = 28 jours max via timeout Discord.
        (
            Some(Utc::now() + chrono::Duration::seconds(28 * 24 * 3600)),
            1,
        )
    };

    let reason = if args.len() > reason_start_idx {
        args[reason_start_idx..].join(" ")
    } else {
        "Aucune raison".to_string()
    };

    let affected = handle_timeout(ctx, guild_id, &targets, expires_at).await;

    for uid in &targets {
        add_sanction(
            ctx,
            guild_id,
            *uid,
            msg.author.id,
            if temporary { "tempmute" } else { "mute" },
            &reason,
            None,
            expires_at,
        )
        .await;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title(if temporary { "TempMute" } else { "Mute" })
            .description(format!("{} membre(s) mute.", affected))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_unmute(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.is_empty() {
        return;
    }

    let targets = parse_targets(args[0]).await;
    if targets.is_empty() {
        return;
    }

    let affected = handle_timeout(ctx, guild_id, &targets, None).await;

    if let Some(pool) = pool(ctx).await {
        let bot_id = ctx.cache.current_user().id;
        for uid in &targets {
            let _ = sqlx::query(
                r#"
                UPDATE bot_sanctions
                SET active = FALSE
                WHERE bot_id = $1 AND guild_id = $2 AND user_id = $3 AND active = TRUE AND kind IN ('mute','tempmute');
                "#,
            )
            .bind(bot_id.get() as i64)
            .bind(guild_id.get() as i64)
            .bind(uid.get() as i64)
            .execute(&pool)
            .await;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("UnMute")
            .description(format!("{} membre(s) unmute.", affected))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_cmute(ctx: &Context, msg: &Message, args: &[&str], temporary: bool) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.is_empty() {
        return;
    }

    let targets = parse_targets(args[0]).await;
    if targets.is_empty() {
        return;
    }

    let (expires_at, reason_start_idx) = if temporary {
        let Some(duration_raw) = args.get(1) else {
            return;
        };
        let Some(duration) = duration_from_input(duration_raw) else {
            return;
        };
        (
            Some(Utc::now() + chrono::Duration::seconds(duration.as_secs() as i64)),
            2,
        )
    } else {
        (None, 1)
    };

    let reason = if args.len() > reason_start_idx {
        args[reason_start_idx..].join(" ")
    } else {
        "Aucune raison".to_string()
    };

    let affected = channel_mute_users(ctx, msg.channel_id, &targets, true).await;

    for uid in &targets {
        add_sanction(
            ctx,
            guild_id,
            *uid,
            msg.author.id,
            if temporary { "tempcmute" } else { "cmute" },
            &reason,
            Some(msg.channel_id),
            expires_at,
        )
        .await;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title(if temporary { "TempCMute" } else { "CMute" })
            .description(format!("{} membre(s) cmute.", affected))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_uncmute(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.is_empty() {
        return;
    }

    let targets = parse_targets(args[0]).await;
    if targets.is_empty() {
        return;
    }

    let affected = channel_mute_users(ctx, msg.channel_id, &targets, false).await;

    if let Some(pool) = pool(ctx).await {
        let bot_id = ctx.cache.current_user().id;
        for uid in &targets {
            let _ = sqlx::query(
                r#"
                UPDATE bot_sanctions
                SET active = FALSE
                WHERE bot_id = $1 AND guild_id = $2 AND user_id = $3 AND active = TRUE AND kind IN ('cmute','tempcmute') AND channel_id = $4;
                "#,
            )
            .bind(bot_id.get() as i64)
            .bind(guild_id.get() as i64)
            .bind(uid.get() as i64)
            .bind(msg.channel_id.get() as i64)
            .execute(&pool)
            .await;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("UnCMute")
            .description(format!("{} membre(s) uncmute.", affected))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_mutelist(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let rows = sqlx::query_as::<_, (i64, String, Option<i64>, Option<chrono::DateTime<Utc>>)>(
        r#"
        SELECT user_id, kind, channel_id, expires_at
        FROM bot_sanctions
        WHERE bot_id = $1 AND guild_id = $2 AND active = TRUE AND kind IN ('mute','tempmute','cmute','tempcmute')
        ORDER BY created_at DESC
        LIMIT 60;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let desc = if rows.is_empty() {
        "Aucun mute en cours.".to_string()
    } else {
        rows.into_iter()
            .map(|(uid, kind, channel_id, exp)| {
                let channel = channel_id
                    .map(|c| format!(" dans <#{}>", c))
                    .unwrap_or_default();
                let until = exp
                    .map(|d| format!(" jusqu'à <t:{}:R>", d.timestamp()))
                    .unwrap_or_default();
                format!("- <@{}> `{}`{}{}", uid, kind, channel, until)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("MuteList")
            .description(desc)
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_unmuteall(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let rows = sqlx::query_as::<_, (i64, String, Option<i64>)>(
        r#"
        SELECT user_id, kind, channel_id
        FROM bot_sanctions
        WHERE bot_id = $1 AND guild_id = $2 AND active = TRUE AND kind IN ('mute','tempmute','cmute','tempcmute');
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let mut changed = 0usize;
    for (uid, kind, channel_id) in rows {
        let user_id = UserId::new(uid as u64);
        if kind == "mute" || kind == "tempmute" {
            changed += handle_timeout(ctx, guild_id, &[user_id], None).await;
        } else if let Some(cid) = channel_id {
            changed += channel_mute_users(ctx, ChannelId::new(cid as u64), &[user_id], false).await;
        }
    }

    let _ = sqlx::query(
        r#"
        UPDATE bot_sanctions
        SET active = FALSE
        WHERE bot_id = $1 AND guild_id = $2 AND active = TRUE AND kind IN ('mute','tempmute','cmute','tempcmute');
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .execute(&pool)
    .await;

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("UnMuteAll")
            .description(format!(
                "{} opération(s) de unmute/cmute annulé(es).",
                changed
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_kick(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.is_empty() {
        return;
    }

    let targets = parse_targets(args[0]).await;
    if targets.is_empty() {
        return;
    }

    let reason = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Aucune raison".to_string()
    };

    let mut done = 0usize;
    for uid in &targets {
        if guild_id
            .kick_with_reason(&ctx.http, *uid, &reason)
            .await
            .is_ok()
        {
            done += 1;
            add_sanction(
                ctx,
                guild_id,
                *uid,
                msg.author.id,
                "kick",
                &reason,
                None,
                None,
            )
            .await;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Kick")
            .description(format!("{} membre(s) expulsé(s).", done))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_ban(ctx: &Context, msg: &Message, args: &[&str], temporary: bool) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.is_empty() {
        return;
    }

    let targets = parse_targets(args[0]).await;
    if targets.is_empty() {
        return;
    }

    let (expires_at, reason_start_idx) = if temporary {
        let Some(duration_raw) = args.get(1) else {
            return;
        };
        let Some(duration) = duration_from_input(duration_raw) else {
            return;
        };
        (
            Some(Utc::now() + chrono::Duration::seconds(duration.as_secs() as i64)),
            2,
        )
    } else {
        (None, 1)
    };

    let reason = if args.len() > reason_start_idx {
        args[reason_start_idx..].join(" ")
    } else {
        "Aucune raison".to_string()
    };

    let mut done = 0usize;
    for uid in &targets {
        if guild_id
            .ban_with_reason(&ctx.http, *uid, 0, &reason)
            .await
            .is_ok()
        {
            done += 1;
            add_sanction(
                ctx,
                guild_id,
                *uid,
                msg.author.id,
                if temporary { "tempban" } else { "ban" },
                &reason,
                None,
                expires_at,
            )
            .await;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title(if temporary { "TempBan" } else { "Ban" })
            .description(format!("{} membre(s) banni(s).", done))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_unban(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.is_empty() {
        return;
    }

    let targets = parse_targets(args[0]).await;
    if targets.is_empty() {
        return;
    }

    let mut done = 0usize;
    for uid in &targets {
        if guild_id.unban(&ctx.http, *uid).await.is_ok() {
            done += 1;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("UnBan")
            .description(format!("{} membre(s) unban.", done))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_banlist(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let bans = guild_id
        .bans(&ctx.http, None, None)
        .await
        .unwrap_or_default();
    let desc = if bans.is_empty() {
        "Aucun ban en cours.".to_string()
    } else {
        bans.into_iter()
            .map(|ban| format!("- <@{}> ({})", ban.user.id.get(), ban.user.tag()))
            .collect::<Vec<_>>()
            .join("\n")
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("BanList")
            .description(desc)
            .color(theme_color(ctx).await),
    )
    .await;
}

async fn edit_channel_visibility(
    ctx: &Context,
    guild_id: GuildId,
    channel_id: ChannelId,
    lock: Option<bool>,
    hide: Option<bool>,
) -> bool {
    let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await else {
        return false;
    };

    let everyone_role = guild
        .roles
        .values()
        .find(|r| r.name == "@everyone")
        .map(|r| r.id);
    let Some(everyone_role) = everyone_role else {
        return false;
    };

    let Ok(channels) = guild_id.channels(&ctx.http).await else {
        return false;
    };
    let Some(channel) = channels.get(&channel_id) else {
        return false;
    };

    let mut allow = Permissions::empty();
    let mut deny = Permissions::empty();

    if let Some(locked) = lock {
        if channel.kind == ChannelType::Text || channel.kind == ChannelType::News {
            if locked {
                deny |= Permissions::SEND_MESSAGES;
            } else {
                allow |= Permissions::SEND_MESSAGES;
            }
        } else {
            if locked {
                deny |= Permissions::CONNECT | Permissions::SPEAK;
            } else {
                allow |= Permissions::CONNECT | Permissions::SPEAK;
            }
        }
    }

    if let Some(hidden) = hide {
        if hidden {
            deny |= Permissions::VIEW_CHANNEL;
        } else {
            allow |= Permissions::VIEW_CHANNEL;
        }
    }

    channel_id
        .create_permission(
            &ctx.http,
            PermissionOverwrite {
                allow,
                deny,
                kind: PermissionOverwriteType::Role(everyone_role),
            },
        )
        .await
        .is_ok()
}

pub async fn handle_lock_unlock(ctx: &Context, msg: &Message, args: &[&str], lock: bool) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let target = args
        .first()
        .and_then(|raw| parse_channel_id(raw))
        .unwrap_or(msg.channel_id);

    let ok = edit_channel_visibility(ctx, guild_id, target, Some(lock), None).await;

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title(if lock { "Lock" } else { "Unlock" })
            .description(if ok {
                format!("Salon <#{}> mis à jour.", target.get())
            } else {
                "Échec de mise à jour du salon.".to_string()
            })
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_hide_unhide(ctx: &Context, msg: &Message, args: &[&str], hide: bool) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let target = args
        .first()
        .and_then(|raw| parse_channel_id(raw))
        .unwrap_or(msg.channel_id);

    let ok = edit_channel_visibility(ctx, guild_id, target, None, Some(hide)).await;

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title(if hide { "Hide" } else { "UnHide" })
            .description(if ok {
                format!("Salon <#{}> mis à jour.", target.get())
            } else {
                "Échec de mise à jour du salon.".to_string()
            })
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_lockall_unlockall(ctx: &Context, msg: &Message, lock: bool) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    let Ok(channels) = guild_id.channels(&ctx.http).await else {
        return;
    };

    let mut changed = 0usize;
    for channel_id in channels.keys() {
        if edit_channel_visibility(ctx, guild_id, *channel_id, Some(lock), None).await {
            changed += 1;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title(if lock { "LockAll" } else { "UnlockAll" })
            .description(format!("{} salon(s) mis à jour.", changed))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_hideall_unhideall(ctx: &Context, msg: &Message, hide: bool) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    let Ok(channels) = guild_id.channels(&ctx.http).await else {
        return;
    };

    let mut changed = 0usize;
    for channel_id in channels.keys() {
        if edit_channel_visibility(ctx, guild_id, *channel_id, None, Some(hide)).await {
            changed += 1;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title(if hide { "HideAll" } else { "UnHideAll" })
            .description(format!("{} salon(s) mis à jour.", changed))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_add_del_role(ctx: &Context, msg: &Message, args: &[&str], add: bool) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.len() < 2 {
        return;
    }

    let Some(target) = parse_user_id(args[0]) else {
        return;
    };
    let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await else {
        return;
    };
    let Some(role) = parse_role(&guild, args[1]) else {
        return;
    };

    let done = if let Ok(member) = guild_id.member(&ctx.http, target).await {
        let r = if add {
            member.add_role(&ctx.http, role.id).await
        } else {
            member.remove_role(&ctx.http, role.id).await
        };
        r.is_ok()
    } else {
        false
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title(if add { "AddRole" } else { "DelRole" })
            .description(if done {
                format!(
                    "Rôle <@&{}> {} à <@{}>.",
                    role.id.get(),
                    if add { "ajouté" } else { "retiré" },
                    target.get()
                )
            } else {
                "Échec de modification du rôle.".to_string()
            })
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_derank(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.is_empty() {
        return;
    }

    let targets = parse_targets(args[0]).await;
    if targets.is_empty() {
        return;
    }

    let mut done = 0usize;
    for uid in &targets {
        if let Ok(member) = guild_id.member(&ctx.http, *uid).await {
            let roles = member.roles.clone();
            let mut ok = true;
            for role_id in roles {
                if member.remove_role(&ctx.http, role_id).await.is_err() {
                    ok = false;
                }
            }
            if ok {
                done += 1;
            }
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Derank")
            .description(format!("{} membre(s) dérank.", done))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn maybe_run_maintenance(ctx: &Context, guild_id: Option<GuildId>) {
    let Some(guild_id) = guild_id else {
        return;
    };

    let now = Instant::now();
    let lock = MODERATION_TICK.get_or_init(|| Mutex::new(Instant::now() - Duration::from_secs(60)));
    {
        let mut last = lock.lock().expect("moderation tick lock poisoned");
        if now.duration_since(*last) < Duration::from_secs(30) {
            return;
        }
        *last = now;
    }

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;
    let now_dt = Utc::now();

    let rows = sqlx::query_as::<_, (i64, i64, String, Option<i64>)>(
        r#"
        SELECT id, user_id, kind, channel_id
        FROM bot_sanctions
        WHERE bot_id = $1 AND guild_id = $2 AND active = TRUE AND expires_at IS NOT NULL AND expires_at <= $3;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(now_dt)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    for (id, uid, kind, channel_id) in &rows {
        let user_id = UserId::new(*uid as u64);
        if kind == "tempmute" {
            let _ = handle_timeout(ctx, guild_id, &[user_id], None).await;
        } else if kind == "tempcmute" {
            if let Some(cid) = channel_id {
                let _ =
                    channel_mute_users(ctx, ChannelId::new(*cid as u64), &[user_id], false).await;
            }
        } else if kind == "tempban" {
            let _ = guild_id.unban(&ctx.http, user_id).await;
        }

        let _ = sqlx::query("UPDATE bot_sanctions SET active = FALSE WHERE id = $1")
            .bind(*id)
            .execute(&pool)
            .await;
    }
}
