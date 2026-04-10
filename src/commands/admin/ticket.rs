use chrono::Utc;
use serenity::all::{PermissionOverwrite, PermissionOverwriteType, Permissions};
use serenity::builder::{
    CreateActionRow, CreateButton, CreateChannel, CreateEmbed, CreateInputText,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateModal,
};
use serenity::model::Colour;
use serenity::model::application::{
    ActionRowComponent, ButtonStyle, ComponentInteraction, InputTextStyle, ModalInteraction,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::db;

const TICKET_MENU: &str = "ticket:settings";

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

fn sanitize_channel_name(input: &str) -> String {
    let mut out = String::new();
    let mut previous_dash = false;

    for ch in input.to_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            previous_dash = false;
        } else if (ch.is_whitespace() || ch == '-' || ch == '_') && !previous_dash {
            out.push('-');
            previous_dash = true;
        }
    }

    out.trim_matches('-').to_string()
}

fn ticket_embed(settings: &db::TicketSettings) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title("Gestion des tickets")
        .description("Utilise les boutons ci-dessous pour gérer le système de tickets.")
        .colour(Colour::from_rgb(90, 160, 255))
        .timestamp(Utc::now())
        .field(
            "Statut",
            if settings.enabled { "Actif" } else { "Inactif" },
            true,
        );

    if let Some(category_id) = settings.category_id {
        embed = embed.field("Catégorie", format!("<#{}>", category_id), true);
    }

    if let Some(log_channel_id) = settings.log_channel_id {
        embed = embed.field("Logs", format!("<#{}>", log_channel_id), true);
    }

    embed
}

fn ticket_components(owner_id: UserId, settings: &db::TicketSettings) -> Vec<CreateActionRow> {
    let toggle_label = if settings.enabled {
        "Désactiver"
    } else {
        "Activer"
    };

    vec![CreateActionRow::Buttons(vec![
        CreateButton::new(format!("{}:create:{}", TICKET_MENU, owner_id.get()))
            .label("Créer")
            .style(ButtonStyle::Success),
        CreateButton::new(format!("{}:configure:{}", TICKET_MENU, owner_id.get()))
            .label("Configurer")
            .style(ButtonStyle::Secondary),
        CreateButton::new(format!("{}:toggle:{}", TICKET_MENU, owner_id.get()))
            .label(toggle_label)
            .style(ButtonStyle::Primary),
        CreateButton::new(format!("{}:refresh:{}", TICKET_MENU, owner_id.get()))
            .label("Rafraîchir")
            .style(ButtonStyle::Secondary),
    ])]
}

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<db::DbPoolKey>().cloned()
}

async fn current_settings(ctx: &Context, guild_id: GuildId) -> Option<db::TicketSettings> {
    let pool = pool(ctx).await?;
    let bot_id = ctx.cache.current_user().id.get() as i64;
    db::get_or_create_ticket_settings(&pool, bot_id, guild_id.get() as i64)
        .await
        .ok()
}

async fn show_menu(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(settings) = current_settings(ctx, guild_id).await else {
        return;
    };

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .embed(ticket_embed(&settings))
                .components(ticket_components(msg.author.id, &settings)),
        )
        .await;
}

async fn create_ticket_channel(
    ctx: &Context,
    guild_id: GuildId,
    creator: UserId,
    title: String,
    settings: &db::TicketSettings,
) -> Result<ChannelId, String> {
    let pool = pool(ctx)
        .await
        .ok_or_else(|| "Base de données indisponible".to_string())?;

    let name = sanitize_channel_name(&title);
    if name.is_empty() {
        return Err("Nom de ticket invalide".to_string());
    }

    let mut builder = CreateChannel::new(format!("ticket-{}", name))
        .kind(ChannelType::Text)
        .permissions(vec![
            PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::VIEW_CHANNEL.union(
                    Permissions::SEND_MESSAGES
                        | Permissions::READ_MESSAGE_HISTORY
                        | Permissions::ATTACH_FILES
                        | Permissions::EMBED_LINKS,
                ),
                kind: PermissionOverwriteType::Role(RoleId::new(guild_id.get())),
            },
            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL
                    | Permissions::SEND_MESSAGES
                    | Permissions::READ_MESSAGE_HISTORY
                    | Permissions::ATTACH_FILES
                    | Permissions::EMBED_LINKS
                    | Permissions::ADD_REACTIONS,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(creator),
            },
        ]);

    if let Some(category_id) = settings.category_id {
        builder = builder.category(ChannelId::new(category_id as u64));
    }

    let channel = guild_id
        .create_channel(&ctx.http, builder)
        .await
        .map_err(|e| format!("Impossible de créer le salon: {e}"))?;

    let _ = db::create_ticket(
        &pool,
        settings.bot_id,
        settings.guild_id,
        channel.id.get() as i64,
        creator.get() as i64,
        title,
    )
    .await;

    Ok(channel.id)
}

pub async fn handle_ticket_settings(ctx: &Context, msg: &Message, _args: &[&str]) {
    show_menu(ctx, msg).await;
}

pub async fn handle_component_interaction(ctx: &Context, component: &ComponentInteraction) -> bool {
    if !component.data.custom_id.starts_with(TICKET_MENU) {
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
                        .content("Seul l'auteur du menu peut l'utiliser.")
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
        return true;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let settings = db::get_or_create_ticket_settings(&pool, bot_id, guild_id.get() as i64)
        .await
        .ok();

    let Some(settings) = settings else {
        return true;
    };

    if action.ends_with(":refresh") {
        let _ = component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(ticket_embed(&settings))
                        .components(ticket_components(component.user.id, &settings)),
                ),
            )
            .await;
        return true;
    }

    if action.ends_with(":toggle") {
        if let Ok(updated) = db::update_ticket_settings(
            &pool,
            bot_id,
            guild_id.get() as i64,
            settings.category_id,
            settings.log_channel_id,
            !settings.enabled,
        )
        .await
        {
            let _ = component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .embed(ticket_embed(&updated))
                            .components(ticket_components(component.user.id, &updated)),
                    ),
                )
                .await;
        }
        return true;
    }

    if action.ends_with(":configure") {
        let modal = CreateModal::new(component.data.custom_id.clone(), "Configurer les tickets")
            .components(vec![
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Catégorie", "category_id")
                        .required(false),
                ),
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Salon de logs", "log_channel_id")
                        .required(false),
                ),
            ]);

        let _ = component
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await;
        return true;
    }

    if action.ends_with(":create") {
        let modal = CreateModal::new(component.data.custom_id.clone(), "Créer un ticket")
            .components(vec![CreateActionRow::InputText(
                CreateInputText::new(InputTextStyle::Short, "Nom du ticket", "ticket_title")
                    .required(true)
                    .max_length(100),
            )]);

        let _ = component
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await;
        return true;
    }

    false
}

pub async fn handle_modal_interaction(ctx: &Context, modal: &ModalInteraction) -> bool {
    if !modal.data.custom_id.starts_with(TICKET_MENU) {
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
                        .content("Seul l'auteur du menu peut soumettre ce formulaire.")
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
        return true;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let current = db::get_or_create_ticket_settings(&pool, bot_id, guild_id.get() as i64)
        .await
        .ok();

    let Some(settings) = current else {
        return true;
    };

    if action.ends_with(":configure") {
        let category_id =
            modal_value(modal, "category_id").and_then(|value| value.trim().parse::<i64>().ok());
        let log_channel_id =
            modal_value(modal, "log_channel_id").and_then(|value| value.trim().parse::<i64>().ok());

        if let Ok(updated) = db::update_ticket_settings(
            &pool,
            bot_id,
            guild_id.get() as i64,
            category_id,
            log_channel_id,
            settings.enabled,
        )
        .await
        {
            let _ = modal
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .embed(ticket_embed(&updated))
                            .components(ticket_components(modal.user.id, &updated))
                            .ephemeral(true),
                    ),
                )
                .await;
        }

        return true;
    }

    if action.ends_with(":create") {
        let title = modal_value(modal, "ticket_title").unwrap_or_default();
        let title = title.trim().to_string();

        if title.is_empty() {
            let _ = modal
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("Nom de ticket invalide.")
                            .ephemeral(true),
                    ),
                )
                .await;
            return true;
        }

        match create_ticket_channel(ctx, guild_id, modal.user.id, title.clone(), &settings).await {
            Ok(channel_id) => {
                let _ = modal
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .embed(
                                    CreateEmbed::new()
                                        .title("Ticket créé")
                                        .description(format!("Salon: <#{}>", channel_id.get()))
                                        .colour(Colour::from_rgb(0, 200, 120))
                                        .timestamp(Utc::now()),
                                )
                                .ephemeral(true),
                        ),
                    )
                    .await;
            }
            Err(error) => {
                let _ = modal
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content(error)
                                .ephemeral(true),
                        ),
                    )
                    .await;
            }
        }

        return true;
    }

    false
}

pub struct TicketCommand;
pub static COMMAND_DESCRIPTOR: TicketCommand = TicketCommand;

impl crate::commands::command_contract::CommandSpec for TicketCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "ticket",
            category: "admin",
            params: "settings",
            summary: "Ouvre la gestion des tickets",
            description: "Affiche le menu de configuration du systeme de tickets.",
            examples: &["+ticket", "+help ticket"],
            default_aliases: &[],
            default_permission: 8,
        }
    }
}
