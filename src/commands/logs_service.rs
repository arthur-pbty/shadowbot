use std::collections::BTreeSet;

use chrono::Utc;
use serenity::builder::{CreateChannel, CreateEmbed};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_channel_id, send_embed, theme_color};
use crate::db::DbPoolKey;

const LOG_TYPES: &[(&str, &str)] = &[
    ("moderation", "modlog"),
    ("message", "messagelog"),
    ("voice", "voicelog"),
    ("boost", "boostlog"),
    ("role", "rolelog"),
    ("raid", "raidlog"),
    ("channel", "channellog"),
];

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

fn parse_target_channel(msg: &Message, args: &[&str], idx: usize) -> Option<ChannelId> {
    args.get(idx)
        .and_then(|raw| parse_channel_id(raw))
        .or(Some(msg.channel_id))
}

async fn set_log_channel(
    ctx: &Context,
    guild_id: GuildId,
    log_type: &str,
    channel_id: Option<ChannelId>,
    enabled: bool,
) {
    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let _ = sqlx::query(
        r#"
        INSERT INTO bot_log_channels (bot_id, guild_id, log_type, channel_id, enabled)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (bot_id, guild_id, log_type)
        DO UPDATE SET channel_id = EXCLUDED.channel_id, enabled = EXCLUDED.enabled, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(log_type)
    .bind(channel_id.map(|c| c.get() as i64))
    .bind(enabled)
    .execute(&pool)
    .await;
}

async fn get_log_channel(ctx: &Context, guild_id: GuildId, log_type: &str) -> Option<ChannelId> {
    let pool = pool(ctx).await?;
    let bot_id = ctx.cache.current_user().id;

    let row = sqlx::query_as::<_, (Option<i64>, bool)>(
        r#"
        SELECT channel_id, enabled
        FROM bot_log_channels
        WHERE bot_id = $1 AND guild_id = $2 AND log_type = $3
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(log_type)
    .fetch_optional(&pool)
    .await
    .ok()
    .flatten()?;

    if !row.1 {
        return None;
    }

    row.0
        .and_then(|id| u64::try_from(id).ok().map(ChannelId::new))
}

async fn is_nolog_channel(
    ctx: &Context,
    guild_id: GuildId,
    channel_id: ChannelId,
    kind: &str,
) -> bool {
    let Some(pool) = pool(ctx).await else {
        return false;
    };
    let bot_id = ctx.cache.current_user().id;

    let row = sqlx::query_as::<_, (bool, bool)>(
        r#"
        SELECT disable_message, disable_voice
        FROM bot_nolog_channels
        WHERE bot_id = $1 AND guild_id = $2 AND channel_id = $3
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(channel_id.get() as i64)
    .fetch_optional(&pool)
    .await
    .ok()
    .flatten();

    let Some((disable_message, disable_voice)) = row else {
        return false;
    };

    match kind {
        "message" => disable_message,
        "voice" => disable_voice,
        _ => false,
    }
}

pub async fn send_log_embed(ctx: &Context, guild_id: GuildId, log_type: &str, embed: CreateEmbed) {
    if let Some(channel_id) = get_log_channel(ctx, guild_id, log_type).await {
        let _ = channel_id
            .send_message(
                &ctx.http,
                serenity::builder::CreateMessage::new().embed(embed),
            )
            .await;
    }
}

pub async fn emit_log(
    ctx: &Context,
    guild_id: GuildId,
    log_type: &str,
    user_id: Option<UserId>,
    channel_id: Option<ChannelId>,
    role_id: Option<RoleId>,
    action: &str,
    mut embed: CreateEmbed,
) {
    let bot_id = ctx.cache.current_user().id;
    let timestamp = Utc::now();

    embed = embed.timestamp(timestamp);

    if let Some(pool) = pool(ctx).await {
        let _ = crate::db::insert_audit_log(
            &pool, bot_id, guild_id, log_type, user_id, channel_id, role_id, None, action, None,
        )
        .await;
    }

    send_log_embed(ctx, guild_id, log_type, embed).await;
}

pub async fn handle_log_toggle(
    ctx: &Context,
    msg: &Message,
    args: &[&str],
    log_type: &str,
    label: &str,
) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(action) = args.first().map(|s| s.to_lowercase()) else {
        let embed = CreateEmbed::new()
            .title(label)
            .description(format!("Usage: +{} <on [salon]|off>", label.to_lowercase()))
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    match action.as_str() {
        "on" => {
            let channel = parse_target_channel(msg, args, 1);
            set_log_channel(ctx, guild_id, log_type, channel, true).await;
            let embed = CreateEmbed::new()
                .title(label)
                .description(format!(
                    "Activé dans {}.",
                    channel
                        .map(|c| format!("<#{}>", c.get()))
                        .unwrap_or_else(|| "ce salon".to_string())
                ))
                .color(theme_color(ctx).await);
            send_embed(ctx, msg, embed).await;
        }
        "off" => {
            set_log_channel(ctx, guild_id, log_type, None, false).await;
            let embed = CreateEmbed::new()
                .title(label)
                .description("Désactivé.")
                .color(theme_color(ctx).await);
            send_embed(ctx, msg, embed).await;
        }
        _ => {
            let embed = CreateEmbed::new()
                .title(label)
                .description(format!("Usage: +{} <on [salon]|off>", label.to_lowercase()))
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
        }
    }
}

pub async fn handle_raidlog(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    if args
        .first()
        .map(|a| a.eq_ignore_ascii_case("off"))
        .unwrap_or(false)
    {
        set_log_channel(ctx, guild_id, "raid", None, false).await;
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("RaidLog")
                .description("Désactivé.")
                .color(theme_color(ctx).await),
        )
        .await;
        return;
    }

    let channel = parse_target_channel(msg, args, 0);
    set_log_channel(ctx, guild_id, "raid", channel, true).await;
    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("RaidLog")
            .description(format!(
                "Activé dans {}.",
                channel
                    .map(|c| format!("<#{}>", c.get()))
                    .unwrap_or_else(|| "ce salon".to_string())
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_autoconfiglog(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let mut created = Vec::new();
    for (log_type, cmd) in LOG_TYPES {
        let name = format!("{}-logs", cmd.replace("log", ""));
        if let Ok(channel) = guild_id
            .create_channel(&ctx.http, CreateChannel::new(name).kind(ChannelType::Text))
            .await
        {
            set_log_channel(ctx, guild_id, log_type, Some(channel.id), true).await;
            created.push(format!("{} -> <#{}>", log_type, channel.id.get()));
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("AutoConfigLog")
            .description(if created.is_empty() {
                "Aucun salon créé.".to_string()
            } else {
                created.join("\n")
            })
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_set_modlogs(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let row = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT modlog_events
        FROM bot_log_settings
        WHERE bot_id = $1 AND guild_id = $2
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .fetch_optional(&pool)
    .await
    .ok()
    .flatten();

    let mut events = row
        .map(|(s,)| {
            s.split(',')
                .map(|v| v.trim().to_lowercase())
                .filter(|v| !v.is_empty())
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_else(|| {
            [
                "warn",
                "mute",
                "tempmute",
                "unmute",
                "cmute",
                "tempcmute",
                "uncmute",
                "kick",
                "ban",
                "tempban",
                "unban",
                "lock",
                "unlock",
                "hide",
                "unhide",
                "addrole",
                "delrole",
                "derank",
                "clear",
                "sanctions",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
        });

    if args.len() >= 2 {
        let event = args[0].to_lowercase();
        let state = args[1].to_lowercase();
        if state == "on" {
            events.insert(event);
        } else if state == "off" {
            events.remove(&event);
        }

        let serialized = events.iter().cloned().collect::<Vec<_>>().join(",");
        let _ = sqlx::query(
            r#"
            INSERT INTO bot_log_settings (bot_id, guild_id, modlog_events)
            VALUES ($1, $2, $3)
            ON CONFLICT (bot_id, guild_id)
            DO UPDATE SET modlog_events = EXCLUDED.modlog_events, updated_at = NOW();
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(serialized)
        .execute(&pool)
        .await;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Set ModLogs")
            .description(format!(
                "Événements actifs:\n{}\n\nUsage: +set modlogs <event> <on/off>",
                events.iter().cloned().collect::<Vec<_>>().join(", ")
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn handle_join_leave_settings(ctx: &Context, msg: &Message, args: &[&str], kind: &str) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    if args.is_empty() || !args[0].eq_ignore_ascii_case("settings") {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title(format!("{} settings", kind))
                .description(format!(
                    "Usage: +{} settings [on/off] [salon] [message...]",
                    kind
                ))
                .color(0xED4245),
        )
        .await;
        return;
    }

    if args.len() == 1 {
        let row = sqlx::query_as::<_, (bool, Option<i64>, Option<String>)>(
            r#"
            SELECT enabled, channel_id, custom_message
            FROM bot_join_leave_settings
            WHERE bot_id = $1 AND guild_id = $2 AND kind = $3
            LIMIT 1;
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(kind)
        .fetch_optional(&pool)
        .await
        .ok()
        .flatten();

        let desc = if let Some((enabled, channel_id, custom_message)) = row {
            format!(
                "État: {}\nSalon: {}\nMessage: {}",
                if enabled { "on" } else { "off" },
                channel_id
                    .map(|id| format!("<#{}>", id))
                    .unwrap_or_else(|| "non défini".to_string()),
                custom_message.unwrap_or_else(|| "(défaut)".to_string())
            )
        } else {
            "Aucun réglage configuré.".to_string()
        };

        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title(format!("{} settings", kind))
                .description(desc)
                .color(theme_color(ctx).await),
        )
        .await;
        return;
    }

    let action = args[1].to_lowercase();
    let enabled = action == "on";
    let channel = if enabled {
        parse_target_channel(msg, args, 2)
    } else {
        None
    };
    let message_start = if enabled { 3 } else { 2 };
    let custom_message = if args.len() > message_start {
        Some(args[message_start..].join(" "))
    } else {
        None
    };

    let _ = sqlx::query(
        r#"
        INSERT INTO bot_join_leave_settings (bot_id, guild_id, kind, enabled, channel_id, custom_message)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (bot_id, guild_id, kind)
        DO UPDATE SET enabled = EXCLUDED.enabled, channel_id = EXCLUDED.channel_id,
                      custom_message = EXCLUDED.custom_message, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(kind)
    .bind(enabled)
    .bind(channel.map(|c| c.get() as i64))
    .bind(custom_message)
    .execute(&pool)
    .await;

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title(format!("{} settings", kind))
            .description(format!(
                "{} {}",
                if enabled { "Activé" } else { "Désactivé" },
                channel
                    .map(|c| format!("dans <#{}>", c.get()))
                    .unwrap_or_default()
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn on_member_join(ctx: &Context, guild_id: GuildId, user: &User) {
    if let Some(channel_id) = get_log_channel(ctx, guild_id, "raid").await {
        let _ = channel_id
            .send_message(
                &ctx.http,
                serenity::builder::CreateMessage::new().embed(
                    CreateEmbed::new().title("RaidLog").description(format!(
                        "Nouveau membre: <@{}> (`{}`)",
                        user.id.get(),
                        user.tag()
                    )),
                ),
            )
            .await;
    }

    run_join_leave_action(ctx, guild_id, "join", user).await;
}

pub async fn on_member_leave(ctx: &Context, guild_id: GuildId, user: &User) {
    run_join_leave_action(ctx, guild_id, "leave", user).await;
}

async fn run_join_leave_action(ctx: &Context, guild_id: GuildId, kind: &str, user: &User) {
    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let row = sqlx::query_as::<_, (bool, Option<i64>, Option<String>)>(
        r#"
        SELECT enabled, channel_id, custom_message
        FROM bot_join_leave_settings
        WHERE bot_id = $1 AND guild_id = $2 AND kind = $3
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(kind)
    .fetch_optional(&pool)
    .await
    .ok()
    .flatten();

    let Some((enabled, channel_id, custom_message)) = row else {
        return;
    };
    if !enabled {
        return;
    }

    let channel_id = channel_id
        .and_then(|id| u64::try_from(id).ok().map(ChannelId::new))
        .unwrap_or_else(|| ChannelId::new(guild_id.get()));

    let content = custom_message.unwrap_or_else(|| {
        if kind == "join" {
            format!("Bienvenue <@{}> !", user.id.get())
        } else {
            format!("<@{}> a quitté le serveur.", user.id.get())
        }
    });

    let _ = channel_id.say(&ctx.http, content).await;
}

pub async fn handle_set_boostembed(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.len() < 2 {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Set BoostEmbed")
                .description("Usage: +set boostembed <title|description|color> <valeur>")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let field = args[0].to_lowercase();
    let value = args[1..].join(" ");
    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let _ = sqlx::query(
        r#"
        INSERT INTO bot_boost_embed (bot_id, guild_id, enabled, title, description, color)
        VALUES ($1, $2, TRUE, NULL, NULL, NULL)
        ON CONFLICT (bot_id, guild_id)
        DO NOTHING;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .execute(&pool)
    .await;

    match field.as_str() {
        "title" => {
            let _ = sqlx::query(
                "UPDATE bot_boost_embed SET title = $3, updated_at = NOW() WHERE bot_id = $1 AND guild_id = $2",
            )
            .bind(bot_id.get() as i64)
            .bind(guild_id.get() as i64)
            .bind(value)
            .execute(&pool)
            .await;
        }
        "description" => {
            let _ = sqlx::query(
                "UPDATE bot_boost_embed SET description = $3, updated_at = NOW() WHERE bot_id = $1 AND guild_id = $2",
            )
            .bind(bot_id.get() as i64)
            .bind(guild_id.get() as i64)
            .bind(value)
            .execute(&pool)
            .await;
        }
        "color" => {
            let normalized = value
                .trim()
                .trim_start_matches('#')
                .trim_start_matches("0x");
            if let Ok(color) = u32::from_str_radix(normalized, 16) {
                let _ = sqlx::query(
                    "UPDATE bot_boost_embed SET color = $3, updated_at = NOW() WHERE bot_id = $1 AND guild_id = $2",
                )
                .bind(bot_id.get() as i64)
                .bind(guild_id.get() as i64)
                .bind(color as i32)
                .execute(&pool)
                .await;
            }
        }
        _ => {}
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Set BoostEmbed")
            .description("Configuration mise à jour.")
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn send_boost_embed(ctx: &Context, guild_id: GuildId, user: &User) {
    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let row = sqlx::query_as::<_, (bool, Option<String>, Option<String>, Option<i32>)>(
        r#"
        SELECT enabled, title, description, color
        FROM bot_boost_embed
        WHERE bot_id = $1 AND guild_id = $2
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .fetch_optional(&pool)
    .await
    .ok()
    .flatten();

    let enabled = row.as_ref().map(|r| r.0).unwrap_or(true);
    if !enabled {
        return;
    }

    let title = row
        .as_ref()
        .and_then(|r| r.1.clone())
        .unwrap_or_else(|| "Nouveau boost".to_string());
    let description = row
        .as_ref()
        .and_then(|r| r.2.clone())
        .unwrap_or_else(|| format!("<@{}> vient de booster le serveur !", user.id.get()));
    let color = row
        .as_ref()
        .and_then(|r| r.3)
        .map(|c| c.max(0) as u32)
        .unwrap_or(0xF47FFF);

    let embed = CreateEmbed::new()
        .title(title)
        .description(description)
        .color(color);

    send_log_embed(ctx, guild_id, "boost", embed).await;
}

pub async fn handle_nolog(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.is_empty() {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("NoLog")
                .description("Usage: +nolog <add/del> [salon] [message|voice|all]")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let action = args[0].to_lowercase();
    let channel = parse_target_channel(msg, args, 1).unwrap_or(msg.channel_id);
    let scope = args
        .get(2)
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "all".to_string());

    let set_message = scope == "all" || scope == "message";
    let set_voice = scope == "all" || scope == "voice";

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    if action == "add" {
        let _ = sqlx::query(
            r#"
            INSERT INTO bot_nolog_channels (bot_id, guild_id, channel_id, disable_message, disable_voice)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (bot_id, guild_id, channel_id)
            DO UPDATE SET disable_message = bot_nolog_channels.disable_message OR EXCLUDED.disable_message,
                          disable_voice = bot_nolog_channels.disable_voice OR EXCLUDED.disable_voice,
                          updated_at = NOW();
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(channel.get() as i64)
        .bind(set_message)
        .bind(set_voice)
        .execute(&pool)
        .await;
    } else if action == "del" {
        let _ = sqlx::query(
            r#"
            UPDATE bot_nolog_channels
            SET disable_message = CASE WHEN $4 THEN FALSE ELSE disable_message END,
                disable_voice = CASE WHEN $5 THEN FALSE ELSE disable_voice END,
                updated_at = NOW()
            WHERE bot_id = $1 AND guild_id = $2 AND channel_id = $3;
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(channel.get() as i64)
        .bind(set_message)
        .bind(set_voice)
        .execute(&pool)
        .await;

        let _ = sqlx::query(
            r#"
            DELETE FROM bot_nolog_channels
            WHERE bot_id = $1 AND guild_id = $2 AND channel_id = $3
              AND disable_message = FALSE AND disable_voice = FALSE;
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(channel.get() as i64)
        .execute(&pool)
        .await;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("NoLog")
            .description(format!(
                "{} appliqué sur <#{}> ({})",
                action,
                channel.get(),
                scope
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn on_message_deleted(
    ctx: &Context,
    guild_id: Option<GuildId>,
    channel_id: ChannelId,
    message_id: MessageId,
    author_id: Option<UserId>,
    content: Option<String>,
) {
    let Some(guild_id) = guild_id else {
        return;
    };
    if is_nolog_channel(ctx, guild_id, channel_id, "message").await {
        return;
    }

    let embed = CreateEmbed::new()
        .title("Message supprimé")
        .description(format!(
            "Salon: <#{}>\nAuteur: {}\nMessage: `{}`\nContenu: {}",
            channel_id.get(),
            author_id
                .map(|id| format!("<@{}>", id.get()))
                .unwrap_or_else(|| "inconnu".to_string()),
            message_id.get(),
            content.unwrap_or_else(|| "(indisponible)".to_string())
        ));
    send_log_embed(ctx, guild_id, "message", embed).await;
}

pub async fn on_message_edited(
    ctx: &Context,
    guild_id: Option<GuildId>,
    channel_id: ChannelId,
    author_id: Option<UserId>,
    before: Option<String>,
    after: Option<String>,
) {
    let Some(guild_id) = guild_id else {
        return;
    };
    if is_nolog_channel(ctx, guild_id, channel_id, "message").await {
        return;
    }

    let embed = CreateEmbed::new()
        .title("Message édité")
        .description(format!(
            "Salon: <#{}>\nAuteur: {}\nAvant: {}\nAprès: {}",
            channel_id.get(),
            author_id
                .map(|id| format!("<@{}>", id.get()))
                .unwrap_or_else(|| "inconnu".to_string()),
            before.unwrap_or_else(|| "(indisponible)".to_string()),
            after.unwrap_or_else(|| "(indisponible)".to_string())
        ));

    send_log_embed(ctx, guild_id, "message", embed).await;
}

pub async fn on_voice_update(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
    old_channel: Option<ChannelId>,
    new_channel: Option<ChannelId>,
) {
    if let Some(ch) = new_channel.or(old_channel) {
        if is_nolog_channel(ctx, guild_id, ch, "voice").await {
            return;
        }
    }

    let action = match (old_channel, new_channel) {
        (None, Some(to)) => format!("<@{}> a rejoint <#{}>", user_id.get(), to.get()),
        (Some(from), None) => format!("<@{}> a quitté <#{}>", user_id.get(), from.get()),
        (Some(from), Some(to)) => format!(
            "<@{}> a bougé de <#{}> vers <#{}>",
            user_id.get(),
            from.get(),
            to.get()
        ),
        _ => return,
    };

    send_log_embed(
        ctx,
        guild_id,
        "voice",
        CreateEmbed::new().title("VoiceLog").description(action),
    )
    .await;
}

pub async fn on_role_create(ctx: &Context, guild_id: GuildId, role: &Role) {
    send_log_embed(
        ctx,
        guild_id,
        "role",
        CreateEmbed::new().title("Role créé").description(format!(
            "<@&{}> (`{}`)",
            role.id.get(),
            role.name
        )),
    )
    .await;
}

pub async fn on_role_update(
    ctx: &Context,
    guild_id: GuildId,
    old_role: Option<&Role>,
    new_role: &Role,
) {
    let desc = if let Some(old) = old_role {
        format!(
            "`{}` -> `{}`\nID: <@&{}>",
            old.name,
            new_role.name,
            new_role.id.get()
        )
    } else {
        format!("Role mis à jour: <@&{}>", new_role.id.get())
    };

    send_log_embed(
        ctx,
        guild_id,
        "role",
        CreateEmbed::new().title("Role modifié").description(desc),
    )
    .await;
}

pub async fn on_role_delete(
    ctx: &Context,
    guild_id: GuildId,
    role_id: RoleId,
    role: Option<&Role>,
) {
    let desc = role
        .map(|r| format!("`{}` (`{}`)", r.name, role_id.get()))
        .unwrap_or_else(|| format!("ID `{}`", role_id.get()));
    send_log_embed(
        ctx,
        guild_id,
        "role",
        CreateEmbed::new().title("Role supprimé").description(desc),
    )
    .await;
}

pub async fn on_member_roles_updated(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
    old_roles: &[RoleId],
    new_roles: &[RoleId],
) {
    let old_set = old_roles.iter().copied().collect::<BTreeSet<_>>();
    let new_set = new_roles.iter().copied().collect::<BTreeSet<_>>();

    let added = new_set
        .difference(&old_set)
        .map(|r| format!("<@&{}>", r.get()))
        .collect::<Vec<_>>();
    let removed = old_set
        .difference(&new_set)
        .map(|r| format!("<@&{}>", r.get()))
        .collect::<Vec<_>>();

    if added.is_empty() && removed.is_empty() {
        return;
    }

    let desc = format!(
        "Membre: <@{}>\nAjoutés: {}\nRetirés: {}",
        user_id.get(),
        if added.is_empty() {
            "aucun".to_string()
        } else {
            added.join(", ")
        },
        if removed.is_empty() {
            "aucun".to_string()
        } else {
            removed.join(", ")
        }
    );

    send_log_embed(
        ctx,
        guild_id,
        "role",
        CreateEmbed::new().title("RoleLog membre").description(desc),
    )
    .await;
}

pub async fn on_boost_update(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
    old_boost: Option<Timestamp>,
    new_boost: Option<Timestamp>,
) {
    match (old_boost, new_boost) {
        (None, Some(_)) => {
            send_log_embed(
                ctx,
                guild_id,
                "boost",
                CreateEmbed::new()
                    .title("Nouveau boost")
                    .description(format!("<@{}> a boost le serveur.", user_id.get())),
            )
            .await;

            if let Ok(user) = ctx.http.get_user(user_id).await {
                send_boost_embed(ctx, guild_id, &user).await;
            }
        }
        (Some(_), None) => {
            send_log_embed(
                ctx,
                guild_id,
                "boost",
                CreateEmbed::new()
                    .title("Boost retiré")
                    .description(format!("<@{}> ne boost plus le serveur.", user_id.get())),
            )
            .await;
        }
        _ => {}
    }
}

pub async fn log_moderation_command(ctx: &Context, msg: &Message, command: &str, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let enabled = is_modlog_event_enabled(ctx, guild_id, command).await;
    if !enabled {
        return;
    }

    let content = if args.is_empty() {
        command.to_string()
    } else {
        format!("{} {}", command, args.join(" "))
    };

    send_log_embed(
        ctx,
        guild_id,
        "moderation",
        CreateEmbed::new().title("ModLog").description(format!(
            "Modérateur: <@{}>\nCommande: `+{}`",
            msg.author.id.get(),
            content
        )),
    )
    .await;
}

async fn is_modlog_event_enabled(ctx: &Context, guild_id: GuildId, event: &str) -> bool {
    let Some(pool) = pool(ctx).await else {
        return true;
    };
    let bot_id = ctx.cache.current_user().id;

    let row = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT modlog_events
        FROM bot_log_settings
        WHERE bot_id = $1 AND guild_id = $2
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .fetch_optional(&pool)
    .await
    .ok()
    .flatten();

    let Some((events,)) = row else {
        return true;
    };

    let set = events
        .split(',')
        .map(|v| v.trim().to_lowercase())
        .filter(|v| !v.is_empty())
        .collect::<BTreeSet<_>>();

    set.contains(&event.to_lowercase())
}

pub async fn on_channel_create(ctx: &Context, channel: &GuildChannel) {
    emit_log(
        ctx,
        channel.guild_id,
        "channel",
        None,
        Some(channel.id),
        None,
        "créé",
        CreateEmbed::new()
            .title("Channel Créé")
            .description(format!(
                "Salon: <#{}> \nNom: {} \nType: {}",
                channel.id.get(),
                channel.name,
                match channel.kind {
                    ChannelType::Text => "Texte",
                    ChannelType::Voice => "Vocal",
                    ChannelType::Category => "Catégorie",
                    _ => "Autre",
                }
            )),
    )
    .await;
}

pub async fn on_channel_delete(ctx: &Context, channel: &GuildChannel) {
    emit_log(
        ctx,
        channel.guild_id,
        "channel",
        None,
        Some(channel.id),
        None,
        "supprimé",
        CreateEmbed::new()
            .title("Channel Supprimé")
            .description(format!(
                "Salon: {}\nNom: {}\nType: {}",
                channel.id.get(),
                channel.name,
                match channel.kind {
                    ChannelType::Text => "Texte",
                    ChannelType::Voice => "Vocal",
                    ChannelType::Category => "Catégorie",
                    _ => "Autre",
                }
            )),
    )
    .await;
}

pub async fn on_channel_update(ctx: &Context, old_data: Option<GuildChannel>, new: &GuildChannel) {
    let mut changes = Vec::new();

    if let Some(old) = old_data {
        if old.name != new.name {
            changes.push(format!("Nom: `{}` → `{}`", old.name, new.name));
        }
        if old.topic != new.topic {
            changes.push(format!(
                "Sujet: `{}` → `{}`",
                old.topic.as_deref().unwrap_or("(vide)"),
                new.topic.as_deref().unwrap_or("(vide)")
            ));
        }
        if old.nsfw != new.nsfw {
            changes.push(format!("NSFW: {} → {}", old.nsfw, new.nsfw));
        }
    }

    if changes.is_empty() {
        return;
    }

    emit_log(
        ctx,
        new.guild_id,
        "channel",
        None,
        Some(new.id),
        None,
        "modifié",
        CreateEmbed::new()
            .title("Channel Modifié")
            .description(format!(
                "Salon: <#{}>\n{}",
                new.id.get(),
                changes.join("\n")
            )),
    )
    .await;
}
