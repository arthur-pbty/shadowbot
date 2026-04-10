use chrono::Utc;
use serenity::all::{
    ActionRowComponent, ButtonStyle, Channel, ChannelId, ChannelType, ComponentInteraction,
    GuildId, InputTextStyle, Message, MessageId, ModalInteraction, PermissionOverwrite,
    PermissionOverwriteType, Permissions, RoleId, User, UserId, VoiceState,
};
use serenity::builder::{
    CreateActionRow, CreateButton, CreateChannel, CreateEmbed, CreateInputText,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateModal,
    EditChannel, EditMessage,
};
use serenity::model::Colour;
use serenity::prelude::*;
use std::collections::HashSet;

use crate::db;

const TEMPVOC_MENU: &str = "tempvoc:settings";
const TEMPVOC_ROOM_SCOPE: &str = "room";
const TEMPVOC_MODAL_SCOPE: &str = "modal";
const MEMBER_LIST_INPUT_ID: &str = "members";
const TRANSFER_OWNER_INPUT_ID: &str = "new_owner";
const SETTINGS_NAME_INPUT_ID: &str = "room_name";
const SETTINGS_LIMIT_INPUT_ID: &str = "user_limit";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RoomMode {
    Open,
    Closed,
    Private,
}

impl RoomMode {
    fn from_db(value: &str) -> Self {
        match value {
            "closed" => Self::Closed,
            "private" => Self::Private,
            _ => Self::Open,
        }
    }

    fn as_db(&self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
            Self::Private => "private",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Open => "Ouvert",
            Self::Closed => "Ferme",
            Self::Private => "Prive",
        }
    }
}

fn parse_owner_id(custom_id: &str) -> Option<(String, u64)> {
    let mut parts = custom_id.rsplitn(2, ':');
    let owner = parts.next()?.parse::<u64>().ok()?;
    let action = parts.next()?.to_string();
    Some((action, owner))
}

fn parse_scoped_channel_id(custom_id: &str, expected_scope: &str) -> Option<(String, ChannelId)> {
    let mut parts = custom_id.split(':');
    let namespace = parts.next()?;
    let scope = parts.next()?;
    let action = parts.next()?.to_string();
    let channel_id = parts.next()?.parse::<u64>().ok()?;

    if namespace != "tempvoc" || scope != expected_scope || parts.next().is_some() {
        return None;
    }

    Some((action, ChannelId::new(channel_id)))
}

fn room_button_id(action: &str, channel_id: ChannelId) -> String {
    format!(
        "tempvoc:{}:{}:{}",
        TEMPVOC_ROOM_SCOPE,
        action,
        channel_id.get()
    )
}

fn room_modal_id(action: &str, channel_id: ChannelId) -> String {
    format!(
        "tempvoc:{}:{}:{}",
        TEMPVOC_MODAL_SCOPE,
        action,
        channel_id.get()
    )
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

fn decode_member_list(raw: &str) -> Vec<u64> {
    serde_json::from_str::<Vec<u64>>(raw).unwrap_or_default()
}

fn encode_member_list(ids: &[u64]) -> String {
    serde_json::to_string(ids).unwrap_or_else(|_| "[]".to_string())
}

fn normalize_member_list(ids: Vec<u64>) -> Vec<u64> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();

    for id in ids {
        if id == 0 || !seen.insert(id) {
            continue;
        }
        out.push(id);
    }

    out.sort_unstable();
    out
}

fn parse_user_ids_input(input: &str) -> Vec<u64> {
    let parsed = input
        .split(|ch: char| ch.is_whitespace() || ch == ',' || ch == ';')
        .filter_map(|chunk| {
            let digits: String = chunk.chars().filter(|ch| ch.is_ascii_digit()).collect();
            if digits.is_empty() {
                return None;
            }
            digits.parse::<u64>().ok()
        })
        .collect::<Vec<_>>();

    normalize_member_list(parsed)
}

fn format_member_mentions(ids: &[u64]) -> String {
    if ids.is_empty() {
        return "Aucun".to_string();
    }

    ids.iter()
        .map(|id| format!("<@{}>", id))
        .collect::<Vec<_>>()
        .join(", ")
}

fn normalize_room_name(input: &str) -> String {
    let compact = input.split_whitespace().collect::<Vec<_>>().join(" ");
    compact.trim().chars().take(100).collect::<String>()
}

fn default_room_name(user: &User) -> String {
    normalize_room_name(&format!("🔊 Salon de {}", user.name))
}

async fn unique_voice_name(
    ctx: &Context,
    guild_id: GuildId,
    requested_name: &str,
    ignored_channel_id: Option<ChannelId>,
) -> Option<String> {
    let requested_name = normalize_room_name(requested_name);
    if requested_name.is_empty() {
        return None;
    }

    let Ok(channels) = guild_id.channels(&ctx.http).await else {
        return Some(requested_name);
    };

    let existing_names = channels
        .values()
        .filter(|channel| {
            channel.kind == ChannelType::Voice
                && ignored_channel_id
                    .map(|ignored| ignored != channel.id)
                    .unwrap_or(true)
        })
        .map(|channel| channel.name.to_lowercase())
        .collect::<HashSet<_>>();

    if !existing_names.contains(&requested_name.to_lowercase()) {
        return Some(requested_name);
    }

    for suffix in 2..5000 {
        let suffix_text = format!(" {}", suffix);
        let max_base_len = 100usize.saturating_sub(suffix_text.chars().count());
        let base = requested_name
            .chars()
            .take(max_base_len)
            .collect::<String>();
        let candidate = format!("{}{}", base, suffix_text);

        if !existing_names.contains(&candidate.to_lowercase()) {
            return Some(candidate);
        }
    }

    Some(requested_name)
}

fn tempvoc_embed(settings: &db::TempvocSettings) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title("Tempvoc")
        .description("Gere les vocaux temporaires du serveur.")
        .colour(Colour::from_rgb(100, 180, 255))
        .timestamp(Utc::now())
        .field(
            "Statut",
            if settings.enabled { "Actif" } else { "Inactif" },
            true,
        );

    if let Some(trigger) = settings.trigger_channel_id {
        embed = embed.field("Canal declencheur", format!("<#{}>", trigger), true);
    }

    if let Some(category) = settings.category_id {
        embed = embed.field("Categorie", format!("<#{}>", category), true);
    }

    embed
}

fn tempvoc_settings_components(
    owner_id: UserId,
    settings: &db::TempvocSettings,
) -> Vec<CreateActionRow> {
    let toggle_label = if settings.enabled {
        "Desactiver"
    } else {
        "Activer"
    };

    vec![CreateActionRow::Buttons(vec![
        CreateButton::new(format!("{}:toggle:{}", TEMPVOC_MENU, owner_id.get()))
            .label(toggle_label)
            .style(ButtonStyle::Primary),
        CreateButton::new(format!("{}:configure:{}", TEMPVOC_MENU, owner_id.get()))
            .label("Configurer")
            .style(ButtonStyle::Secondary),
        CreateButton::new(format!("{}:refresh:{}", TEMPVOC_MENU, owner_id.get()))
            .label("Rafraichir")
            .style(ButtonStyle::Success),
    ])]
}

fn tempvoc_room_embed(room: &db::TempvocRoom, notice: Option<&str>) -> CreateEmbed {
    let mode = RoomMode::from_db(&room.voice_mode);
    let whitelist = decode_member_list(&room.whitelist_json);
    let blacklist = decode_member_list(&room.blacklist_json);
    let limit_label = if room.user_limit <= 0 {
        "Illimite".to_string()
    } else {
        room.user_limit.to_string()
    };

    let options = format!(
        "Micro: {}\nCamera: {}\nSoundboard: {}",
        if room.allow_micro {
            "Autorise"
        } else {
            "Bloque"
        },
        if room.allow_camera {
            "Autorisee"
        } else {
            "Bloquee"
        },
        if room.allow_soundboard {
            "Autorise"
        } else {
            "Bloque"
        },
    );

    let mut embed = CreateEmbed::new()
        .title("Configuration du vocal temporaire")
        .description("Voici l'espace de configuration de ton salon vocal temporaire. Les options disponibles te permettent de personnaliser les permissions de ton salon selon tes preferences.")
        .colour(Colour::from_rgb(46, 204, 113))
        .timestamp(Utc::now())
        .field(
            "🔓 Ouvert",
            "Le salon est accessible a tous les membres, sauf ceux en blacklist.",
            false,
        )
        .field(
            "🔒 Ferme",
            "Le salon est visible pour tous, mais seulement accessible aux membres whitelist.",
            false,
        )
        .field(
            "🙈 Prive",
            "Le salon est visible et accessible uniquement pour les membres whitelist.",
            false,
        )
        .field("✅ Whitelist", format_member_mentions(&whitelist), false)
        .field("⛔ Blacklist", format_member_mentions(&blacklist), false)
        .field(
            "🧹 Purge",
            "Deconnecte tous les membres qui ne sont pas owner ou whitelist.",
            false,
        )
        .field(
            "👑 Owner",
            "Transfere la gestion du vocal a un membre de ton choix.",
            false,
        )
        .field(
            "Etat actuel",
            format!(
                "Mode: {}\nOwner: <@{}>\nLimite: {}",
                mode.label(), room.owner_id, limit_label
            ),
            false,
        )
        .field("Options", options, false)
        .field(
            "💡 Astuce",
            "Les membres whitelist ne sont pas impactes par les restrictions de mode.",
            false,
        );

    if let Some(room_name) = &room.room_name {
        embed = embed.field("Nom par defaut", room_name, false);
    }

    if let Some(notice) = notice {
        embed = embed.field("Mise a jour", notice, false);
    }

    embed
}

fn mode_button_style(current_mode: RoomMode, expected_mode: RoomMode) -> ButtonStyle {
    if current_mode == expected_mode {
        ButtonStyle::Success
    } else {
        ButtonStyle::Secondary
    }
}

fn toggle_button_style(active: bool) -> ButtonStyle {
    if active {
        ButtonStyle::Success
    } else {
        ButtonStyle::Danger
    }
}

fn tempvoc_room_components(channel_id: ChannelId, room: &db::TempvocRoom) -> Vec<CreateActionRow> {
    let mode = RoomMode::from_db(&room.voice_mode);

    vec![
        CreateActionRow::Buttons(vec![
            CreateButton::new(room_button_id("open", channel_id))
                .label("Ouvrir")
                .style(mode_button_style(mode, RoomMode::Open)),
            CreateButton::new(room_button_id("closed", channel_id))
                .label("Fermer")
                .style(mode_button_style(mode, RoomMode::Closed)),
            CreateButton::new(room_button_id("private", channel_id))
                .label("Prive")
                .style(mode_button_style(mode, RoomMode::Private)),
        ]),
        CreateActionRow::Buttons(vec![
            CreateButton::new(room_button_id("whitelist", channel_id))
                .label("Whitelist")
                .style(ButtonStyle::Primary),
            CreateButton::new(room_button_id("blacklist", channel_id))
                .label("Blacklist")
                .style(ButtonStyle::Danger),
            CreateButton::new(room_button_id("purge", channel_id))
                .label("Purge")
                .style(ButtonStyle::Secondary),
        ]),
        CreateActionRow::Buttons(vec![
            CreateButton::new(room_button_id("micro", channel_id))
                .label("Micro")
                .style(toggle_button_style(room.allow_micro)),
            CreateButton::new(room_button_id("camera", channel_id))
                .label("Camera")
                .style(toggle_button_style(room.allow_camera)),
            CreateButton::new(room_button_id("soundboard", channel_id))
                .label("Soundboard")
                .style(toggle_button_style(room.allow_soundboard)),
        ]),
        CreateActionRow::Buttons(vec![
            CreateButton::new(room_button_id("transfer", channel_id))
                .label("Transferer l'owner")
                .style(ButtonStyle::Secondary),
            CreateButton::new(room_button_id("settings", channel_id))
                .label("Settings")
                .style(ButtonStyle::Secondary),
            CreateButton::new(room_button_id("save", channel_id))
                .label("Save")
                .style(ButtonStyle::Primary),
        ]),
    ]
}

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<db::DbPoolKey>().cloned()
}

async fn show_menu(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let settings = db::get_or_create_tempvoc_settings(&pool, bot_id, guild_id.get() as i64)
        .await
        .unwrap_or(db::TempvocSettings {
            bot_id,
            guild_id: guild_id.get() as i64,
            trigger_channel_id: None,
            category_id: None,
            enabled: false,
            updated_at: Utc::now(),
        });

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .embed(tempvoc_embed(&settings))
                .components(tempvoc_settings_components(msg.author.id, &settings)),
        )
        .await;
}

pub async fn handle_tempvoc(ctx: &Context, msg: &Message, _args: &[&str]) {
    show_menu(ctx, msg).await;
}

async fn respond_ephemeral_component(
    ctx: &Context,
    component: &ComponentInteraction,
    content: &str,
) {
    let _ = component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(content)
                    .ephemeral(true),
            ),
        )
        .await;
}

async fn respond_ephemeral_modal(ctx: &Context, modal: &ModalInteraction, content: &str) {
    let _ = modal
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(content)
                    .ephemeral(true),
            ),
        )
        .await;
}

fn normalize_room_lists(room: &mut db::TempvocRoom) {
    let mut whitelist = normalize_member_list(decode_member_list(&room.whitelist_json));
    let mut blacklist = normalize_member_list(decode_member_list(&room.blacklist_json));

    whitelist.retain(|id| *id as i64 != room.owner_id);
    blacklist.retain(|id| *id as i64 != room.owner_id);
    whitelist.retain(|id| !blacklist.contains(id));

    room.whitelist_json = encode_member_list(&whitelist);
    room.blacklist_json = encode_member_list(&blacklist);
}

fn mode_permissions(mode: RoomMode) -> (Permissions, Permissions) {
    match mode {
        RoomMode::Open => (
            Permissions::VIEW_CHANNEL.union(Permissions::CONNECT),
            Permissions::empty(),
        ),
        RoomMode::Closed => (Permissions::VIEW_CHANNEL, Permissions::CONNECT),
        RoomMode::Private => (
            Permissions::empty(),
            Permissions::VIEW_CHANNEL.union(Permissions::CONNECT),
        ),
    }
}

async fn apply_room_permissions(ctx: &Context, room: &db::TempvocRoom) -> bool {
    let guild_id = GuildId::new(room.guild_id as u64);
    let channel_id = ChannelId::new(room.channel_id as u64);

    let Ok(channel) = channel_id.to_channel(&ctx.http).await else {
        return false;
    };

    let Channel::Guild(guild_channel) = channel else {
        return false;
    };

    let everyone_role = RoleId::new(guild_id.get());
    let mut overwrites = guild_channel.permission_overwrites.clone();
    overwrites.retain(|overwrite| match overwrite.kind {
        PermissionOverwriteType::Role(role_id) => role_id != everyone_role,
        PermissionOverwriteType::Member(_) => false,
        _ => true,
    });

    let mode = RoomMode::from_db(&room.voice_mode);
    let (mut everyone_allow, mut everyone_deny) = mode_permissions(mode);

    if room.allow_micro {
        everyone_allow |= Permissions::SPEAK;
    } else {
        everyone_deny |= Permissions::SPEAK;
    }

    if room.allow_camera {
        everyone_allow |= Permissions::STREAM;
    } else {
        everyone_deny |= Permissions::STREAM;
    }

    if room.allow_soundboard {
        everyone_allow |= Permissions::USE_SOUNDBOARD;
    } else {
        everyone_deny |= Permissions::USE_SOUNDBOARD;
    }

    overwrites.push(PermissionOverwrite {
        allow: everyone_allow,
        deny: everyone_deny,
        kind: PermissionOverwriteType::Role(everyone_role),
    });

    let owner_id = UserId::new(room.owner_id as u64);
    overwrites.push(PermissionOverwrite {
        allow: Permissions::VIEW_CHANNEL
            .union(Permissions::CONNECT)
            .union(Permissions::SPEAK)
            .union(Permissions::STREAM)
            .union(Permissions::USE_SOUNDBOARD),
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Member(owner_id),
    });

    for member_id in decode_member_list(&room.blacklist_json) {
        if member_id as i64 == room.owner_id {
            continue;
        }

        overwrites.push(PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL.union(Permissions::CONNECT),
            kind: PermissionOverwriteType::Member(UserId::new(member_id)),
        });
    }

    for member_id in decode_member_list(&room.whitelist_json) {
        if member_id as i64 == room.owner_id {
            continue;
        }

        overwrites.push(PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL.union(Permissions::CONNECT),
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(UserId::new(member_id)),
        });
    }

    let limit = room.user_limit.clamp(0, 99) as u32;
    channel_id
        .edit(
            &ctx.http,
            EditChannel::new().permissions(overwrites).user_limit(limit),
        )
        .await
        .is_ok()
}

async fn refresh_room_panel_message(ctx: &Context, room: &db::TempvocRoom, notice: Option<&str>) {
    let Some(control_channel_id) = room.control_message_channel_id else {
        return;
    };

    let Some(control_message_id) = room.control_message_id else {
        return;
    };

    let _ = ChannelId::new(control_channel_id as u64)
        .edit_message(
            &ctx.http,
            MessageId::new(control_message_id as u64),
            EditMessage::new()
                .content(format!("<@{}>", room.owner_id))
                .embed(tempvoc_room_embed(room, notice))
                .components(tempvoc_room_components(
                    ChannelId::new(room.channel_id as u64),
                    room,
                )),
        )
        .await;
}

async fn persist_room_state(
    ctx: &Context,
    pool: &sqlx::PgPool,
    room: &mut db::TempvocRoom,
    notice: Option<&str>,
) -> bool {
    normalize_room_lists(room);

    let Ok(updated_room) = db::save_tempvoc_room_state(pool, room).await else {
        return false;
    };

    *room = updated_room;
    let _ = apply_room_permissions(ctx, room).await;
    refresh_room_panel_message(ctx, room, notice).await;
    true
}

async fn send_room_panel(ctx: &Context, pool: &sqlx::PgPool, room: &mut db::TempvocRoom) {
    let channel_id = ChannelId::new(room.channel_id as u64);

    let Ok(message) = channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .content(format!("<@{}>", room.owner_id))
                .embed(tempvoc_room_embed(room, None))
                .components(tempvoc_room_components(channel_id, room)),
        )
        .await
    else {
        return;
    };

    if let Ok(updated_room) = db::set_tempvoc_room_control_message(
        pool,
        room.channel_id,
        message.channel_id.get() as i64,
        message.id.get() as i64,
    )
    .await
    {
        *room = updated_room;
    }
}

async fn create_temp_channel(
    ctx: &Context,
    guild_id: GuildId,
    user: &User,
    settings: &db::TempvocSettings,
) {
    let Some(trigger_channel_id) = settings.trigger_channel_id else {
        return;
    };

    let Ok(trigger_channel) = ChannelId::new(trigger_channel_id as u64)
        .to_channel(&ctx.http)
        .await
    else {
        return;
    };

    let Channel::Guild(trigger) = trigger_channel else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };

    let profile = db::get_or_create_tempvoc_profile(
        &pool,
        settings.bot_id,
        settings.guild_id,
        user.id.get() as i64,
    )
    .await
    .unwrap_or(db::TempvocProfile {
        bot_id: settings.bot_id,
        guild_id: settings.guild_id,
        user_id: user.id.get() as i64,
        voice_mode: RoomMode::Open.as_db().to_string(),
        allow_micro: true,
        allow_camera: true,
        allow_soundboard: true,
        user_limit: 0,
        room_name: Some(default_room_name(user)),
        updated_at: Utc::now(),
    });

    let base_name = profile
        .room_name
        .as_deref()
        .map(normalize_room_name)
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| default_room_name(user));

    let Some(unique_name) = unique_voice_name(ctx, guild_id, &base_name, None).await else {
        return;
    };

    let category_id = settings
        .category_id
        .map(|value| ChannelId::new(value as u64))
        .or(trigger.parent_id);

    let mut builder = CreateChannel::new(unique_name)
        .kind(ChannelType::Voice)
        .permissions(trigger.permission_overwrites.clone());

    if let Some(category_id) = category_id {
        builder = builder.category(category_id);
    }

    if profile.user_limit > 0 {
        builder = builder.user_limit(profile.user_limit as u32);
    }

    let Ok(channel) = guild_id.create_channel(&ctx.http, builder).await else {
        return;
    };

    if guild_id
        .move_member(&ctx.http, user.id, channel.id)
        .await
        .is_err()
    {
        let _ = channel.delete(&ctx.http).await;
        return;
    }

    let voice_mode = RoomMode::from_db(&profile.voice_mode).as_db().to_string();
    let user_limit = profile.user_limit.clamp(0, 99);
    let room_name = if base_name.is_empty() {
        None
    } else {
        Some(base_name.as_str())
    };

    let Ok(mut room) = db::create_tempvoc_room(
        &pool,
        settings.bot_id,
        settings.guild_id,
        channel.id.get() as i64,
        user.id.get() as i64,
        &voice_mode,
        profile.allow_micro,
        profile.allow_camera,
        profile.allow_soundboard,
        user_limit,
        room_name,
    )
    .await
    else {
        let _ = channel.delete(&ctx.http).await;
        return;
    };

    let _ = apply_room_permissions(ctx, &room).await;
    send_room_panel(ctx, &pool, &mut room).await;
}

async fn delete_temp_channel(ctx: &Context, channel_id: ChannelId) {
    if let Ok(channel) = channel_id.to_channel(&ctx.http).await {
        if let Channel::Guild(guild_channel) = channel {
            let _ = guild_channel.delete(&ctx.http).await;
        }
    }
}

async fn cached_room_members(ctx: &Context, guild_id: GuildId, channel_id: ChannelId) -> usize {
    ctx.cache
        .guild(guild_id)
        .map(|guild| {
            guild
                .voice_states
                .values()
                .filter(|state| state.channel_id == Some(channel_id))
                .count()
        })
        .unwrap_or(0)
}

async fn purge_room_members(
    ctx: &Context,
    guild_id: GuildId,
    channel_id: ChannelId,
    owner_id: u64,
    whitelist: &[u64],
) -> usize {
    let mut allowed = HashSet::new();
    allowed.insert(owner_id);
    for member_id in whitelist {
        allowed.insert(*member_id);
    }

    let mut to_disconnect = Vec::new();
    if let Some(guild) = ctx.cache.guild(guild_id) {
        for (user_id, voice_state) in &guild.voice_states {
            if voice_state.channel_id == Some(channel_id) && !allowed.contains(&user_id.get()) {
                to_disconnect.push(*user_id);
            }
        }
    }

    let mut disconnected = 0usize;
    for user_id in to_disconnect {
        if guild_id.disconnect_member(&ctx.http, user_id).await.is_ok() {
            disconnected += 1;
        }
    }

    disconnected
}

async fn handle_settings_component_interaction(
    ctx: &Context,
    component: &ComponentInteraction,
) -> bool {
    if !component.data.custom_id.starts_with(TEMPVOC_MENU) {
        return false;
    }

    let Some((action, owner_id)) = parse_owner_id(&component.data.custom_id) else {
        return false;
    };

    if component.user.id.get() != owner_id {
        respond_ephemeral_component(ctx, component, "Seul l'auteur du menu peut l'utiliser.").await;
        return true;
    }

    let Some(guild_id) = component.guild_id else {
        return true;
    };

    let Some(pool) = pool(ctx).await else {
        return true;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let settings = db::get_or_create_tempvoc_settings(&pool, bot_id, guild_id.get() as i64)
        .await
        .ok();

    let Some(settings) = settings else {
        return true;
    };

    if action.ends_with(":configure") {
        let modal = CreateModal::new(component.data.custom_id.clone(), "Configurer Tempvoc")
            .components(vec![
                CreateActionRow::InputText(
                    CreateInputText::new(
                        InputTextStyle::Short,
                        "Canal declencheur",
                        "trigger_channel_id",
                    )
                    .required(false),
                ),
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Categorie", "category_id")
                        .required(false),
                ),
            ]);

        let _ = component
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await;
        return true;
    }

    if action.ends_with(":toggle") {
        let new_settings = db::update_tempvoc_settings(
            &pool,
            bot_id,
            guild_id.get() as i64,
            settings.trigger_channel_id,
            settings.category_id,
            !settings.enabled,
        )
        .await
        .ok();

        if let Some(updated) = new_settings {
            let _ = component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .embed(tempvoc_embed(&updated))
                            .components(tempvoc_settings_components(component.user.id, &updated)),
                    ),
                )
                .await;
        }

        return true;
    }

    if action.ends_with(":refresh") {
        let _ = component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(tempvoc_embed(&settings))
                        .components(tempvoc_settings_components(component.user.id, &settings)),
                ),
            )
            .await;
        return true;
    }

    false
}

async fn handle_room_component_interaction(
    ctx: &Context,
    component: &ComponentInteraction,
) -> bool {
    let Some((action, channel_id)) =
        parse_scoped_channel_id(&component.data.custom_id, TEMPVOC_ROOM_SCOPE)
    else {
        return false;
    };

    let Some(pool) = pool(ctx).await else {
        return true;
    };

    let Some(mut room) = db::get_tempvoc_room_by_channel(&pool, channel_id.get() as i64)
        .await
        .ok()
        .flatten()
    else {
        respond_ephemeral_component(ctx, component, "Ce panel tempvoc n'est plus actif.").await;
        return true;
    };

    if component.user.id.get() as i64 != room.owner_id {
        respond_ephemeral_component(
            ctx,
            component,
            "Seul l'owner du vocal temporaire peut utiliser ce panel.",
        )
        .await;
        return true;
    }

    room.control_message_channel_id = Some(component.message.channel_id.get() as i64);
    room.control_message_id = Some(component.message.id.get() as i64);

    match action.as_str() {
        "open" | "closed" | "private" => {
            room.voice_mode = action.clone();

            if !persist_room_state(ctx, &pool, &mut room, None).await {
                respond_ephemeral_component(
                    ctx,
                    component,
                    "Impossible de mettre a jour ce vocal.",
                )
                .await;
                return true;
            }

            let _ = component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .embed(tempvoc_room_embed(&room, None))
                            .components(tempvoc_room_components(channel_id, &room)),
                    ),
                )
                .await;
            return true;
        }
        "whitelist" => {
            let modal = CreateModal::new(
                room_modal_id("whitelist", channel_id),
                "Modifier la whitelist",
            )
            .components(vec![CreateActionRow::InputText(
                CreateInputText::new(
                    InputTextStyle::Paragraph,
                    "Membres (mentions/IDs, vide pour effacer)",
                    MEMBER_LIST_INPUT_ID,
                )
                .required(false),
            )]);

            let _ = component
                .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
                .await;
            return true;
        }
        "blacklist" => {
            let modal = CreateModal::new(
                room_modal_id("blacklist", channel_id),
                "Modifier la blacklist",
            )
            .components(vec![CreateActionRow::InputText(
                CreateInputText::new(
                    InputTextStyle::Paragraph,
                    "Membres (mentions/IDs, vide pour effacer)",
                    MEMBER_LIST_INPUT_ID,
                )
                .required(false),
            )]);

            let _ = component
                .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
                .await;
            return true;
        }
        "purge" => {
            let whitelist = decode_member_list(&room.whitelist_json);
            let purged = purge_room_members(
                ctx,
                GuildId::new(room.guild_id as u64),
                channel_id,
                room.owner_id as u64,
                &whitelist,
            )
            .await;

            let notice = format!("{} membre(s) deconnecte(s).", purged);
            let _ = component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .embed(tempvoc_room_embed(&room, Some(&notice)))
                            .components(tempvoc_room_components(channel_id, &room)),
                    ),
                )
                .await;

            refresh_room_panel_message(ctx, &room, Some(&notice)).await;
            return true;
        }
        "micro" => {
            room.allow_micro = !room.allow_micro;
        }
        "camera" => {
            room.allow_camera = !room.allow_camera;
        }
        "soundboard" => {
            room.allow_soundboard = !room.allow_soundboard;
        }
        "transfer" => {
            let modal =
                CreateModal::new(room_modal_id("transfer", channel_id), "Transferer l'owner")
                    .components(vec![CreateActionRow::InputText(
                        CreateInputText::new(
                            InputTextStyle::Short,
                            "Nouveau owner (mention ou ID)",
                            TRANSFER_OWNER_INPUT_ID,
                        )
                        .required(true),
                    )]);

            let _ = component
                .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
                .await;
            return true;
        }
        "settings" => {
            let modal =
                CreateModal::new(room_modal_id("settings", channel_id), "Parametres du salon")
                    .components(vec![
                        CreateActionRow::InputText(
                            CreateInputText::new(
                                InputTextStyle::Short,
                                "Nom du salon",
                                SETTINGS_NAME_INPUT_ID,
                            )
                            .required(false),
                        ),
                        CreateActionRow::InputText(
                            CreateInputText::new(
                                InputTextStyle::Short,
                                "Limite (0 a 99)",
                                SETTINGS_LIMIT_INPUT_ID,
                            )
                            .required(false),
                        ),
                    ]);

            let _ = component
                .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
                .await;
            return true;
        }
        "save" => {
            let save_result = db::save_tempvoc_profile(
                &pool,
                room.bot_id,
                room.guild_id,
                room.owner_id,
                RoomMode::from_db(&room.voice_mode).as_db(),
                room.allow_micro,
                room.allow_camera,
                room.allow_soundboard,
                room.user_limit,
                room.room_name.as_deref(),
            )
            .await;

            let notice = if save_result.is_ok() {
                "Configuration sauvegardee comme profil par defaut."
            } else {
                "Echec de sauvegarde du profil par defaut."
            };

            let _ = component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .embed(tempvoc_room_embed(&room, Some(notice)))
                            .components(tempvoc_room_components(channel_id, &room)),
                    ),
                )
                .await;

            refresh_room_panel_message(ctx, &room, Some(notice)).await;
            return true;
        }
        _ => {
            return false;
        }
    }

    if !persist_room_state(ctx, &pool, &mut room, None).await {
        respond_ephemeral_component(ctx, component, "Impossible de mettre a jour ce vocal.").await;
        return true;
    }

    let _ = component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embed(tempvoc_room_embed(&room, None))
                    .components(tempvoc_room_components(channel_id, &room)),
            ),
        )
        .await;

    true
}

pub async fn handle_component_interaction(ctx: &Context, component: &ComponentInteraction) -> bool {
    if component.data.custom_id.starts_with(TEMPVOC_MENU) {
        return handle_settings_component_interaction(ctx, component).await;
    }

    if component
        .data
        .custom_id
        .starts_with(&format!("tempvoc:{}:", TEMPVOC_ROOM_SCOPE))
    {
        return handle_room_component_interaction(ctx, component).await;
    }

    false
}

async fn handle_settings_modal_interaction(ctx: &Context, modal: &ModalInteraction) -> bool {
    if !modal.data.custom_id.starts_with(TEMPVOC_MENU) {
        return false;
    }

    let Some((action, owner_id)) = parse_owner_id(&modal.data.custom_id) else {
        return false;
    };

    if modal.user.id.get() != owner_id {
        respond_ephemeral_modal(
            ctx,
            modal,
            "Seul l'auteur du menu peut soumettre ce formulaire.",
        )
        .await;
        return true;
    }

    if !action.contains(":configure") {
        return false;
    }

    let Some(guild_id) = modal.guild_id else {
        return true;
    };

    let Some(pool) = pool(ctx).await else {
        return true;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let trigger_channel_id =
        modal_value(modal, "trigger_channel_id").and_then(|value| value.trim().parse::<i64>().ok());
    let category_id =
        modal_value(modal, "category_id").and_then(|value| value.trim().parse::<i64>().ok());

    let updated = db::update_tempvoc_settings(
        &pool,
        bot_id,
        guild_id.get() as i64,
        trigger_channel_id,
        category_id,
        true,
    )
    .await
    .ok();

    if let Some(updated) = updated {
        let _ = modal
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .embed(tempvoc_embed(&updated))
                        .components(tempvoc_settings_components(modal.user.id, &updated))
                        .ephemeral(true),
                ),
            )
            .await;
    }

    true
}

async fn handle_room_modal_interaction(ctx: &Context, modal: &ModalInteraction) -> bool {
    let Some((action, channel_id)) =
        parse_scoped_channel_id(&modal.data.custom_id, TEMPVOC_MODAL_SCOPE)
    else {
        return false;
    };

    let Some(pool) = pool(ctx).await else {
        return true;
    };

    let Some(mut room) = db::get_tempvoc_room_by_channel(&pool, channel_id.get() as i64)
        .await
        .ok()
        .flatten()
    else {
        respond_ephemeral_modal(ctx, modal, "Ce vocal temporaire n'est plus actif.").await;
        return true;
    };

    if modal.user.id.get() as i64 != room.owner_id {
        respond_ephemeral_modal(
            ctx,
            modal,
            "Seul l'owner du vocal temporaire peut soumettre ce formulaire.",
        )
        .await;
        return true;
    }

    match action.as_str() {
        "whitelist" => {
            let raw = modal_value(modal, MEMBER_LIST_INPUT_ID).unwrap_or_default();
            room.whitelist_json = encode_member_list(&parse_user_ids_input(&raw));

            if !persist_room_state(ctx, &pool, &mut room, Some("Whitelist mise a jour.")).await {
                respond_ephemeral_modal(ctx, modal, "Impossible de mettre a jour la whitelist.")
                    .await;
                return true;
            }

            respond_ephemeral_modal(ctx, modal, "Whitelist mise a jour.").await;
            return true;
        }
        "blacklist" => {
            let raw = modal_value(modal, MEMBER_LIST_INPUT_ID).unwrap_or_default();
            room.blacklist_json = encode_member_list(&parse_user_ids_input(&raw));

            if !persist_room_state(ctx, &pool, &mut room, Some("Blacklist mise a jour.")).await {
                respond_ephemeral_modal(ctx, modal, "Impossible de mettre a jour la blacklist.")
                    .await;
                return true;
            }

            respond_ephemeral_modal(ctx, modal, "Blacklist mise a jour.").await;
            return true;
        }
        "transfer" => {
            let raw = modal_value(modal, TRANSFER_OWNER_INPUT_ID).unwrap_or_default();
            let Some(new_owner_id) = parse_user_ids_input(&raw).first().copied() else {
                respond_ephemeral_modal(ctx, modal, "Utilisateur invalide.").await;
                return true;
            };

            let guild_id = GuildId::new(room.guild_id as u64);
            if guild_id
                .member(&ctx.http, UserId::new(new_owner_id))
                .await
                .is_err()
            {
                respond_ephemeral_modal(ctx, modal, "Ce membre n'est pas present sur le serveur.")
                    .await;
                return true;
            }

            room.owner_id = new_owner_id as i64;
            if !persist_room_state(ctx, &pool, &mut room, Some("Owner transfere.")).await {
                respond_ephemeral_modal(ctx, modal, "Impossible de transferer l'owner.").await;
                return true;
            }

            respond_ephemeral_modal(
                ctx,
                modal,
                &format!("Owner transfere a <@{}>.", new_owner_id),
            )
            .await;
            return true;
        }
        "settings" => {
            let guild_id = GuildId::new(room.guild_id as u64);

            let room_name_input = modal_value(modal, SETTINGS_NAME_INPUT_ID).unwrap_or_default();
            let room_name_input = normalize_room_name(&room_name_input);
            if !room_name_input.is_empty() {
                let Some(unique_name) =
                    unique_voice_name(ctx, guild_id, &room_name_input, Some(channel_id)).await
                else {
                    respond_ephemeral_modal(ctx, modal, "Nom de salon invalide.").await;
                    return true;
                };

                if channel_id
                    .edit(&ctx.http, EditChannel::new().name(unique_name))
                    .await
                    .is_err()
                {
                    respond_ephemeral_modal(ctx, modal, "Impossible de renommer le vocal.").await;
                    return true;
                }

                room.room_name = Some(room_name_input);
            }

            let limit_input = modal_value(modal, SETTINGS_LIMIT_INPUT_ID).unwrap_or_default();
            if !limit_input.trim().is_empty() {
                let Ok(parsed_limit) = limit_input.trim().parse::<i32>() else {
                    respond_ephemeral_modal(
                        ctx,
                        modal,
                        "La limite doit etre un nombre entre 0 et 99.",
                    )
                    .await;
                    return true;
                };

                room.user_limit = parsed_limit.clamp(0, 99);
            }

            if !persist_room_state(ctx, &pool, &mut room, Some("Parametres mis a jour.")).await {
                respond_ephemeral_modal(ctx, modal, "Impossible d'appliquer les parametres.").await;
                return true;
            }

            respond_ephemeral_modal(ctx, modal, "Parametres appliques.").await;
            return true;
        }
        _ => {
            return false;
        }
    }
}

pub async fn handle_modal_interaction(ctx: &Context, modal: &ModalInteraction) -> bool {
    if modal.data.custom_id.starts_with(TEMPVOC_MENU) {
        return handle_settings_modal_interaction(ctx, modal).await;
    }

    if modal
        .data
        .custom_id
        .starts_with(&format!("tempvoc:{}:", TEMPVOC_MODAL_SCOPE))
    {
        return handle_room_modal_interaction(ctx, modal).await;
    }

    false
}

pub async fn handle_voice_state_update(ctx: &Context, old: Option<&VoiceState>, new: &VoiceState) {
    let Some(guild_id) = new.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let guild_id_i64 = guild_id.get() as i64;
    let settings = db::get_or_create_tempvoc_settings(&pool, bot_id, guild_id_i64)
        .await
        .ok();

    let Some(settings) = settings else {
        return;
    };

    let old_channel = old.and_then(|state| state.channel_id);
    let new_channel = new.channel_id;

    if settings.enabled
        && settings.trigger_channel_id.is_some()
        && new_channel.map(|channel| channel.get() as i64) == settings.trigger_channel_id
    {
        if let Ok(member) = guild_id.member(&ctx.http, new.user_id).await {
            create_temp_channel(ctx, guild_id, &member.user, &settings).await;
        }
    }

    if let Some(old_channel) = old_channel {
        if db::get_tempvoc_room_by_channel(&pool, old_channel.get() as i64)
            .await
            .ok()
            .flatten()
            .is_some()
            && cached_room_members(ctx, guild_id, old_channel).await == 0
        {
            delete_temp_channel(ctx, old_channel).await;
            let _ = db::delete_tempvoc_room(&pool, old_channel.get() as i64).await;
        }
    }
}

pub async fn cleanup_stale_rooms_on_ready(ctx: &Context) {
    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let Ok(rooms) = db::get_tempvoc_rooms_by_bot(&pool, bot_id).await else {
        return;
    };

    for room in rooms {
        let guild_id = GuildId::new(room.guild_id as u64);
        let channel_id = ChannelId::new(room.channel_id as u64);

        if channel_id.to_channel(&ctx.http).await.is_err() {
            let _ = db::delete_tempvoc_room(&pool, room.channel_id).await;
            continue;
        }

        let Some(_) = ctx.cache.guild(guild_id) else {
            continue;
        };

        if cached_room_members(ctx, guild_id, channel_id).await == 0 {
            delete_temp_channel(ctx, channel_id).await;
            let _ = db::delete_tempvoc_room(&pool, room.channel_id).await;
        }
    }
}

pub struct TempvocCommand;
pub static COMMAND_DESCRIPTOR: TempvocCommand = TempvocCommand;

impl crate::commands::command_contract::CommandSpec for TempvocCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "tempvoc",
            category: "channel",
            params: "[cmd]",
            description: "Affiche le menu de configuration du systeme de vocaux temporaires.",
            examples: &["+tempvoc", "+tempvoccmd", "+help tempvoc"],
            default_aliases: &[],
            allow_in_dm: false,
            default_permission: 5,
        }
    }
}
