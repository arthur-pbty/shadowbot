use std::collections::BTreeSet;

use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::db::DbPoolKey;

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
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

async fn record_audit_log(
    ctx: &Context,
    guild_id: GuildId,
    log_type: &str,
    user_id: Option<UserId>,
    channel_id: Option<ChannelId>,
    role_id: Option<RoleId>,
    action: &str,
) {
    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id;
    let _ = crate::db::insert_audit_log(
        &pool, bot_id, guild_id, log_type, user_id, channel_id, role_id, None, action, None,
    )
    .await;
}

pub async fn send_log_embed(ctx: &Context, guild_id: GuildId, log_type: &str, embed: CreateEmbed) {
    record_audit_log(ctx, guild_id, log_type, None, None, None, log_type).await;

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
    let timestamp = Utc::now();

    embed = embed.timestamp(timestamp);

    record_audit_log(
        ctx, guild_id, log_type, user_id, channel_id, role_id, action,
    )
    .await;

    if let Some(log_channel_id) = get_log_channel(ctx, guild_id, log_type).await {
        let _ = log_channel_id
            .send_message(
                &ctx.http,
                serenity::builder::CreateMessage::new().embed(embed),
            )
            .await;
    }
}

pub async fn on_member_join(ctx: &Context, guild_id: GuildId, user: &User) {
    emit_log(
        ctx,
        guild_id,
        "raid",
        Some(user.id),
        None,
        None,
        "join",
        CreateEmbed::new().title("RaidLog").description(format!(
            "Nouveau membre: <@{}> (`{}`)",
            user.id.get(),
            user.tag()
        )),
    )
    .await;

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

    if let Some(channel_id) = get_log_channel(ctx, guild_id, "boost").await {
        let _ = channel_id
            .send_message(
                &ctx.http,
                serenity::builder::CreateMessage::new().embed(embed),
            )
            .await;
    }
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

    emit_log(
        ctx,
        guild_id,
        "moderation",
        Some(msg.author.id),
        Some(msg.channel_id),
        None,
        command,
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
