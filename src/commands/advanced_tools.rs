use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serenity::builder::{
    CreateActionRow, CreateButton, CreateChannel, CreateEmbed, CreateEmbedFooter, CreateInputText,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateModal,
    EditMember, EditMessage, EditRole,
};
use serenity::model::application::{ActionRowComponent, InputTextStyle};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_channel_id, send_embed, theme_color};
use crate::db::DbPoolKey;

static MAINTENANCE_TICK: OnceLock<Mutex<Instant>> = OnceLock::new();

const ADV_GIVEAWAY_OPEN_MODAL: &str = "adv:giveaway:open_modal";
const ADV_GIVEAWAY_END_MODAL: &str = "adv:giveaway:end_modal";
const ADV_BACKUP_CREATE_MODAL: &str = "adv:backup:create_modal";
const ADV_BACKUP_LIST_MODAL: &str = "adv:backup:list_modal";
const ADV_BACKUP_LOAD_MODAL: &str = "adv:backup:load_modal";
const ADV_BACKUP_DELETE_MODAL: &str = "adv:backup:delete_modal";
const ADV_AUTOREACT_ADD_MODAL: &str = "adv:autoreact:add_modal";
const ADV_AUTOREACT_DEL_MODAL: &str = "adv:autoreact:del_modal";
const ADV_AUTOREACT_LIST: &str = "adv:autoreact:list";
const ADV_CHOOSE_MODAL: &str = "adv:choose:modal";
const ADV_EMBED_MODAL: &str = "adv:embed:modal";
const ADV_LOADING_MODAL: &str = "adv:loading:modal";

fn parse_owner_component_id(custom_id: &str) -> Option<(&str, u64)> {
    let mut parts = custom_id.rsplitn(2, ':');
    let owner = parts.next()?.parse::<u64>().ok()?;
    let action = parts.next()?;
    if !action.starts_with("adv:") {
        return None;
    }
    Some((action, owner))
}

fn modal_value(modal: &ModalInteraction, wanted_id: &str) -> Option<String> {
    for row in &modal.data.components {
        for component in &row.components {
            if let ActionRowComponent::InputText(input) = component {
                if input.custom_id == wanted_id {
                    return input.value.clone();
                }
            }
        }
    }
    None
}

async fn respond_ephemeral(
    ctx: &Context,
    component: &ComponentInteraction,
    content: impl Into<String>,
) {
    let _ = component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(content.into())
                    .ephemeral(true),
            ),
        )
        .await;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BackupRole {
    id: u64,
    name: String,
    color: u32,
    hoist: bool,
    mentionable: bool,
    permissions: u64,
    position: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BackupOverwrite {
    kind: String,
    target_id: u64,
    allow: u64,
    deny: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BackupChannel {
    id: u64,
    name: String,
    kind: String,
    position: i64,
    parent_id: Option<u64>,
    topic: Option<String>,
    nsfw: bool,
    bitrate: Option<u32>,
    user_limit: Option<u32>,
    slowmode: Option<u16>,
    overwrites: Vec<BackupOverwrite>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BackupEmoji {
    id: u64,
    name: String,
    animated: bool,
    image_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BackupMemberRoles {
    user_id: u64,
    role_ids: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServerBackupPayload {
    guild_id: u64,
    guild_name: String,
    roles: Vec<BackupRole>,
    channels: Vec<BackupChannel>,
    emojis: Vec<BackupEmoji>,
    members: Vec<BackupMemberRoles>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EmojiBackupPayload {
    guild_id: u64,
    guild_name: String,
    emojis: Vec<BackupEmoji>,
}

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

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
        "" => value,
        _ => return None,
    };

    Some(Duration::from_secs(secs.max(1)))
}

fn parse_backup_kind(input: &str) -> Option<&'static str> {
    match input.to_lowercase().as_str() {
        "serveur" | "server" | "srv" => Some("server"),
        "emoji" | "emojis" => Some("emoji"),
        _ => None,
    }
}

pub fn backup_kind_from_input(input: &str) -> Option<&'static str> {
    parse_backup_kind(input)
}

fn channel_kind_to_str(kind: ChannelType) -> String {
    match kind {
        ChannelType::Text => "text",
        ChannelType::Voice => "voice",
        ChannelType::Category => "category",
        ChannelType::News => "news",
        ChannelType::Stage => "stage",
        ChannelType::Forum => "forum",
        _ => "other",
    }
    .to_string()
}

fn channel_kind_from_str(kind: &str) -> ChannelType {
    match kind {
        "voice" => ChannelType::Voice,
        "category" => ChannelType::Category,
        "news" => ChannelType::News,
        "stage" => ChannelType::Stage,
        "forum" => ChannelType::Forum,
        _ => ChannelType::Text,
    }
}

fn serialize_overwrites(source: &[PermissionOverwrite]) -> Vec<BackupOverwrite> {
    source
        .iter()
        .map(|ow| {
            let (kind, target_id) = match ow.kind {
                PermissionOverwriteType::Role(role_id) => ("role".to_string(), role_id.get()),
                PermissionOverwriteType::Member(user_id) => ("member".to_string(), user_id.get()),
                _ => ("other".to_string(), 0),
            };

            BackupOverwrite {
                kind,
                target_id,
                allow: ow.allow.bits(),
                deny: ow.deny.bits(),
            }
        })
        .collect()
}

fn deserialize_overwrites(
    source: &[BackupOverwrite],
    role_map: &HashMap<u64, RoleId>,
) -> Vec<PermissionOverwrite> {
    let mut out = Vec::new();

    for ow in source {
        let kind = match ow.kind.as_str() {
            "role" => {
                let Some(mapped) = role_map.get(&ow.target_id) else {
                    continue;
                };
                PermissionOverwriteType::Role(*mapped)
            }
            "member" => PermissionOverwriteType::Member(UserId::new(ow.target_id)),
            _ => continue,
        };

        out.push(PermissionOverwrite {
            allow: Permissions::from_bits_truncate(ow.allow),
            deny: Permissions::from_bits_truncate(ow.deny),
            kind,
        });
    }

    out
}

pub async fn apply_autoreacts(ctx: &Context, msg: &Message) {
    if msg.author.bot {
        return;
    }

    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id;
    let rows = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT emoji
        FROM bot_autoreacts
        WHERE bot_id = $1 AND guild_id = $2 AND channel_id = $3
        ORDER BY emoji ASC;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(msg.channel_id.get() as i64)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    for (emoji_text,) in rows {
        if let Ok(reaction_type) = ReactionType::try_from(emoji_text.as_str()) {
            let _ = msg.react(&ctx.http, reaction_type).await;
        }
    }
}

pub async fn maybe_run_maintenance(ctx: &Context, guild_id: Option<GuildId>) {
    let Some(guild_id) = guild_id else {
        return;
    };

    let now = Instant::now();
    let lock =
        MAINTENANCE_TICK.get_or_init(|| Mutex::new(Instant::now() - Duration::from_secs(60)));
    {
        let mut last = lock.lock().expect("maintenance tick lock poisoned");
        if now.duration_since(*last) < Duration::from_secs(30) {
            return;
        }
        *last = now;
    }

    run_temprole_cleanup(ctx, guild_id).await;
    run_autobackup_tick(ctx, guild_id).await;
}

async fn run_temprole_cleanup(ctx: &Context, guild_id: GuildId) {
    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id;
    let now = Utc::now();

    let rows = sqlx::query_as::<_, (i64, i64)>(
        r#"
        SELECT user_id, role_id
        FROM bot_temproles
        WHERE bot_id = $1 AND guild_id = $2 AND active = TRUE AND expires_at <= $3;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(now)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    for (user_id, role_id) in &rows {
        if let Ok(member) = guild_id
            .member(&ctx.http, UserId::new(*user_id as u64))
            .await
        {
            let _ = member
                .remove_role(&ctx.http, RoleId::new(*role_id as u64))
                .await;
        }
    }

    if !rows.is_empty() {
        let _ = sqlx::query(
            r#"
            UPDATE bot_temproles
            SET active = FALSE
            WHERE bot_id = $1 AND guild_id = $2 AND active = TRUE AND expires_at <= $3;
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(now)
        .execute(&pool)
        .await;
    }
}

async fn run_autobackup_tick(ctx: &Context, guild_id: GuildId) {
    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id;
    let rows = sqlx::query_as::<_, (String, i32)>(
        r#"
        SELECT kind, interval_days
        FROM bot_autobackups
        WHERE bot_id = $1 AND guild_id = $2 AND next_run_at <= NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    for (kind, days) in rows {
        let auto_name = format!("auto_{}_{}", kind, Utc::now().format("%Y%m%d_%H%M%S"));
        let _ = create_backup_internal(ctx, guild_id, &kind, &auto_name).await;

        let _ = sqlx::query(
            r#"
            UPDATE bot_autobackups
            SET last_run_at = NOW(),
                next_run_at = NOW() + make_interval(days => $4)
            WHERE bot_id = $1 AND guild_id = $2 AND kind = $3;
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(&kind)
        .bind(days)
        .execute(&pool)
        .await;
    }
}

async fn serialize_server_backup(
    ctx: &Context,
    guild_id: GuildId,
) -> Result<ServerBackupPayload, String> {
    let guild = guild_id
        .to_partial_guild(&ctx.http)
        .await
        .map_err(|e| format!("Impossible de lire le serveur: {e}"))?;

    let channels = guild_id
        .channels(&ctx.http)
        .await
        .map_err(|e| format!("Impossible de lire les salons: {e}"))?;

    let members = guild_id
        .members(&ctx.http, None, None)
        .await
        .unwrap_or_default();

    let mut roles = guild
        .roles
        .values()
        .map(|role| BackupRole {
            id: role.id.get(),
            name: role.name.clone(),
            color: role.colour.0,
            hoist: role.hoist,
            mentionable: role.mentionable,
            permissions: role.permissions.bits(),
            position: role.position as i64,
        })
        .collect::<Vec<_>>();
    roles.sort_by_key(|r| r.position);

    let mut channels_list = channels
        .values()
        .map(|ch| BackupChannel {
            id: ch.id.get(),
            name: ch.name.clone(),
            kind: channel_kind_to_str(ch.kind),
            position: ch.position as i64,
            parent_id: ch.parent_id.map(|id| id.get()),
            topic: ch.topic.clone(),
            nsfw: ch.nsfw,
            bitrate: ch.bitrate.map(|v| v as u32),
            user_limit: ch.user_limit.map(|v| v as u32),
            slowmode: ch.rate_limit_per_user,
            overwrites: serialize_overwrites(&ch.permission_overwrites),
        })
        .collect::<Vec<_>>();
    channels_list.sort_by(|a, b| {
        a.position
            .cmp(&b.position)
            .then_with(|| a.name.cmp(&b.name))
    });

    let emojis = guild
        .emojis
        .values()
        .map(|emoji| BackupEmoji {
            id: emoji.id.get(),
            name: emoji.name.clone(),
            animated: emoji.animated,
            image_url: emoji.url(),
        })
        .collect::<Vec<_>>();

    let members = members
        .into_iter()
        .map(|m| BackupMemberRoles {
            user_id: m.user.id.get(),
            role_ids: m.roles.iter().map(|rid| rid.get()).collect(),
        })
        .collect::<Vec<_>>();

    Ok(ServerBackupPayload {
        guild_id: guild_id.get(),
        guild_name: guild.name,
        roles,
        channels: channels_list,
        emojis,
        members,
    })
}

async fn serialize_emoji_backup(
    ctx: &Context,
    guild_id: GuildId,
) -> Result<EmojiBackupPayload, String> {
    let guild = guild_id
        .to_partial_guild(&ctx.http)
        .await
        .map_err(|e| format!("Impossible de lire le serveur: {e}"))?;

    let emojis = guild
        .emojis
        .values()
        .map(|emoji| BackupEmoji {
            id: emoji.id.get(),
            name: emoji.name.clone(),
            animated: emoji.animated,
            image_url: emoji.url(),
        })
        .collect::<Vec<_>>();

    Ok(EmojiBackupPayload {
        guild_id: guild_id.get(),
        guild_name: guild.name,
        emojis,
    })
}

async fn create_backup_internal(
    ctx: &Context,
    guild_id: GuildId,
    kind: &str,
    name: &str,
) -> Result<(), String> {
    let Some(pool) = pool(ctx).await else {
        return Err("Base de données indisponible".to_string());
    };

    let bot_id = ctx.cache.current_user().id;

    let payload_value = if kind == "server" {
        serde_json::to_value(serialize_server_backup(ctx, guild_id).await?)
            .map_err(|e| format!("Erreur serialisation backup: {e}"))?
    } else {
        serde_json::to_value(serialize_emoji_backup(ctx, guild_id).await?)
            .map_err(|e| format!("Erreur serialisation backup: {e}"))?
    };

    sqlx::query(
        r#"
        INSERT INTO bot_backups (bot_id, guild_id, kind, backup_name, payload)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (bot_id, guild_id, kind, backup_name)
        DO UPDATE SET payload = EXCLUDED.payload, created_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(kind)
    .bind(name)
    .bind(payload_value)
    .execute(&pool)
    .await
    .map_err(|e| format!("Erreur insertion backup: {e}"))?;

    Ok(())
}

pub async fn backup_create(
    ctx: &Context,
    guild_id: GuildId,
    kind: &str,
    name: &str,
) -> Result<(), String> {
    create_backup_internal(ctx, guild_id, kind, name).await
}

async fn restore_emoji_backup(
    ctx: &Context,
    guild_id: GuildId,
    payload: EmojiBackupPayload,
) -> Result<usize, String> {
    let mut created = 0usize;

    for emoji in payload.emojis {
        let response = reqwest::get(&emoji.image_url)
            .await
            .map_err(|e| format!("Erreur téléchargement emoji {}: {e}", emoji.name))?;
        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Erreur lecture emoji {}: {e}", emoji.name))?;

        let data_uri = format!("data:image/png;base64,{}", {
            use base64::Engine;
            base64::engine::general_purpose::STANDARD.encode(bytes)
        });

        if guild_id
            .create_emoji(&ctx.http, &emoji.name, &data_uri)
            .await
            .is_ok()
        {
            created += 1;
        }
    }

    Ok(created)
}

async fn restore_server_backup(
    ctx: &Context,
    guild_id: GuildId,
    payload: ServerBackupPayload,
) -> Result<(usize, usize, usize), String> {
    let partial = guild_id
        .to_partial_guild(&ctx.http)
        .await
        .map_err(|e| format!("Impossible de lire le serveur: {e}"))?;

    let mut role_map = HashMap::<u64, RoleId>::new();
    let mut category_map = HashMap::<u64, ChannelId>::new();

    for role in partial.roles.values() {
        role_map.insert(role.id.get(), role.id);
    }

    let mut created_roles = 0usize;
    let mut created_channels = 0usize;
    let mut restored_members = 0usize;

    for role in payload.roles.iter().filter(|r| r.name != "@everyone") {
        let existing = partial
            .roles
            .values()
            .find(|r| r.name == role.name)
            .map(|r| r.id);
        let role_id = if let Some(existing_id) = existing {
            existing_id
        } else {
            let created = guild_id
                .create_role(
                    &ctx.http,
                    EditRole::new()
                        .name(&role.name)
                        .hoist(role.hoist)
                        .mentionable(role.mentionable)
                        .permissions(Permissions::from_bits_truncate(role.permissions))
                        .colour(role.color),
                )
                .await
                .map_err(|e| format!("Creation role {} impossible: {e}", role.name))?;
            created_roles += 1;
            created.id
        };

        role_map.insert(role.id, role_id);
    }

    for channel in payload.channels.iter().filter(|ch| ch.kind == "category") {
        let created = guild_id
            .create_channel(
                &ctx.http,
                CreateChannel::new(&channel.name)
                    .kind(ChannelType::Category)
                    .position(channel.position as u16),
            )
            .await;

        if let Ok(new_channel) = created {
            category_map.insert(channel.id, new_channel.id);
            created_channels += 1;
        }
    }

    for channel in payload.channels.iter().filter(|ch| ch.kind != "category") {
        let mut builder = CreateChannel::new(&channel.name)
            .kind(channel_kind_from_str(&channel.kind))
            .position(channel.position as u16)
            .nsfw(channel.nsfw);

        if let Some(parent_id) = channel.parent_id {
            if let Some(mapped_parent) = category_map.get(&parent_id) {
                builder = builder.category(*mapped_parent);
            }
        }
        if let Some(topic) = &channel.topic {
            builder = builder.topic(topic);
        }
        if let Some(bitrate) = channel.bitrate {
            builder = builder.bitrate(bitrate);
        }
        if let Some(user_limit) = channel.user_limit {
            builder = builder.user_limit(user_limit);
        }
        if let Some(slowmode) = channel.slowmode {
            builder = builder.rate_limit_per_user(slowmode);
        }

        let overwrites = deserialize_overwrites(&channel.overwrites, &role_map);
        if !overwrites.is_empty() {
            builder = builder.permissions(overwrites);
        }

        if guild_id.create_channel(&ctx.http, builder).await.is_ok() {
            created_channels += 1;
        }
    }

    for member in &payload.members {
        if let Ok(mut target) = guild_id
            .member(&ctx.http, UserId::new(member.user_id))
            .await
        {
            let mapped_roles = member
                .role_ids
                .iter()
                .filter_map(|old_id| role_map.get(old_id).copied())
                .collect::<Vec<_>>();

            if target
                .edit(&ctx.http, EditMember::new().roles(mapped_roles))
                .await
                .is_ok()
            {
                restored_members += 1;
            }
        }
    }

    Ok((created_roles, created_channels, restored_members))
}

async fn handle_backup_list(ctx: &Context, msg: &Message, guild_id: GuildId, kind: &str) {
    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id;
    let rows = sqlx::query_as::<_, (String, DateTime<Utc>)>(
        r#"
        SELECT backup_name, created_at
        FROM bot_backups
        WHERE bot_id = $1 AND guild_id = $2 AND kind = $3
        ORDER BY created_at DESC;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(kind)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let desc = if rows.is_empty() {
        "Aucune backup enregistrée.".to_string()
    } else {
        rows.into_iter()
            .map(|(name, ts)| format!("- `{}` · <t:{}:R>", name, ts.timestamp()))
            .collect::<Vec<_>>()
            .join("\n")
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title(format!("Backups {}", kind))
            .description(desc)
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn backup_list(ctx: &Context, msg: &Message, guild_id: GuildId, kind: &str) {
    handle_backup_list(ctx, msg, guild_id, kind).await;
}

async fn handle_backup_delete(
    ctx: &Context,
    msg: &Message,
    guild_id: GuildId,
    kind: &str,
    name: &str,
) {
    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id;
    let deleted = sqlx::query(
        r#"
        DELETE FROM bot_backups
        WHERE bot_id = $1 AND guild_id = $2 AND kind = $3 AND backup_name = $4;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(kind)
    .bind(name)
    .execute(&pool)
    .await
    .ok()
    .map(|res| res.rows_affected())
    .unwrap_or(0);

    let desc = if deleted > 0 {
        format!("Backup `{}` supprimée.", name)
    } else {
        format!("Aucune backup `{}` trouvée.", name)
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Backup")
            .description(desc)
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn backup_delete(
    ctx: &Context,
    msg: &Message,
    guild_id: GuildId,
    kind: &str,
    name: &str,
) {
    handle_backup_delete(ctx, msg, guild_id, kind, name).await;
}

async fn handle_backup_load(
    ctx: &Context,
    msg: &Message,
    guild_id: GuildId,
    kind: &str,
    name: &str,
) {
    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id;

    let row = sqlx::query_as::<_, (serde_json::Value,)>(
        r#"
        SELECT payload
        FROM bot_backups
        WHERE bot_id = $1 AND guild_id = $2 AND kind = $3 AND backup_name = $4
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(kind)
    .bind(name)
    .fetch_optional(&pool)
    .await
    .ok()
    .flatten();

    let Some((payload_value,)) = row else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Backup")
                .description("Backup introuvable.")
                .color(0xED4245),
        )
        .await;
        return;
    };

    let result_text = if kind == "emoji" {
        match serde_json::from_value::<EmojiBackupPayload>(payload_value) {
            Ok(payload) => match restore_emoji_backup(ctx, guild_id, payload).await {
                Ok(count) => format!("Load emoji terminé: {} emojis créés.", count),
                Err(err) => format!("Erreur load emoji: {}", err),
            },
            Err(err) => format!("Payload invalide: {err}"),
        }
    } else {
        match serde_json::from_value::<ServerBackupPayload>(payload_value) {
            Ok(payload) => match restore_server_backup(ctx, guild_id, payload).await {
                Ok((roles, channels, members)) => format!(
                    "Load serveur terminé: {} rôles, {} salons, {} membres synchronisés.",
                    roles, channels, members
                ),
                Err(err) => format!("Erreur load serveur: {}", err),
            },
            Err(err) => format!("Payload invalide: {err}"),
        }
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Backup")
            .description(result_text)
            .color(theme_color(ctx).await),
    )
    .await;
}

pub async fn backup_load(ctx: &Context, msg: &Message, guild_id: GuildId, kind: &str, name: &str) {
    handle_backup_load(ctx, msg, guild_id, kind, name).await;
}

pub async fn handle_component_interaction(ctx: &Context, component: &ComponentInteraction) -> bool {
    let Some((action, owner_id)) = parse_owner_component_id(&component.data.custom_id) else {
        if component.data.custom_id == "adv:giveaway:join" {
            respond_ephemeral(ctx, component, "Participation enregistrée. Bonne chance !").await;
            return true;
        }
        return false;
    };

    if component.user.id.get() != owner_id {
        respond_ephemeral(
            ctx,
            component,
            "Seul l'auteur du menu peut utiliser ce bouton.",
        )
        .await;
        return true;
    }

    let open_modal = |custom_id: String, title: &str, rows: Vec<CreateActionRow>| {
        CreateInteractionResponse::Modal(CreateModal::new(custom_id, title).components(rows))
    };

    let response = match action {
        ADV_GIVEAWAY_OPEN_MODAL => Some(open_modal(
            component.data.custom_id.clone(),
            "Créer un Giveaway",
            vec![
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Titre", "title")
                        .required(true)
                        .max_length(100),
                ),
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Durée (ex: 10m)", "duration")
                        .required(true)
                        .max_length(20),
                ),
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Nombre de gagnants", "winners")
                        .required(true)
                        .max_length(3),
                ),
            ],
        )),
        ADV_GIVEAWAY_END_MODAL => Some(open_modal(
            component.data.custom_id.clone(),
            "Terminer un Giveaway",
            vec![CreateActionRow::InputText(
                CreateInputText::new(InputTextStyle::Short, "ID du message", "message_id")
                    .required(true)
                    .max_length(30),
            )],
        )),
        ADV_CHOOSE_MODAL => Some(open_modal(
            component.data.custom_id.clone(),
            "Choose",
            vec![CreateActionRow::InputText(
                CreateInputText::new(
                    InputTextStyle::Paragraph,
                    "Options (séparées par |)",
                    "options",
                )
                .required(true)
                .max_length(1500),
            )],
        )),
        ADV_EMBED_MODAL => Some(open_modal(
            component.data.custom_id.clone(),
            "Créer un Embed",
            vec![
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Titre", "title")
                        .required(true)
                        .max_length(256),
                ),
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Paragraph, "Description", "description")
                        .required(true)
                        .max_length(4000),
                ),
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Couleur hex (optionnel)", "color")
                        .required(false)
                        .max_length(8),
                ),
            ],
        )),
        ADV_LOADING_MODAL => Some(open_modal(
            component.data.custom_id.clone(),
            "Créer un Loading",
            vec![
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Durée (ex: 20s)", "duration")
                        .required(true)
                        .max_length(20),
                ),
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Message", "message")
                        .required(true)
                        .max_length(120),
                ),
            ],
        )),
        ADV_BACKUP_CREATE_MODAL => Some(open_modal(
            component.data.custom_id.clone(),
            "Créer une Backup",
            vec![
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Type (serveur/emoji)", "kind")
                        .required(true)
                        .max_length(20),
                ),
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Nom", "name")
                        .required(true)
                        .max_length(80),
                ),
            ],
        )),
        ADV_BACKUP_LIST_MODAL => Some(open_modal(
            component.data.custom_id.clone(),
            "Lister les Backups",
            vec![CreateActionRow::InputText(
                CreateInputText::new(InputTextStyle::Short, "Type (serveur/emoji)", "kind")
                    .required(true)
                    .max_length(20),
            )],
        )),
        ADV_BACKUP_LOAD_MODAL => Some(open_modal(
            component.data.custom_id.clone(),
            "Charger une Backup",
            vec![
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Type (serveur/emoji)", "kind")
                        .required(true)
                        .max_length(20),
                ),
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Nom", "name")
                        .required(true)
                        .max_length(80),
                ),
            ],
        )),
        ADV_BACKUP_DELETE_MODAL => Some(open_modal(
            component.data.custom_id.clone(),
            "Supprimer une Backup",
            vec![
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Type (serveur/emoji)", "kind")
                        .required(true)
                        .max_length(20),
                ),
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Nom", "name")
                        .required(true)
                        .max_length(80),
                ),
            ],
        )),
        ADV_AUTOREACT_ADD_MODAL | ADV_AUTOREACT_DEL_MODAL => Some(open_modal(
            component.data.custom_id.clone(),
            if action == ADV_AUTOREACT_ADD_MODAL {
                "Ajouter AutoReact"
            } else {
                "Supprimer AutoReact"
            },
            vec![
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Salon (#id)", "channel")
                        .required(true)
                        .max_length(50),
                ),
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Emoji", "emoji")
                        .required(true)
                        .max_length(80),
                ),
            ],
        )),
        ADV_AUTOREACT_LIST => {
            let Some(guild_id) = component.guild_id else {
                respond_ephemeral(ctx, component, "Commande disponible uniquement en serveur.")
                    .await;
                return true;
            };

            let Some(pool) = pool(ctx).await else {
                respond_ephemeral(ctx, component, "Base de données indisponible.").await;
                return true;
            };

            let bot_id = ctx.cache.current_user().id;
            let rows = sqlx::query_as::<_, (i64, String)>(
                r#"
                SELECT channel_id, emoji
                FROM bot_autoreacts
                WHERE bot_id = $1 AND guild_id = $2
                ORDER BY channel_id ASC, emoji ASC;
                "#,
            )
            .bind(bot_id.get() as i64)
            .bind(guild_id.get() as i64)
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

            let text = if rows.is_empty() {
                "Aucun autoreact configuré.".to_string()
            } else {
                rows.into_iter()
                    .map(|(channel_id, emoji)| format!("- <#{}> -> {}", channel_id, emoji))
                    .collect::<Vec<_>>()
                    .join("\n")
            };

            respond_ephemeral(ctx, component, text).await;
            None
        }
        _ => None,
    };

    if let Some(response) = response {
        let _ = component.create_response(&ctx.http, response).await;
        return true;
    }

    false
}

pub async fn handle_modal_interaction(ctx: &Context, modal: &ModalInteraction) -> bool {
    let Some((action, owner_id)) = parse_owner_component_id(&modal.data.custom_id) else {
        return false;
    };

    if modal.user.id.get() != owner_id {
        let _ = modal
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Seul l'auteur du menu peut soumettre ce formulaire.")
                        .ephemeral(true),
                ),
            )
            .await;
        return true;
    }

    let Some(guild_id) = modal.guild_id else {
        let _ = modal
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Cette action nécessite un serveur.")
                        .ephemeral(true),
                ),
            )
            .await;
        return true;
    };

    match action {
        ADV_GIVEAWAY_OPEN_MODAL => {
            let title = modal_value(modal, "title").unwrap_or_else(|| "Giveaway".to_string());
            let duration = modal_value(modal, "duration").unwrap_or_else(|| "10m".to_string());
            let winners = modal_value(modal, "winners").unwrap_or_else(|| "1".to_string());

            let embed = CreateEmbed::new()
                .title(format!("🎉 {}", title))
                .description(format!(
                    "Clique sur le bouton pour participer.\nDurée: **{}**\nGagnants: **{}**",
                    duration, winners
                ))
                .color(theme_color(ctx).await)
                .footer(CreateEmbedFooter::new(
                    "Utilise +end giveaway <ID> pour terminer",
                ));

            let _ = modal
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .embed(embed)
                            .components(vec![CreateActionRow::Buttons(vec![
                                CreateButton::new("adv:giveaway:join")
                                    .label("Participer")
                                    .emoji('🎉')
                                    .style(ButtonStyle::Success),
                            ])]),
                    ),
                )
                .await;
            return true;
        }
        ADV_GIVEAWAY_END_MODAL => {
            let message_id = modal_value(modal, "message_id")
                .and_then(|v| v.trim().parse::<u64>().ok())
                .map(MessageId::new);

            if let Some(message_id) = message_id {
                let _ = modal
                    .channel_id
                    .edit_message(
                        &ctx.http,
                        message_id,
                        EditMessage::new().content("🎉 Giveaway terminé manuellement."),
                    )
                    .await;

                let _ = modal
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("Giveaway terminé.")
                                .ephemeral(true),
                        ),
                    )
                    .await;
            } else {
                let _ = modal
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("ID invalide.")
                                .ephemeral(true),
                        ),
                    )
                    .await;
            }
            return true;
        }
        ADV_CHOOSE_MODAL => {
            let content = modal_value(modal, "options").unwrap_or_default();
            let options = content
                .split('|')
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty())
                .collect::<Vec<_>>();

            let text = if options.len() >= 2 {
                let pick = options
                    .choose(&mut rand::thread_rng())
                    .cloned()
                    .unwrap_or_else(|| options[0].clone());
                format!("Résultat: **{}**", pick)
            } else {
                "Donne au moins 2 options séparées par `|`.".to_string()
            };

            let _ = modal
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(text)
                            .ephemeral(true),
                    ),
                )
                .await;
            return true;
        }
        ADV_EMBED_MODAL => {
            let title = modal_value(modal, "title").unwrap_or_else(|| "Embed".to_string());
            let description = modal_value(modal, "description").unwrap_or_default();
            let color_raw = modal_value(modal, "color").unwrap_or_default();
            let color = u32::from_str_radix(
                color_raw
                    .trim()
                    .trim_start_matches('#')
                    .trim_start_matches("0x"),
                16,
            )
            .unwrap_or(theme_color(ctx).await);

            let embed = CreateEmbed::new()
                .title(title)
                .description(description)
                .color(color);

            let _ = modal
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().embed(embed),
                    ),
                )
                .await;
            return true;
        }
        ADV_LOADING_MODAL => {
            let duration_raw = modal_value(modal, "duration").unwrap_or_else(|| "10s".to_string());
            let message = modal_value(modal, "message").unwrap_or_else(|| "Chargement".to_string());

            let _ = modal
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("Animation lancée.")
                            .ephemeral(true),
                    ),
                )
                .await;

            let Some(duration) = duration_from_input(&duration_raw) else {
                return true;
            };
            let total_secs = duration.as_secs().clamp(1, 120);
            let ctx_cloned = ctx.clone();
            let channel_id = modal.channel_id;
            tokio::spawn(async move {
                let mut sent = match channel_id
                    .send_message(
                        &ctx_cloned.http,
                        CreateMessage::new().content("[----------] 0%"),
                    )
                    .await
                {
                    Ok(m) => m,
                    Err(_) => return,
                };

                for i in 0..=10_u64 {
                    let done = "#".repeat(i as usize);
                    let todo = "-".repeat((10 - i) as usize);
                    let percent = i * 10;

                    let _ = sent
                        .edit(
                            &ctx_cloned.http,
                            EditMessage::new()
                                .content(format!("{} [{}{}] {}%", message, done, todo, percent)),
                        )
                        .await;

                    if i < 10 {
                        tokio::time::sleep(Duration::from_secs((total_secs / 10).max(1))).await;
                    }
                }
            });

            return true;
        }
        ADV_BACKUP_CREATE_MODAL
        | ADV_BACKUP_LIST_MODAL
        | ADV_BACKUP_LOAD_MODAL
        | ADV_BACKUP_DELETE_MODAL => {
            let Some(pool) = pool(ctx).await else {
                let _ = modal
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("Base de données indisponible.")
                                .ephemeral(true),
                        ),
                    )
                    .await;
                return true;
            };
            let bot_id = ctx.cache.current_user().id;
            let kind_raw = modal_value(modal, "kind").unwrap_or_default();
            let Some(kind) = parse_backup_kind(&kind_raw) else {
                let _ = modal
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("Type invalide: utilise serveur ou emoji.")
                                .ephemeral(true),
                        ),
                    )
                    .await;
                return true;
            };

            let text = if action == ADV_BACKUP_CREATE_MODAL {
                let name = modal_value(modal, "name").unwrap_or_else(|| "backup".to_string());
                match create_backup_internal(ctx, guild_id, kind, name.trim()).await {
                    Ok(()) => format!("Backup `{}` ({}) créée.", name.trim(), kind),
                    Err(err) => format!("Erreur: {}", err),
                }
            } else if action == ADV_BACKUP_LIST_MODAL {
                let rows = sqlx::query_as::<_, (String, DateTime<Utc>)>(
                    r#"
                    SELECT backup_name, created_at
                    FROM bot_backups
                    WHERE bot_id = $1 AND guild_id = $2 AND kind = $3
                    ORDER BY created_at DESC;
                    "#,
                )
                .bind(bot_id.get() as i64)
                .bind(guild_id.get() as i64)
                .bind(kind)
                .fetch_all(&pool)
                .await
                .unwrap_or_default();

                if rows.is_empty() {
                    "Aucune backup enregistrée.".to_string()
                } else {
                    rows.into_iter()
                        .map(|(name, ts)| format!("- `{}` · <t:{}:R>", name, ts.timestamp()))
                        .collect::<Vec<_>>()
                        .join("\n")
                }
            } else if action == ADV_BACKUP_LOAD_MODAL {
                let name = modal_value(modal, "name").unwrap_or_default();
                let row = sqlx::query_as::<_, (serde_json::Value,)>(
                    r#"
                    SELECT payload
                    FROM bot_backups
                    WHERE bot_id = $1 AND guild_id = $2 AND kind = $3 AND backup_name = $4
                    LIMIT 1;
                    "#,
                )
                .bind(bot_id.get() as i64)
                .bind(guild_id.get() as i64)
                .bind(kind)
                .bind(name.trim())
                .fetch_optional(&pool)
                .await
                .ok()
                .flatten();

                if let Some((payload_value,)) = row {
                    if kind == "emoji" {
                        match serde_json::from_value::<EmojiBackupPayload>(payload_value) {
                            Ok(payload) => match restore_emoji_backup(ctx, guild_id, payload).await
                            {
                                Ok(count) => format!("Load emoji terminé: {} emojis créés.", count),
                                Err(err) => format!("Erreur load emoji: {}", err),
                            },
                            Err(err) => format!("Payload invalide: {err}"),
                        }
                    } else {
                        match serde_json::from_value::<ServerBackupPayload>(payload_value) {
                            Ok(payload) => {
                                match restore_server_backup(ctx, guild_id, payload).await {
                                    Ok((roles, channels, members)) => format!(
                                        "Load serveur terminé: {} rôles, {} salons, {} membres.",
                                        roles, channels, members
                                    ),
                                    Err(err) => format!("Erreur load serveur: {}", err),
                                }
                            }
                            Err(err) => format!("Payload invalide: {err}"),
                        }
                    }
                } else {
                    "Backup introuvable.".to_string()
                }
            } else {
                let name = modal_value(modal, "name").unwrap_or_default();
                let deleted = sqlx::query(
                    r#"
                    DELETE FROM bot_backups
                    WHERE bot_id = $1 AND guild_id = $2 AND kind = $3 AND backup_name = $4;
                    "#,
                )
                .bind(bot_id.get() as i64)
                .bind(guild_id.get() as i64)
                .bind(kind)
                .bind(name.trim())
                .execute(&pool)
                .await
                .ok()
                .map(|res| res.rows_affected())
                .unwrap_or(0);

                if deleted > 0 {
                    format!("Backup `{}` supprimée.", name.trim())
                } else {
                    format!("Aucune backup `{}` trouvée.", name.trim())
                }
            };

            let _ = modal
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(text)
                            .ephemeral(true),
                    ),
                )
                .await;

            return true;
        }
        ADV_AUTOREACT_ADD_MODAL | ADV_AUTOREACT_DEL_MODAL => {
            let Some(pool) = pool(ctx).await else {
                let _ = modal
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("Base de données indisponible.")
                                .ephemeral(true),
                        ),
                    )
                    .await;
                return true;
            };
            let bot_id = ctx.cache.current_user().id;

            let channel_raw = modal_value(modal, "channel").unwrap_or_default();
            let emoji = modal_value(modal, "emoji").unwrap_or_default();
            let Some(channel_id) = parse_channel_id(channel_raw.trim()) else {
                let _ = modal
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("Salon invalide.")
                                .ephemeral(true),
                        ),
                    )
                    .await;
                return true;
            };

            if action == ADV_AUTOREACT_ADD_MODAL {
                let _ = sqlx::query(
                    r#"
                    INSERT INTO bot_autoreacts (bot_id, guild_id, channel_id, emoji)
                    VALUES ($1, $2, $3, $4)
                    ON CONFLICT (bot_id, guild_id, channel_id, emoji) DO NOTHING;
                    "#,
                )
                .bind(bot_id.get() as i64)
                .bind(guild_id.get() as i64)
                .bind(channel_id.get() as i64)
                .bind(emoji.trim())
                .execute(&pool)
                .await;
            } else {
                let _ = sqlx::query(
                    r#"
                    DELETE FROM bot_autoreacts
                    WHERE bot_id = $1 AND guild_id = $2 AND channel_id = $3 AND emoji = $4;
                    "#,
                )
                .bind(bot_id.get() as i64)
                .bind(guild_id.get() as i64)
                .bind(channel_id.get() as i64)
                .bind(emoji.trim())
                .execute(&pool)
                .await;
            }

            let _ = modal
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("Configuration AutoReact mise à jour.")
                            .ephemeral(true),
                    ),
                )
                .await;
            return true;
        }
        _ => {}
    }

    false
}
