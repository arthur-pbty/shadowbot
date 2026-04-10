use chrono::Utc;
use serenity::builder::{
    CreateActionRow, CreateButton, CreateEmbed, CreateInputText, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, CreateModal, EditMessage,
};
use serenity::model::application::{
    ActionRowComponent, ButtonKind, ButtonStyle, ComponentInteraction, InputTextStyle,
    ModalInteraction,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::theme_color;

const ROLEMENU_PANEL_PREFIX: &str = "rolemenu:panel";
const ROLEMENU_MODAL_PREFIX: &str = "rolemenu:modal";
const ROLEMENU_TOGGLE_PREFIX: &str = "rolemenu:toggle";

#[derive(Clone, Copy)]
struct RoleMenuTarget {
    owner_id: u64,
    channel_id: u64,
    message_id: u64,
}

fn panel_custom_id(action: &str, target: RoleMenuTarget) -> String {
    format!(
        "{}:{}:{}:{}:{}",
        ROLEMENU_PANEL_PREFIX, action, target.owner_id, target.channel_id, target.message_id
    )
}

fn modal_custom_id(action: &str, target: RoleMenuTarget) -> String {
    format!(
        "{}:{}:{}:{}:{}",
        ROLEMENU_MODAL_PREFIX, action, target.owner_id, target.channel_id, target.message_id
    )
}

fn parse_target_id(prefix_kind: &str, custom_id: &str) -> Option<(String, RoleMenuTarget)> {
    let parts = custom_id.split(':').collect::<Vec<_>>();
    if parts.len() != 6 {
        return None;
    }

    if parts[0] != "rolemenu" || parts[1] != prefix_kind {
        return None;
    }

    let action = parts[2].to_string();
    let owner_id = parts[3].parse::<u64>().ok()?;
    let channel_id = parts[4].parse::<u64>().ok()?;
    let message_id = parts[5].parse::<u64>().ok()?;

    Some((
        action,
        RoleMenuTarget {
            owner_id,
            channel_id,
            message_id,
        },
    ))
}

fn parse_panel_custom_id(custom_id: &str) -> Option<(String, RoleMenuTarget)> {
    parse_target_id("panel", custom_id)
}

fn parse_modal_custom_id(custom_id: &str) -> Option<(String, RoleMenuTarget)> {
    parse_target_id("modal", custom_id)
}

fn parse_toggle_role_id(custom_id: &str) -> Option<RoleId> {
    let mut parts = custom_id.split(':');
    let root = parts.next()?;
    let action = parts.next()?;
    let role_id = parts.next()?.parse::<u64>().ok()?;
    if root != "rolemenu" || action != "toggle" || parts.next().is_some() {
        return None;
    }

    Some(RoleId::new(role_id))
}

fn parse_role_id_input(raw: &str) -> Option<RoleId> {
    let cleaned = raw
        .trim()
        .trim_start_matches("<@&")
        .trim_end_matches('>');
    cleaned.parse::<u64>().ok().map(RoleId::new)
}

fn parse_button_style(raw: &str) -> ButtonStyle {
    match raw.trim().to_lowercase().as_str() {
        "primary" | "blue" | "bleu" => ButtonStyle::Primary,
        "success" | "green" | "vert" => ButtonStyle::Success,
        "danger" | "red" | "rouge" => ButtonStyle::Danger,
        _ => ButtonStyle::Secondary,
    }
}

fn parse_hex_color(raw: &str) -> Option<u32> {
    let mut value = raw.trim();
    if value.is_empty() {
        return None;
    }

    if let Some(stripped) = value.strip_prefix('#') {
        value = stripped;
    }
    if let Some(stripped) = value.strip_prefix("0x") {
        value = stripped;
    }

    if value.is_empty() {
        return None;
    }

    u32::from_str_radix(value, 16).ok()
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

fn panel_components(target: RoleMenuTarget) -> Vec<CreateActionRow> {
    vec![CreateActionRow::Buttons(vec![
        CreateButton::new(panel_custom_id("embed", target))
            .label("Configurer l'embed")
            .style(ButtonStyle::Primary),
        CreateButton::new(panel_custom_id("addrole", target))
            .label("Ajouter bouton role")
            .style(ButtonStyle::Success),
        CreateButton::new(panel_custom_id("refresh", target))
            .label("Rafraichir")
            .style(ButtonStyle::Secondary),
    ])]
}

fn panel_embed(target: RoleMenuTarget) -> CreateEmbed {
    CreateEmbed::new()
        .title("RoleMenu")
        .description("Panneau interactif pour configurer un menu de roles.")
        .field("Message cible", target.message_id.to_string(), true)
        .field("Canal", format!("<#{}>", target.channel_id), true)
        .timestamp(Utc::now())
}

fn default_menu_embed(color: u32) -> CreateEmbed {
    CreateEmbed::new()
        .title("Menu de roles")
        .description("Cliquez sur les boutons ci-dessous pour recevoir ou retirer un role.")
        .color(color)
        .timestamp(Utc::now())
}

fn collect_message_buttons(message: &Message) -> Vec<CreateButton> {
    let mut buttons = Vec::new();

    for row in &message.components {
        for component in &row.components {
            if let ActionRowComponent::Button(button) = component {
                buttons.push(CreateButton::from(button.clone()));
            }
        }
    }

    buttons
}

fn message_has_role_button(message: &Message, role_id: RoleId) -> bool {
    let wanted = format!("{}:{}", ROLEMENU_TOGGLE_PREFIX, role_id.get());

    for row in &message.components {
        for component in &row.components {
            if let ActionRowComponent::Button(button) = component {
                if let ButtonKind::NonLink {
                    custom_id, ..
                } = &button.data
                {
                    if custom_id == &wanted {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn button_rows(buttons: &[CreateButton]) -> Vec<CreateActionRow> {
    buttons
        .chunks(5)
        .map(|chunk| CreateActionRow::Buttons(chunk.to_vec()))
        .collect()
}

fn parse_message_id_arg(raw: &str) -> Option<MessageId> {
    raw.trim().parse::<u64>().ok().map(MessageId::new)
}

async fn fetch_target_message(ctx: &Context, target: RoleMenuTarget) -> Option<Message> {
    ChannelId::new(target.channel_id)
        .message(&ctx.http, MessageId::new(target.message_id))
        .await
        .ok()
}

async fn respond_component_ephemeral(ctx: &Context, component: &ComponentInteraction, text: &str) {
    let _ = component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(text)
                    .ephemeral(true),
            ),
        )
        .await;
}

async fn respond_modal_ephemeral(ctx: &Context, modal: &ModalInteraction, text: &str) {
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
}

pub async fn handle_rolemenu(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(_guild_id) = msg.guild_id else {
        return;
    };

    let color = theme_color(ctx).await;

    let target_message = if let Some(raw_id) = args.first() {
        let Some(message_id) = parse_message_id_arg(raw_id) else {
            let _ = msg
                .channel_id
                .send_message(
                    &ctx.http,
                    CreateMessage::new().embed(
                        CreateEmbed::new()
                            .title("RoleMenu")
                            .description("ID invalide. Utilisation: +rolemenu [message_id]")
                            .color(color),
                    ),
                )
                .await;
            return;
        };

        let Ok(existing) = msg.channel_id.message(&ctx.http, message_id).await else {
            let _ = msg
                .channel_id
                .send_message(
                    &ctx.http,
                    CreateMessage::new().embed(
                        CreateEmbed::new()
                            .title("RoleMenu")
                            .description("Message introuvable dans ce salon.")
                            .color(color),
                    ),
                )
                .await;
            return;
        };

        existing
    } else {
        let created = msg
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().embed(default_menu_embed(color)),
            )
            .await;

        let Ok(created) = created else {
            return;
        };

        created
    };

    let target = RoleMenuTarget {
        owner_id: msg.author.id.get(),
        channel_id: msg.channel_id.get(),
        message_id: target_message.id.get(),
    };

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .embed(panel_embed(target).color(color))
                .components(panel_components(target)),
        )
        .await;
}

pub async fn handle_component_interaction(ctx: &Context, component: &ComponentInteraction) -> bool {
    if let Some(role_id) = parse_toggle_role_id(&component.data.custom_id) {
        let Some(guild_id) = component.guild_id else {
            respond_component_ephemeral(ctx, component, "Interaction indisponible hors serveur.")
                .await;
            return true;
        };

        let Ok(member) = guild_id.member(&ctx.http, component.user.id).await else {
            respond_component_ephemeral(ctx, component, "Membre introuvable.").await;
            return true;
        };

        let has_role = member.roles.contains(&role_id);
        let updated = if has_role {
            member.remove_role(&ctx.http, role_id).await.is_ok()
        } else {
            member.add_role(&ctx.http, role_id).await.is_ok()
        };

        let text = if updated {
            if has_role {
                format!("Role <@&{}> retire.", role_id.get())
            } else {
                format!("Role <@&{}> ajoute.", role_id.get())
            }
        } else {
            "Impossible de mettre a jour ce role (permissions insuffisantes ?).".to_string()
        };

        respond_component_ephemeral(ctx, component, &text).await;
        return true;
    }

    if !component.data.custom_id.starts_with(ROLEMENU_PANEL_PREFIX) {
        return false;
    }

    let Some((action, target)) = parse_panel_custom_id(&component.data.custom_id) else {
        return false;
    };

    if component.user.id.get() != target.owner_id {
        respond_component_ephemeral(ctx, component, "Seul l'auteur du panneau peut l'utiliser.")
            .await;
        return true;
    }

    if action == "refresh" {
        let color = theme_color(ctx).await;
        let _ = component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(panel_embed(target).color(color))
                        .components(panel_components(target)),
                ),
            )
            .await;
        return true;
    }

    if action == "embed" {
        let modal = CreateModal::new(modal_custom_id("embed", target), "Configurer l'embed")
            .components(vec![
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Titre", "title")
                        .required(false)
                        .max_length(100),
                ),
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Paragraph, "Description", "description")
                        .required(false)
                        .max_length(2000),
                ),
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Couleur HEX", "color")
                        .required(false)
                        .placeholder("#5865F2"),
                ),
            ]);

        let _ = component
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await;
        return true;
    }

    if action == "addrole" {
        let modal =
            CreateModal::new(modal_custom_id("addrole", target), "Ajouter un bouton role")
                .components(vec![
                    CreateActionRow::InputText(
                        CreateInputText::new(InputTextStyle::Short, "Label du bouton", "label")
                            .required(true)
                            .max_length(80),
                    ),
                    CreateActionRow::InputText(
                        CreateInputText::new(InputTextStyle::Short, "Role ID ou mention", "role_id")
                            .required(true),
                    ),
                    CreateActionRow::InputText(
                        CreateInputText::new(InputTextStyle::Short, "Style", "style")
                            .required(false)
                            .placeholder("secondary | primary | success | danger"),
                    ),
                ]);

        let _ = component
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await;
        return true;
    }

    false
}

pub async fn handle_modal_interaction(ctx: &Context, modal: &ModalInteraction) -> bool {
    if !modal.data.custom_id.starts_with(ROLEMENU_MODAL_PREFIX) {
        return false;
    }

    let Some((action, target)) = parse_modal_custom_id(&modal.data.custom_id) else {
        return false;
    };

    if modal.user.id.get() != target.owner_id {
        respond_modal_ephemeral(ctx, modal, "Seul l'auteur du panneau peut soumettre ce formulaire.")
            .await;
        return true;
    }

    if action == "embed" {
        let Some(target_message) = fetch_target_message(ctx, target).await else {
            respond_modal_ephemeral(ctx, modal, "Message cible introuvable.").await;
            return true;
        };

        let title = modal_value(modal, "title")
            .unwrap_or_default()
            .trim()
            .to_string();
        let description = modal_value(modal, "description")
            .unwrap_or_default()
            .trim()
            .to_string();
        let color_input = modal_value(modal, "color").unwrap_or_default();

        let current_embed = target_message.embeds.first();
        let final_title = if title.is_empty() {
            current_embed
                .and_then(|embed| embed.title.clone())
                .unwrap_or_else(|| "Menu de roles".to_string())
        } else {
            title
        };

        let final_description = if description.is_empty() {
            current_embed
                .and_then(|embed| embed.description.clone())
                .unwrap_or_else(|| {
                    "Cliquez sur les boutons ci-dessous pour recevoir ou retirer un role."
                        .to_string()
                })
        } else {
            description
        };

        let fallback_color = theme_color(ctx).await;
        let final_color = parse_hex_color(&color_input)
            .or_else(|| current_embed.and_then(|embed| embed.colour.map(|c| c.0)))
            .unwrap_or(fallback_color);

        let edited = ChannelId::new(target.channel_id)
            .edit_message(
                &ctx.http,
                MessageId::new(target.message_id),
                EditMessage::new().embed(
                    CreateEmbed::new()
                        .title(final_title)
                        .description(final_description)
                        .color(final_color)
                        .timestamp(Utc::now()),
                ),
            )
            .await
            .is_ok();

        if edited {
            respond_modal_ephemeral(ctx, modal, "Embed du rolemenu mis a jour.").await;
        } else {
            respond_modal_ephemeral(ctx, modal, "Impossible de mettre a jour l'embed cible.")
                .await;
        }

        return true;
    }

    if action == "addrole" {
        let Some(target_message) = fetch_target_message(ctx, target).await else {
            respond_modal_ephemeral(ctx, modal, "Message cible introuvable.").await;
            return true;
        };

        let label = modal_value(modal, "label")
            .unwrap_or_default()
            .trim()
            .to_string();
        if label.is_empty() {
            respond_modal_ephemeral(ctx, modal, "Label invalide.").await;
            return true;
        }

        let raw_role = modal_value(modal, "role_id").unwrap_or_default();
        let Some(role_id) = parse_role_id_input(&raw_role) else {
            respond_modal_ephemeral(ctx, modal, "Role invalide. Fournis un ID ou une mention.")
                .await;
            return true;
        };

        if message_has_role_button(&target_message, role_id) {
            respond_modal_ephemeral(ctx, modal, "Ce role est deja present dans le menu.").await;
            return true;
        }

        let mut buttons = collect_message_buttons(&target_message);
        if buttons.len() >= 25 {
            respond_modal_ephemeral(ctx, modal, "Limite de 25 boutons atteinte.").await;
            return true;
        }

        let style_raw = modal_value(modal, "style").unwrap_or_default();
        let style = parse_button_style(&style_raw);

        buttons.push(
            CreateButton::new(format!("{}:{}", ROLEMENU_TOGGLE_PREFIX, role_id.get()))
                .label(label)
                .style(style),
        );

        let edited = ChannelId::new(target.channel_id)
            .edit_message(
                &ctx.http,
                MessageId::new(target.message_id),
                EditMessage::new().components(button_rows(&buttons)),
            )
            .await
            .is_ok();

        if edited {
            respond_modal_ephemeral(ctx, modal, "Bouton role ajoute au menu.").await;
        } else {
            respond_modal_ephemeral(ctx, modal, "Impossible d'ajouter le bouton role.").await;
        }

        return true;
    }

    false
}

pub struct RolemenuCommand;
pub static COMMAND_DESCRIPTOR: RolemenuCommand = RolemenuCommand;

impl crate::commands::command_contract::CommandSpec for RolemenuCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "rolemenu",
            category: "roles",
            params: "[message_id]",
            summary: "Cree ou modifie un menu de roles",
            description: "Ouvre un panneau interactif (boutons + modales) pour construire un embed de roles et des boutons auto-roles.",
            examples: &["+rolemenu", "+rolemenu 123456789012345678", "+help rolemenu"],
            default_aliases: &["rmenu"],
            default_permission: 8,
        }
    }
}