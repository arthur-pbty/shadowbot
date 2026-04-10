use serenity::builder::{
    CreateActionRow, CreateButton, CreateEmbed, CreateInputText, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, CreateModal,
};
use serenity::model::application::{
    ActionRowComponent, ButtonStyle, ComponentInteraction, InputTextStyle, ModalInteraction,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_channel_id, send_embed, theme_color};
use crate::commands::logs_service;
use crate::db::DbPoolKey;

const BOOSTEMBED_MENU: &str = "boostembed:settings";

#[derive(Clone)]
struct BoostEmbedSettings {
    enabled: bool,
    title: Option<String>,
    description: Option<String>,
    color: Option<i32>,
    boost_channel_id: Option<i64>,
    boost_channel_enabled: bool,
}

fn default_settings() -> BoostEmbedSettings {
    BoostEmbedSettings {
        enabled: true,
        title: None,
        description: None,
        color: None,
        boost_channel_id: None,
        boost_channel_enabled: false,
    }
}

fn parse_owner_id(custom_id: &str) -> Option<(String, u64)> {
    let mut parts = custom_id.rsplitn(2, ':');
    let owner = parts.next()?.parse::<u64>().ok()?;
    let action = parts.next()?.to_string();
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

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

async fn ensure_boost_embed_row(pool: &sqlx::PgPool, bot_id: UserId, guild_id: GuildId) {
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
    .execute(pool)
    .await;
}

async fn set_boost_embed_enabled(
    pool: &sqlx::PgPool,
    bot_id: UserId,
    guild_id: GuildId,
    enabled: bool,
) {
    let _ = sqlx::query(
        r#"
        INSERT INTO bot_boost_embed (bot_id, guild_id, enabled)
        VALUES ($1, $2, $3)
        ON CONFLICT (bot_id, guild_id)
        DO UPDATE SET enabled = EXCLUDED.enabled, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(enabled)
    .execute(pool)
    .await;
}

async fn set_boost_log_channel(
    pool: &sqlx::PgPool,
    bot_id: UserId,
    guild_id: GuildId,
    channel_id: Option<ChannelId>,
    enabled: bool,
) {
    let _ = sqlx::query(
        r#"
        INSERT INTO bot_log_channels (bot_id, guild_id, log_type, channel_id, enabled)
        VALUES ($1, $2, 'boost', $3, $4)
        ON CONFLICT (bot_id, guild_id, log_type)
        DO UPDATE SET channel_id = EXCLUDED.channel_id, enabled = EXCLUDED.enabled, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(channel_id.map(|c| c.get() as i64))
    .bind(enabled)
    .execute(pool)
    .await;
}

async fn read_settings(
    pool: &sqlx::PgPool,
    bot_id: UserId,
    guild_id: GuildId,
) -> BoostEmbedSettings {
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
    .fetch_optional(pool)
    .await
    .ok()
    .flatten();

    let channel_row = sqlx::query_as::<_, (Option<i64>, bool)>(
        r#"
        SELECT channel_id, enabled
        FROM bot_log_channels
        WHERE bot_id = $1 AND guild_id = $2 AND log_type = 'boost'
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten();

    let mut settings = default_settings();
    if let Some((enabled, title, description, color)) = row {
        settings.enabled = enabled;
        settings.title = title;
        settings.description = description;
        settings.color = color;
    }

    if let Some((channel_id, enabled)) = channel_row {
        settings.boost_channel_id = channel_id;
        settings.boost_channel_enabled = enabled;
    }

    settings
}

fn settings_embed(settings: &BoostEmbedSettings) -> CreateEmbed {
    let channel_text = if settings.boost_channel_enabled {
        settings
            .boost_channel_id
            .map(|id| format!("<#{}>", id))
            .unwrap_or_else(|| "activé mais salon non défini".to_string())
    } else {
        "désactivé".to_string()
    };

    CreateEmbed::new()
        .title("Configuration Boost Embed")
        .description("Utilise les boutons/modals ci-dessous pour paramétrer l'embed de boost et son salon d'envoi.")
        .field("Embed", if settings.enabled { "on" } else { "off" }, true)
        .field("Salon d'envoi boost", channel_text, true)
        .field(
            "Titre",
            settings
                .title
                .clone()
                .unwrap_or_else(|| "(défaut)".to_string()),
            false,
        )
        .field(
            "Description",
            settings
                .description
                .clone()
                .unwrap_or_else(|| "(défaut)".to_string()),
            false,
        )
        .field(
            "Couleur",
            settings
                .color
                .map(|v| format!("#{:06X}", v.max(0) as u32))
                .unwrap_or_else(|| "(défaut)".to_string()),
            true,
        )
        .color(0xF47FFF)
}

fn settings_components(owner_id: UserId, settings: &BoostEmbedSettings) -> Vec<CreateActionRow> {
    let toggle_style = if settings.enabled {
        ButtonStyle::Danger
    } else {
        ButtonStyle::Success
    };

    vec![
        CreateActionRow::Buttons(vec![
            CreateButton::new(format!("{}:toggle:{}", BOOSTEMBED_MENU, owner_id.get()))
                .label(if settings.enabled {
                    "Désactiver embed"
                } else {
                    "Activer embed"
                })
                .style(toggle_style),
            CreateButton::new(format!("{}:test:{}", BOOSTEMBED_MENU, owner_id.get()))
                .label("Envoyer test")
                .style(ButtonStyle::Primary),
            CreateButton::new(format!("{}:refresh:{}", BOOSTEMBED_MENU, owner_id.get()))
                .label("Rafraîchir")
                .style(ButtonStyle::Secondary),
        ]),
        CreateActionRow::Buttons(vec![
            CreateButton::new(format!("{}:set_here:{}", BOOSTEMBED_MENU, owner_id.get()))
                .label("Salon = ici")
                .style(ButtonStyle::Success),
            CreateButton::new(format!(
                "{}:edit_channel:{}",
                BOOSTEMBED_MENU,
                owner_id.get()
            ))
            .label("Définir salon")
            .style(ButtonStyle::Secondary),
            CreateButton::new(format!(
                "{}:disable_channel:{}",
                BOOSTEMBED_MENU,
                owner_id.get()
            ))
            .label("Couper envoi")
            .style(ButtonStyle::Danger),
        ]),
        CreateActionRow::Buttons(vec![
            CreateButton::new(format!("{}:edit_title:{}", BOOSTEMBED_MENU, owner_id.get()))
                .label("Modifier titre")
                .style(ButtonStyle::Secondary),
            CreateButton::new(format!(
                "{}:edit_description:{}",
                BOOSTEMBED_MENU,
                owner_id.get()
            ))
            .label("Modifier description")
            .style(ButtonStyle::Secondary),
            CreateButton::new(format!("{}:edit_color:{}", BOOSTEMBED_MENU, owner_id.get()))
                .label("Modifier couleur")
                .style(ButtonStyle::Secondary),
        ]),
    ]
}

async fn show_panel(
    ctx: &Context,
    msg: &Message,
    pool: &sqlx::PgPool,
    bot_id: UserId,
    guild_id: GuildId,
) {
    ensure_boost_embed_row(pool, bot_id, guild_id).await;
    let settings = read_settings(pool, bot_id, guild_id).await;
    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .embed(settings_embed(&settings))
                .components(settings_components(msg.author.id, &settings)),
        )
        .await;
}

pub async fn handle_boostembed(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("BoostEmbed")
                .description("DB indisponible.")
                .color(0xED4245),
        )
        .await;
        return;
    };

    let bot_id = ctx.cache.current_user().id;

    if let Some(action) = args.first().map(|v| v.to_lowercase()) {
        match action.as_str() {
            "on" | "off" => {
                set_boost_embed_enabled(&pool, bot_id, guild_id, action == "on").await;
                send_embed(
                    ctx,
                    msg,
                    CreateEmbed::new()
                        .title("BoostEmbed")
                        .description(if action == "on" {
                            "Activé."
                        } else {
                            "Désactivé."
                        })
                        .color(theme_color(ctx).await),
                )
                .await;
                return;
            }
            "test" => {
                logs_service::send_boost_embed(ctx, guild_id, &msg.author).await;
                send_embed(
                    ctx,
                    msg,
                    CreateEmbed::new()
                        .title("BoostEmbed")
                        .description("Test envoyé.")
                        .color(theme_color(ctx).await),
                )
                .await;
                return;
            }
            "settings" | "panel" => {
                show_panel(ctx, msg, &pool, bot_id, guild_id).await;
                return;
            }
            _ => {
                send_embed(
                    ctx,
                    msg,
                    CreateEmbed::new()
                        .title("BoostEmbed")
                        .description("Usage: +boostembed [on|off|test|settings]")
                        .color(0xED4245),
                )
                .await;
                return;
            }
        }
    }

    show_panel(ctx, msg, &pool, bot_id, guild_id).await;
}

pub async fn handle_component_interaction(ctx: &Context, component: &ComponentInteraction) -> bool {
    if !component.data.custom_id.starts_with(BOOSTEMBED_MENU) {
        return false;
    }

    let Some((action, owner_id)) = parse_owner_id(&component.data.custom_id) else {
        return false;
    };

    if component.user.id.get() != owner_id {
        let _ = component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Seul l'auteur du panneau peut l'utiliser.")
                        .ephemeral(true),
                ),
            )
            .await;
        return true;
    }

    let Some(guild_id) = component.guild_id else {
        return true;
    };

    let Some(pool) = pool(ctx).await else {
        let _ = component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("DB indisponible.")
                        .ephemeral(true),
                ),
            )
            .await;
        return true;
    };

    let bot_id = ctx.cache.current_user().id;
    ensure_boost_embed_row(&pool, bot_id, guild_id).await;

    if action.ends_with(":edit_title") {
        let modal = CreateModal::new(
            format!(
                "{}:modal:title:{}",
                BOOSTEMBED_MENU,
                component.user.id.get()
            ),
            "Modifier le titre du boost embed",
        )
        .components(vec![CreateActionRow::InputText(
            CreateInputText::new(InputTextStyle::Short, "Titre (vide = défaut)", "title")
                .required(false),
        )]);

        let _ = component
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await;
        return true;
    }

    if action.ends_with(":edit_description") {
        let modal = CreateModal::new(
            format!(
                "{}:modal:description:{}",
                BOOSTEMBED_MENU,
                component.user.id.get()
            ),
            "Modifier la description du boost embed",
        )
        .components(vec![CreateActionRow::InputText(
            CreateInputText::new(
                InputTextStyle::Paragraph,
                "Description (vide = défaut)",
                "description",
            )
            .required(false),
        )]);

        let _ = component
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await;
        return true;
    }

    if action.ends_with(":edit_color") {
        let modal = CreateModal::new(
            format!(
                "{}:modal:color:{}",
                BOOSTEMBED_MENU,
                component.user.id.get()
            ),
            "Modifier la couleur du boost embed",
        )
        .components(vec![CreateActionRow::InputText(
            CreateInputText::new(InputTextStyle::Short, "Couleur hex (#FF66CC)", "color")
                .required(false),
        )]);

        let _ = component
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await;
        return true;
    }

    if action.ends_with(":edit_channel") {
        let modal = CreateModal::new(
            format!(
                "{}:modal:channel:{}",
                BOOSTEMBED_MENU,
                component.user.id.get()
            ),
            "Définir le salon boost",
        )
        .components(vec![CreateActionRow::InputText(
            CreateInputText::new(
                InputTextStyle::Short,
                "Salon (#mention ou ID, vide = désactiver)",
                "channel",
            )
            .required(false),
        )]);

        let _ = component
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await;
        return true;
    }

    if action.ends_with(":toggle") {
        let settings = read_settings(&pool, bot_id, guild_id).await;
        set_boost_embed_enabled(&pool, bot_id, guild_id, !settings.enabled).await;
    } else if action.ends_with(":set_here") {
        set_boost_log_channel(&pool, bot_id, guild_id, Some(component.channel_id), true).await;
    } else if action.ends_with(":disable_channel") {
        set_boost_log_channel(&pool, bot_id, guild_id, None, false).await;
    } else if action.ends_with(":test") {
        logs_service::send_boost_embed(ctx, guild_id, &component.user).await;
    }

    let settings = read_settings(&pool, bot_id, guild_id).await;
    let _ = component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embed(settings_embed(&settings))
                    .components(settings_components(component.user.id, &settings)),
            ),
        )
        .await;

    true
}

pub async fn handle_modal_interaction(ctx: &Context, modal: &ModalInteraction) -> bool {
    if !modal
        .data
        .custom_id
        .starts_with(&format!("{}:modal:", BOOSTEMBED_MENU))
    {
        return false;
    }

    let Some((action, owner_id)) = parse_owner_id(&modal.data.custom_id) else {
        return false;
    };

    if modal.user.id.get() != owner_id {
        let _ = modal
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Seul l'auteur du panneau peut l'utiliser.")
                        .ephemeral(true),
                ),
            )
            .await;
        return true;
    }

    let Some(guild_id) = modal.guild_id else {
        return true;
    };

    let Some(pool) = pool(ctx).await else {
        let _ = modal
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("DB indisponible.")
                        .ephemeral(true),
                ),
            )
            .await;
        return true;
    };

    let bot_id = ctx.cache.current_user().id;
    ensure_boost_embed_row(&pool, bot_id, guild_id).await;

    if action.ends_with(":modal:title") {
        let title = modal_value(modal, "title").unwrap_or_default();
        let title_value = if title.trim().is_empty() {
            None
        } else {
            Some(title)
        };

        let _ = sqlx::query(
            "UPDATE bot_boost_embed SET title = $3, updated_at = NOW() WHERE bot_id = $1 AND guild_id = $2",
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(title_value)
        .execute(&pool)
        .await;
    } else if action.ends_with(":modal:description") {
        let description = modal_value(modal, "description").unwrap_or_default();
        let desc_value = if description.trim().is_empty() {
            None
        } else {
            Some(description)
        };

        let _ = sqlx::query(
            "UPDATE bot_boost_embed SET description = $3, updated_at = NOW() WHERE bot_id = $1 AND guild_id = $2",
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(desc_value)
        .execute(&pool)
        .await;
    } else if action.ends_with(":modal:color") {
        let raw = modal_value(modal, "color").unwrap_or_default();
        let color_value = if raw.trim().is_empty() {
            None
        } else {
            let normalized = raw.trim().trim_start_matches('#').trim_start_matches("0x");
            match u32::from_str_radix(normalized, 16) {
                Ok(value) => Some(value as i32),
                Err(_) => {
                    let _ = modal
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .content("Couleur invalide. Exemple: `#FF66CC`")
                                    .ephemeral(true),
                            ),
                        )
                        .await;
                    return true;
                }
            }
        };

        let _ = sqlx::query(
            "UPDATE bot_boost_embed SET color = $3, updated_at = NOW() WHERE bot_id = $1 AND guild_id = $2",
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(color_value)
        .execute(&pool)
        .await;
    } else if action.ends_with(":modal:channel") {
        let raw = modal_value(modal, "channel").unwrap_or_default();
        if raw.trim().is_empty() {
            set_boost_log_channel(&pool, bot_id, guild_id, None, false).await;
        } else if let Some(channel) = parse_channel_id(&raw) {
            set_boost_log_channel(&pool, bot_id, guild_id, Some(channel), true).await;
        } else {
            let _ = modal
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("Salon invalide. Donne une mention `#salon` ou un ID.")
                            .ephemeral(true),
                    ),
                )
                .await;
            return true;
        }
    }

    let _ = modal
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Configuration boost embed mise à jour. Clique sur `Rafraîchir` dans le panneau.")
                    .ephemeral(true),
            ),
        )
        .await;

    true
}

pub struct BoostembedCommand;
pub static COMMAND_DESCRIPTOR: BoostembedCommand = BoostembedCommand;

impl crate::commands::command_contract::CommandSpec for BoostembedCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "boostembed",
            category: "logs",
            params: "[on|off|test|settings]",
            description: "Ouvre un panneau avec composants pour paramétrer l'embed boost et le salon où il est envoyé.",
            examples: &["+boostembed", "+boostembed settings", "+boostembed test"],
            default_aliases: &["bembed"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
