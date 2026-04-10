use std::collections::BTreeSet;

use chrono::Utc;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
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

fn color_for_log_type(log_type: &str) -> u32 {
    match log_type {
        "message" => 0xF4C430,
        "voice" => 0x3BA55D,
        "role" => 0x5865F2,
        "channel" => 0x2D7D9A,
        "moderation" => 0xED4245,
        "raid" => 0xF04747,
        "boost" => 0xFF73FA,
        _ => 0x99AAB5,
    }
}

fn truncate_for_embed(value: &str, max_chars: usize) -> String {
    let mut out = String::new();
    let mut count = 0usize;

    for ch in value.chars() {
        if count >= max_chars {
            out.push_str("...");
            break;
        }
        out.push(ch);
        count += 1;
    }

    if out.trim().is_empty() {
        "(vide)".to_string()
    } else {
        out
    }
}

fn enrich_log_embed(
    ctx: &Context,
    guild_id: GuildId,
    log_type: &str,
    action: Option<&str>,
    user_id: Option<UserId>,
    channel_id: Option<ChannelId>,
    role_id: Option<RoleId>,
    log_channel_id: Option<ChannelId>,
    embed: CreateEmbed,
) -> CreateEmbed {
    let now = Utc::now();
    let guild_name = ctx
        .cache
        .guild(guild_id)
        .map(|guild| guild.name.clone())
        .unwrap_or_else(|| "Serveur inconnu".to_string());

    let mut context_lines = Vec::new();
    context_lines.push(format!("Type: `{}`", log_type));
    if let Some(action) = action {
        context_lines.push(format!("Action: `{}`", action));
    }
    context_lines.push(format!("Serveur: `{}` (`{}`)", guild_name, guild_id.get()));
    context_lines.push(format!(
        "Canal de log: {}",
        log_channel_id
            .map(|id| format!("<#{}> (`{}`)", id.get(), id.get()))
            .unwrap_or_else(|| "non configure".to_string())
    ));
    if let Some(channel_id) = channel_id {
        context_lines.push(format!(
            "Canal cible: <#{}> (`{}`)",
            channel_id.get(),
            channel_id.get()
        ));
    }
    if let Some(user_id) = user_id {
        context_lines.push(format!(
            "Utilisateur cible: <@{}> (`{}`)",
            user_id.get(),
            user_id.get()
        ));
    }
    if let Some(role_id) = role_id {
        context_lines.push(format!(
            "Role cible: <@&{}> (`{}`)",
            role_id.get(),
            role_id.get()
        ));
    }
    context_lines.push(format!(
        "Horodatage: <t:{}:F> (<t:{}:R>)",
        now.timestamp(),
        now.timestamp()
    ));

    embed
        .color(color_for_log_type(log_type))
        .timestamp(now)
        .field("Contexte", context_lines.join("\n"), false)
        .footer(CreateEmbedFooter::new(format!(
            "{} • logs {}",
            guild_name, log_type
        )))
}

pub async fn send_log_embed(ctx: &Context, guild_id: GuildId, log_type: &str, embed: CreateEmbed) {
    record_audit_log(ctx, guild_id, log_type, None, None, None, log_type).await;

    let log_channel_id = get_log_channel(ctx, guild_id, log_type).await;

    if let Some(channel_id) = log_channel_id {
        let embed = enrich_log_embed(
            ctx,
            guild_id,
            log_type,
            None,
            None,
            None,
            None,
            Some(channel_id),
            embed,
        );

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
    embed: CreateEmbed,
) {
    record_audit_log(
        ctx, guild_id, log_type, user_id, channel_id, role_id, action,
    )
    .await;

    if let Some(log_channel_id) = get_log_channel(ctx, guild_id, log_type).await {
        let embed = enrich_log_embed(
            ctx,
            guild_id,
            log_type,
            Some(action),
            user_id,
            channel_id,
            role_id,
            Some(log_channel_id),
            embed,
        );

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
        let embed = enrich_log_embed(
            ctx,
            guild_id,
            "boost",
            Some("boost_custom_embed"),
            Some(user.id),
            None,
            None,
            Some(channel_id),
            embed,
        );

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

    let author = author_id
        .map(|id| format!("<@{}> (`{}`)", id.get(), id.get()))
        .unwrap_or_else(|| "inconnu".to_string());
    let content_value = content.unwrap_or_else(|| "(indisponible)".to_string());
    let message_url = format!(
        "https://discord.com/channels/{}/{}/{}",
        guild_id.get(),
        channel_id.get(),
        message_id.get()
    );

    let embed = CreateEmbed::new()
        .title("Message supprime")
        .description("Un message a ete supprime.")
        .field("Auteur", author, true)
        .field(
            "Salon",
            format!("<#{}>\n`{}`", channel_id.get(), channel_id.get()),
            true,
        )
        .field("Message ID", format!("`{}`", message_id.get()), true)
        .field("Lien", format!("[Acces rapide]({})", message_url), false)
        .field("Contenu", truncate_for_embed(&content_value, 900), false);

    emit_log(
        ctx,
        guild_id,
        "message",
        author_id,
        Some(channel_id),
        None,
        "message_delete",
        embed,
    )
    .await;
}

pub async fn on_message_edited(
    ctx: &Context,
    guild_id: Option<GuildId>,
    channel_id: ChannelId,
    message_id: MessageId,
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

    let author = author_id
        .map(|id| format!("<@{}> (`{}`)", id.get(), id.get()))
        .unwrap_or_else(|| "inconnu".to_string());
    let before_value = before.unwrap_or_else(|| "(indisponible)".to_string());
    let after_value = after.unwrap_or_else(|| "(indisponible)".to_string());
    let message_url = format!(
        "https://discord.com/channels/{}/{}/{}",
        guild_id.get(),
        channel_id.get(),
        message_id.get()
    );

    let embed = CreateEmbed::new()
        .title("Message edite")
        .description("Un message a ete modifie.")
        .field("Auteur", author, true)
        .field(
            "Salon",
            format!("<#{}>\n`{}`", channel_id.get(), channel_id.get()),
            true,
        )
        .field("Message ID", format!("`{}`", message_id.get()), true)
        .field("Lien", format!("[Acces rapide]({})", message_url), false)
        .field("Avant", truncate_for_embed(&before_value, 900), false)
        .field("Apres", truncate_for_embed(&after_value, 900), false);

    emit_log(
        ctx,
        guild_id,
        "message",
        author_id,
        Some(channel_id),
        None,
        "message_update",
        embed,
    )
    .await;
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

async fn resolve_guild_id_from_channel(ctx: &Context, channel_id: ChannelId) -> Option<GuildId> {
    let channel = channel_id.to_channel(&ctx.http).await.ok()?;

    match channel {
        serenity::model::channel::Channel::Guild(guild_channel) => Some(guild_channel.guild_id),
        _ => None,
    }
}

pub async fn on_channel_pins_update(ctx: &Context, pin: &ChannelPinsUpdateEvent) {
    let guild_id = if let Some(guild_id) = pin.guild_id {
        guild_id
    } else if let Some(guild_id) = resolve_guild_id_from_channel(ctx, pin.channel_id).await {
        guild_id
    } else {
        return;
    };

    if is_nolog_channel(ctx, guild_id, pin.channel_id, "message").await {
        return;
    }

    let last_pin_timestamp = pin
        .last_pin_timestamp
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_else(|| "Aucun pin actif".to_string());

    send_log_embed(
        ctx,
        guild_id,
        "message",
        CreateEmbed::new()
            .title("Pins du salon mis a jour")
            .description(format!(
                "Salon: <#{}>\nDernier pin: {}",
                pin.channel_id.get(),
                last_pin_timestamp
            )),
    )
    .await;
}

pub async fn on_guild_ban_addition(ctx: &Context, guild_id: GuildId, banned_user: &User) {
    emit_log(
        ctx,
        guild_id,
        "moderation",
        Some(banned_user.id),
        None,
        None,
        "ban_event",
        CreateEmbed::new().title("Ban detecte").description(format!(
            "Utilisateur: <@{}> (`{}`)",
            banned_user.id.get(),
            banned_user.tag()
        )),
    )
    .await;
}

pub async fn on_guild_ban_removal(ctx: &Context, guild_id: GuildId, unbanned_user: &User) {
    emit_log(
        ctx,
        guild_id,
        "moderation",
        Some(unbanned_user.id),
        None,
        None,
        "unban_event",
        CreateEmbed::new()
            .title("Deban detecte")
            .description(format!(
                "Utilisateur: <@{}> (`{}`)",
                unbanned_user.id.get(),
                unbanned_user.tag()
            )),
    )
    .await;
}

pub async fn on_invite_create(ctx: &Context, data: &InviteCreateEvent) {
    let Some(guild_id) = data.guild_id else {
        return;
    };

    let inviter = data
        .inviter
        .as_ref()
        .map(|user| format!("<@{}>", user.id.get()))
        .unwrap_or_else(|| "Inconnu".to_string());

    emit_log(
        ctx,
        guild_id,
        "channel",
        data.inviter.as_ref().map(|user| user.id),
        Some(data.channel_id),
        None,
        "invite_create",
        CreateEmbed::new()
            .title("Invitation creee")
            .description(format!(
                "Code: `{}`\nSalon: <#{}>\nInviteur: {}\nTemporaire: {}\nMax utilisations: {}",
                data.code,
                data.channel_id.get(),
                inviter,
                data.temporary,
                data.max_uses
            )),
    )
    .await;
}

pub async fn on_invite_delete(ctx: &Context, data: &InviteDeleteEvent) {
    let Some(guild_id) = data.guild_id else {
        return;
    };

    emit_log(
        ctx,
        guild_id,
        "channel",
        None,
        Some(data.channel_id),
        None,
        "invite_delete",
        CreateEmbed::new()
            .title("Invitation supprimee")
            .description(format!(
                "Code: `{}`\nSalon: <#{}>",
                data.code,
                data.channel_id.get()
            )),
    )
    .await;
}

pub async fn on_message_delete_bulk(
    ctx: &Context,
    channel_id: ChannelId,
    multiple_deleted_messages_ids: &[MessageId],
    guild_id: Option<GuildId>,
) {
    let Some(guild_id) = guild_id else {
        return;
    };
    if is_nolog_channel(ctx, guild_id, channel_id, "message").await {
        return;
    }

    let ids_preview = multiple_deleted_messages_ids
        .iter()
        .take(10)
        .map(|id| format!("`{}`", id.get()))
        .collect::<Vec<_>>();
    let hidden = multiple_deleted_messages_ids
        .len()
        .saturating_sub(ids_preview.len());

    let mut description = format!(
        "Salon: <#{}>\nTotal supprimes: {}",
        channel_id.get(),
        multiple_deleted_messages_ids.len()
    );

    if !ids_preview.is_empty() {
        description.push_str(&format!("\nExemple IDs: {}", ids_preview.join(", ")));
    }
    if hidden > 0 {
        description.push_str(&format!("\n... et {} autres", hidden));
    }

    emit_log(
        ctx,
        guild_id,
        "message",
        None,
        Some(channel_id),
        None,
        "message_delete_bulk",
        CreateEmbed::new()
            .title("Suppression en masse")
            .description(description),
    )
    .await;
}

pub async fn on_reaction_add(ctx: &Context, reaction: &Reaction) {
    let Some(guild_id) = reaction.guild_id else {
        return;
    };
    if is_nolog_channel(ctx, guild_id, reaction.channel_id, "message").await {
        return;
    }

    let user = reaction
        .user_id
        .map(|id| format!("<@{}>", id.get()))
        .unwrap_or_else(|| "Inconnu".to_string());

    emit_log(
        ctx,
        guild_id,
        "message",
        reaction.user_id,
        Some(reaction.channel_id),
        None,
        "reaction_add",
        CreateEmbed::new()
            .title("Reaction ajoutee")
            .description(format!(
                "Salon: <#{}>\nMessage: `{}`\nUtilisateur: {}\nEmoji: `{:?}`",
                reaction.channel_id.get(),
                reaction.message_id.get(),
                user,
                reaction.emoji
            )),
    )
    .await;
}

pub async fn on_reaction_remove(ctx: &Context, reaction: &Reaction) {
    let Some(guild_id) = reaction.guild_id else {
        return;
    };
    if is_nolog_channel(ctx, guild_id, reaction.channel_id, "message").await {
        return;
    }

    let user = reaction
        .user_id
        .map(|id| format!("<@{}>", id.get()))
        .unwrap_or_else(|| "Inconnu".to_string());

    emit_log(
        ctx,
        guild_id,
        "message",
        reaction.user_id,
        Some(reaction.channel_id),
        None,
        "reaction_remove",
        CreateEmbed::new()
            .title("Reaction retiree")
            .description(format!(
                "Salon: <#{}>\nMessage: `{}`\nUtilisateur: {}\nEmoji: `{:?}`",
                reaction.channel_id.get(),
                reaction.message_id.get(),
                user,
                reaction.emoji
            )),
    )
    .await;
}

pub async fn on_reaction_remove_all(
    ctx: &Context,
    channel_id: ChannelId,
    removed_from_message_id: MessageId,
) {
    let Some(guild_id) = resolve_guild_id_from_channel(ctx, channel_id).await else {
        return;
    };
    if is_nolog_channel(ctx, guild_id, channel_id, "message").await {
        return;
    }

    emit_log(
        ctx,
        guild_id,
        "message",
        None,
        Some(channel_id),
        None,
        "reaction_remove_all",
        CreateEmbed::new()
            .title("Toutes les reactions retirees")
            .description(format!(
                "Salon: <#{}>\nMessage: `{}`",
                channel_id.get(),
                removed_from_message_id.get()
            )),
    )
    .await;
}

pub async fn on_reaction_remove_emoji(ctx: &Context, removed_reactions: &Reaction) {
    let Some(guild_id) = removed_reactions.guild_id else {
        return;
    };
    if is_nolog_channel(ctx, guild_id, removed_reactions.channel_id, "message").await {
        return;
    }

    emit_log(
        ctx,
        guild_id,
        "message",
        removed_reactions.user_id,
        Some(removed_reactions.channel_id),
        None,
        "reaction_remove_emoji",
        CreateEmbed::new()
            .title("Emoji de reaction retire")
            .description(format!(
                "Salon: <#{}>\nMessage: `{}`\nEmoji: `{:?}`",
                removed_reactions.channel_id.get(),
                removed_reactions.message_id.get(),
                removed_reactions.emoji
            )),
    )
    .await;
}

pub async fn on_webhook_update(ctx: &Context, guild_id: GuildId, belongs_to_channel_id: ChannelId) {
    emit_log(
        ctx,
        guild_id,
        "channel",
        None,
        Some(belongs_to_channel_id),
        None,
        "webhook_update",
        CreateEmbed::new()
            .title("Webhook mis a jour")
            .description(format!("Salon: <#{}>", belongs_to_channel_id.get())),
    )
    .await;
}

pub async fn on_thread_create(ctx: &Context, thread: &GuildChannel) {
    emit_log(
        ctx,
        thread.guild_id,
        "channel",
        thread.owner_id,
        Some(thread.id),
        None,
        "thread_create",
        CreateEmbed::new().title("Thread cree").description(format!(
            "Thread: <#{}>\nNom: `{}`\nParent: {}",
            thread.id.get(),
            thread.name,
            thread
                .parent_id
                .map(|id| format!("<#{}>", id.get()))
                .unwrap_or_else(|| "Aucun".to_string())
        )),
    )
    .await;
}

pub async fn on_thread_update(
    ctx: &Context,
    old_thread: Option<GuildChannel>,
    new_thread: &GuildChannel,
) {
    let mut changes = Vec::new();

    if let Some(old_thread) = old_thread {
        if old_thread.name != new_thread.name {
            changes.push(format!(
                "Nom: `{}` -> `{}`",
                old_thread.name, new_thread.name
            ));
        }

        let old_archived = old_thread.thread_metadata.map(|meta| meta.archived);
        let new_archived = new_thread.thread_metadata.map(|meta| meta.archived);
        if old_archived != new_archived {
            changes.push(format!(
                "Archive: {} -> {}",
                old_archived
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "inconnu".to_string()),
                new_archived
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "inconnu".to_string())
            ));
        }

        if old_thread.rate_limit_per_user != new_thread.rate_limit_per_user {
            changes.push(format!(
                "Slowmode: {:?} -> {:?}",
                old_thread.rate_limit_per_user, new_thread.rate_limit_per_user
            ));
        }
    }

    if changes.is_empty() {
        changes.push("Mise a jour detectee (details indisponibles).".to_string());
    }

    emit_log(
        ctx,
        new_thread.guild_id,
        "channel",
        new_thread.owner_id,
        Some(new_thread.id),
        None,
        "thread_update",
        CreateEmbed::new()
            .title("Thread mis a jour")
            .description(format!(
                "Thread: <#{}>\n{}",
                new_thread.id.get(),
                changes.join("\n")
            )),
    )
    .await;
}

pub async fn on_thread_delete(
    ctx: &Context,
    thread: &PartialGuildChannel,
    full_thread_data: Option<&GuildChannel>,
) {
    let name_or_id = full_thread_data
        .map(|thread_data| format!("`{}`", thread_data.name))
        .unwrap_or_else(|| format!("ID `{}`", thread.id.get()));

    emit_log(
        ctx,
        thread.guild_id,
        "channel",
        None,
        Some(thread.id),
        None,
        "thread_delete",
        CreateEmbed::new()
            .title("Thread supprime")
            .description(format!("Thread: {}", name_or_id)),
    )
    .await;
}

pub async fn on_thread_list_sync(ctx: &Context, thread_list_sync: &ThreadListSyncEvent) {
    let parents = thread_list_sync
        .channel_ids
        .as_ref()
        .map(|ids| {
            ids.iter()
                .map(|id| format!("<#{}>", id.get()))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_else(|| "Tous les salons".to_string());

    send_log_embed(
        ctx,
        thread_list_sync.guild_id,
        "channel",
        CreateEmbed::new().title("Thread sync").description(format!(
            "Parents: {}\nThreads sync: {}\nMembres renvoyes: {}",
            parents,
            thread_list_sync.threads.len(),
            thread_list_sync.members.len()
        )),
    )
    .await;
}

pub async fn on_thread_member_update(ctx: &Context, thread_member: &ThreadMember) {
    let Some(guild_id) = thread_member.guild_id else {
        return;
    };

    emit_log(
        ctx,
        guild_id,
        "channel",
        Some(thread_member.user_id),
        Some(thread_member.id),
        None,
        "thread_member_update",
        CreateEmbed::new()
            .title("Membre de thread mis a jour")
            .description(format!(
                "Thread: <#{}>\nUtilisateur: <@{}>",
                thread_member.id.get(),
                thread_member.user_id.get()
            )),
    )
    .await;
}

pub async fn on_thread_members_update(ctx: &Context, event: &ThreadMembersUpdateEvent) {
    let added = event
        .added_members
        .iter()
        .map(|member| format!("<@{}>", member.user_id.get()))
        .take(10)
        .collect::<Vec<_>>();
    let removed = event
        .removed_member_ids
        .iter()
        .map(|user_id| format!("<@{}>", user_id.get()))
        .take(10)
        .collect::<Vec<_>>();

    send_log_embed(
        ctx,
        event.guild_id,
        "channel",
        CreateEmbed::new()
            .title("Membres du thread mis a jour")
            .description(format!(
                "Thread: <#{}>\nMembres approx: {}\nAjoutes: {}\nRetires: {}",
                event.id.get(),
                event.member_count,
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
            )),
    )
    .await;
}

pub async fn on_auto_moderation_rule_create(ctx: &Context, rule: &Rule) {
    emit_log(
        ctx,
        rule.guild_id,
        "moderation",
        Some(rule.creator_id),
        None,
        None,
        "automod_rule_create",
        CreateEmbed::new()
            .title("AutoMod: regle creee")
            .description(format!(
                "Nom: `{}`\nID: `{}`\nActive: {}\nTrigger: `{:?}`\nActions: {}",
                rule.name,
                rule.id.get(),
                rule.enabled,
                rule.trigger.kind(),
                rule.actions.len()
            )),
    )
    .await;
}

pub async fn on_auto_moderation_rule_update(ctx: &Context, rule: &Rule) {
    emit_log(
        ctx,
        rule.guild_id,
        "moderation",
        Some(rule.creator_id),
        None,
        None,
        "automod_rule_update",
        CreateEmbed::new()
            .title("AutoMod: regle mise a jour")
            .description(format!(
                "Nom: `{}`\nID: `{}`\nActive: {}\nTrigger: `{:?}`\nActions: {}",
                rule.name,
                rule.id.get(),
                rule.enabled,
                rule.trigger.kind(),
                rule.actions.len()
            )),
    )
    .await;
}

pub async fn on_auto_moderation_rule_delete(ctx: &Context, rule: &Rule) {
    emit_log(
        ctx,
        rule.guild_id,
        "moderation",
        Some(rule.creator_id),
        None,
        None,
        "automod_rule_delete",
        CreateEmbed::new()
            .title("AutoMod: regle supprimee")
            .description(format!(
                "Nom: `{}`\nID: `{}`\nTrigger: `{:?}`",
                rule.name,
                rule.id.get(),
                rule.trigger.kind()
            )),
    )
    .await;
}

pub async fn on_auto_moderation_action_execution(ctx: &Context, execution: &ActionExecution) {
    let channel = execution
        .channel_id
        .map(|id| format!("<#{}>", id.get()))
        .unwrap_or_else(|| "inconnu".to_string());

    emit_log(
        ctx,
        execution.guild_id,
        "moderation",
        Some(execution.user_id),
        execution.channel_id,
        None,
        "automod_action_execution",
        CreateEmbed::new()
            .title("AutoMod: action executee")
            .description(format!(
                "Regle: `{}`\nUtilisateur: <@{}>\nSalon: {}\nAction: `{:?}`\nTrigger: `{:?}`\nMot cle: {}",
                execution.rule_id.get(),
                execution.user_id.get(),
                channel,
                execution.action.kind(),
                execution.trigger_type,
                execution
                    .matched_keyword
                    .as_deref()
                    .unwrap_or("(aucun)")
            )),
    )
    .await;
}

pub async fn on_stage_instance_create(ctx: &Context, stage_instance: &StageInstance) {
    emit_log(
        ctx,
        stage_instance.guild_id,
        "voice",
        None,
        Some(stage_instance.channel_id),
        None,
        "stage_instance_create",
        CreateEmbed::new().title("Stage cree").description(format!(
            "Salon: <#{}>\nTopic: {}",
            stage_instance.channel_id.get(),
            stage_instance.topic
        )),
    )
    .await;
}

pub async fn on_stage_instance_update(ctx: &Context, stage_instance: &StageInstance) {
    emit_log(
        ctx,
        stage_instance.guild_id,
        "voice",
        None,
        Some(stage_instance.channel_id),
        None,
        "stage_instance_update",
        CreateEmbed::new()
            .title("Stage mis a jour")
            .description(format!(
                "Salon: <#{}>\nTopic: {}",
                stage_instance.channel_id.get(),
                stage_instance.topic
            )),
    )
    .await;
}

pub async fn on_stage_instance_delete(ctx: &Context, stage_instance: &StageInstance) {
    emit_log(
        ctx,
        stage_instance.guild_id,
        "voice",
        None,
        Some(stage_instance.channel_id),
        None,
        "stage_instance_delete",
        CreateEmbed::new()
            .title("Stage supprime")
            .description(format!(
                "Salon: <#{}>\nTopic: {}",
                stage_instance.channel_id.get(),
                stage_instance.topic
            )),
    )
    .await;
}

pub async fn on_voice_channel_status_update(
    ctx: &Context,
    old: Option<String>,
    status: Option<String>,
    id: ChannelId,
    guild_id: GuildId,
) {
    if is_nolog_channel(ctx, guild_id, id, "voice").await {
        return;
    }

    emit_log(
        ctx,
        guild_id,
        "voice",
        None,
        Some(id),
        None,
        "voice_channel_status_update",
        CreateEmbed::new()
            .title("Statut vocal mis a jour")
            .description(format!(
                "Salon: <#{}>\nAvant: {}\nApres: {}",
                id.get(),
                old.as_deref().unwrap_or("(aucun)"),
                status.as_deref().unwrap_or("(aucun)")
            )),
    )
    .await;
}

pub async fn on_guild_scheduled_event_create(ctx: &Context, event: &ScheduledEvent) {
    emit_log(
        ctx,
        event.guild_id,
        "channel",
        event.creator_id,
        event.channel_id,
        None,
        "scheduled_event_create",
        CreateEmbed::new()
            .title("Evenement planifie cree")
            .description(format!(
                "Nom: `{}`\nID: `{}`\nDebut: {}\nStatut: `{:?}`",
                event.name,
                event.id.get(),
                event.start_time,
                event.status
            )),
    )
    .await;
}

pub async fn on_guild_scheduled_event_update(ctx: &Context, event: &ScheduledEvent) {
    emit_log(
        ctx,
        event.guild_id,
        "channel",
        event.creator_id,
        event.channel_id,
        None,
        "scheduled_event_update",
        CreateEmbed::new()
            .title("Evenement planifie mis a jour")
            .description(format!(
                "Nom: `{}`\nID: `{}`\nDebut: {}\nStatut: `{:?}`",
                event.name,
                event.id.get(),
                event.start_time,
                event.status
            )),
    )
    .await;
}

pub async fn on_guild_scheduled_event_delete(ctx: &Context, event: &ScheduledEvent) {
    emit_log(
        ctx,
        event.guild_id,
        "channel",
        event.creator_id,
        event.channel_id,
        None,
        "scheduled_event_delete",
        CreateEmbed::new()
            .title("Evenement planifie supprime")
            .description(format!("Nom: `{}`\nID: `{}`", event.name, event.id.get())),
    )
    .await;
}

pub async fn on_guild_scheduled_event_user_add(
    ctx: &Context,
    subscribed: &GuildScheduledEventUserAddEvent,
) {
    emit_log(
        ctx,
        subscribed.guild_id,
        "raid",
        Some(subscribed.user_id),
        None,
        None,
        "scheduled_event_user_add",
        CreateEmbed::new()
            .title("Inscription evenement")
            .description(format!(
                "Utilisateur: <@{}>\nEvenement: `{}`",
                subscribed.user_id.get(),
                subscribed.scheduled_event_id.get()
            )),
    )
    .await;
}

pub async fn on_guild_scheduled_event_user_remove(
    ctx: &Context,
    unsubscribed: &GuildScheduledEventUserRemoveEvent,
) {
    emit_log(
        ctx,
        unsubscribed.guild_id,
        "raid",
        Some(unsubscribed.user_id),
        None,
        None,
        "scheduled_event_user_remove",
        CreateEmbed::new()
            .title("Desinscription evenement")
            .description(format!(
                "Utilisateur: <@{}>\nEvenement: `{}`",
                unsubscribed.user_id.get(),
                unsubscribed.scheduled_event_id.get()
            )),
    )
    .await;
}

pub async fn on_integration_create(ctx: &Context, integration: &Integration) {
    let Some(guild_id) = integration.guild_id else {
        return;
    };

    emit_log(
        ctx,
        guild_id,
        "moderation",
        integration.user.as_ref().map(|user| user.id),
        None,
        integration.role_id,
        "integration_create",
        CreateEmbed::new()
            .title("Integration creee")
            .description(format!(
                "Nom: `{}`\nID: `{}`\nType: `{}`\nActive: {}",
                integration.name,
                integration.id.get(),
                integration.kind,
                integration.enabled
            )),
    )
    .await;
}

pub async fn on_integration_update(ctx: &Context, integration: &Integration) {
    let Some(guild_id) = integration.guild_id else {
        return;
    };

    emit_log(
        ctx,
        guild_id,
        "moderation",
        integration.user.as_ref().map(|user| user.id),
        None,
        integration.role_id,
        "integration_update",
        CreateEmbed::new()
            .title("Integration mise a jour")
            .description(format!(
                "Nom: `{}`\nID: `{}`\nType: `{}`\nActive: {}",
                integration.name,
                integration.id.get(),
                integration.kind,
                integration.enabled
            )),
    )
    .await;
}

pub async fn on_integration_delete(
    ctx: &Context,
    integration_id: IntegrationId,
    guild_id: GuildId,
    application_id: Option<ApplicationId>,
) {
    emit_log(
        ctx,
        guild_id,
        "moderation",
        None,
        None,
        None,
        "integration_delete",
        CreateEmbed::new()
            .title("Integration supprimee")
            .description(format!(
                "Integration: `{}`\nApplication: {}",
                integration_id.get(),
                application_id
                    .map(|id| id.get().to_string())
                    .unwrap_or_else(|| "inconnue".to_string())
            )),
    )
    .await;
}

pub async fn on_guild_integrations_update(ctx: &Context, guild_id: GuildId) {
    send_log_embed(
        ctx,
        guild_id,
        "moderation",
        CreateEmbed::new()
            .title("Integrations du serveur mises a jour")
            .description("Discord a signale une mise a jour globale des integrations."),
    )
    .await;
}
