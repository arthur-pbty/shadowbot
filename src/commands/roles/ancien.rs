use chrono::Utc;
use serenity::builder::{
    CreateActionRow, CreateButton, CreateEmbed, CreateInputText, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, CreateModal,
};
use serenity::model::application::{
    ActionRowComponent, ButtonStyle, ComponentInteraction, InputTextStyle, ModalInteraction,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::theme_color;
use crate::db;

const ANCIEN_MENU: &str = "ancien:settings";
const ANCIEN_ROLE_INPUT_ID: &str = "role_id";
const ANCIEN_DELAY_INPUT_ID: &str = "delay";

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

fn parse_role_id_input(raw: &str) -> Option<RoleId> {
    let cleaned = raw.trim().trim_start_matches("<@&").trim_end_matches('>');
    cleaned.parse::<u64>().ok().map(RoleId::new)
}

fn parse_delay_seconds(input: &str) -> Option<i64> {
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

    let value = number.parse::<i64>().ok()?;
    if value <= 0 {
        return None;
    }

    let unit = if suffix.is_empty() { "j" } else { &suffix };

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

fn format_delay(seconds: i64) -> String {
    let mut remaining = seconds.max(1);
    let days = remaining / 86_400;
    remaining %= 86_400;
    let hours = remaining / 3_600;
    remaining %= 3_600;
    let minutes = remaining / 60;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{}j", days));
    }
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    if parts.is_empty() {
        parts.push(format!("{}s", seconds.max(1)));
    }

    parts.join(" ")
}

fn ancien_embed(settings: &db::OldMemberSettings) -> CreateEmbed {
    let role_label = settings
        .role_id
        .and_then(|id| u64::try_from(id).ok())
        .map(|id| format!("<@&{}>", id))
        .unwrap_or_else(|| "Non configure".to_string());

    CreateEmbed::new()
        .title("Ancien")
        .description("Definit au bout de combien de temps un membre devient ancien sur le serveur.")
        .field(
            "Statut",
            if settings.enabled { "Actif" } else { "Inactif" },
            true,
        )
        .field("Role ancien", role_label, true)
        .field("Delai", format_delay(settings.delay_seconds), true)
        .field(
            "Configuration",
            "Utilise le bouton Configurer pour definir l'ID du role et le delai.",
            false,
        )
        .timestamp(Utc::now())
}

fn ancien_components(owner_id: UserId, settings: &db::OldMemberSettings) -> Vec<CreateActionRow> {
    let toggle_label = if settings.enabled {
        "Desactiver"
    } else {
        "Activer"
    };

    let toggle_style = if settings.enabled {
        ButtonStyle::Danger
    } else {
        ButtonStyle::Success
    };

    vec![CreateActionRow::Buttons(vec![
        CreateButton::new(format!("{}:toggle:{}", ANCIEN_MENU, owner_id.get()))
            .label(toggle_label)
            .style(toggle_style),
        CreateButton::new(format!("{}:configure:{}", ANCIEN_MENU, owner_id.get()))
            .label("Configurer")
            .style(ButtonStyle::Primary),
        CreateButton::new(format!("{}:refresh:{}", ANCIEN_MENU, owner_id.get()))
            .label("Rafraichir")
            .style(ButtonStyle::Secondary),
    ])]
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
    let settings = db::get_or_create_old_member_settings(&pool, bot_id, guild_id.get() as i64)
        .await
        .unwrap_or(db::OldMemberSettings {
            bot_id,
            guild_id: guild_id.get() as i64,
            role_id: None,
            delay_seconds: 2_592_000,
            enabled: false,
            updated_at: Utc::now(),
        });

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .embed(ancien_embed(&settings).color(theme_color(ctx).await))
                .components(ancien_components(msg.author.id, &settings)),
        )
        .await;
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

pub async fn handle_ancien(ctx: &Context, msg: &Message, _args: &[&str]) {
    show_menu(ctx, msg).await;
}

pub async fn handle_component_interaction(ctx: &Context, component: &ComponentInteraction) -> bool {
    if !component.data.custom_id.starts_with(ANCIEN_MENU) {
        return false;
    }

    let Some((action, owner_id)) = parse_owner_id(&component.data.custom_id) else {
        return false;
    };

    if component.user.id.get() != owner_id {
        respond_ephemeral_component(
            ctx,
            component,
            "Seul l'auteur du menu peut utiliser ces boutons.",
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
    let settings = db::get_or_create_old_member_settings(&pool, bot_id, guild_id.get() as i64)
        .await
        .ok();

    let Some(settings) = settings else {
        return true;
    };

    if action.ends_with(":configure") {
        let modal = CreateModal::new(component.data.custom_id.clone(), "Configurer Ancien")
            .components(vec![
                CreateActionRow::InputText(
                    CreateInputText::new(
                        InputTextStyle::Short,
                        "ID du role ancien (ou mention)",
                        ANCIEN_ROLE_INPUT_ID,
                    )
                    .required(true),
                ),
                CreateActionRow::InputText(
                    CreateInputText::new(
                        InputTextStyle::Short,
                        "Delai (ex: 30j, 72h, 90m)",
                        ANCIEN_DELAY_INPUT_ID,
                    )
                    .required(true),
                ),
            ]);

        let _ = component
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await;
        return true;
    }

    if action.ends_with(":toggle") {
        if !settings.enabled && settings.role_id.is_none() {
            respond_ephemeral_component(
                ctx,
                component,
                "Configure d'abord le role et le delai avant d'activer.",
            )
            .await;
            return true;
        }

        let updated = db::update_old_member_settings(
            &pool,
            bot_id,
            guild_id.get() as i64,
            settings.role_id,
            settings.delay_seconds,
            !settings.enabled,
        )
        .await
        .ok();

        if let Some(updated) = updated {
            let _ = component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .embed(ancien_embed(&updated).color(theme_color(ctx).await))
                            .components(ancien_components(component.user.id, &updated)),
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
                        .embed(ancien_embed(&settings).color(theme_color(ctx).await))
                        .components(ancien_components(component.user.id, &settings)),
                ),
            )
            .await;
        return true;
    }

    false
}

pub async fn handle_modal_interaction(ctx: &Context, modal: &ModalInteraction) -> bool {
    if !modal.data.custom_id.starts_with(ANCIEN_MENU) {
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

    let role_raw = modal_value(modal, ANCIEN_ROLE_INPUT_ID).unwrap_or_default();
    let Some(role_id) = parse_role_id_input(&role_raw) else {
        respond_ephemeral_modal(ctx, modal, "Role invalide. Fournis un ID ou une mention.").await;
        return true;
    };

    let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await else {
        respond_ephemeral_modal(ctx, modal, "Impossible de verifier le role sur ce serveur.").await;
        return true;
    };

    if !guild.roles.contains_key(&role_id) {
        respond_ephemeral_modal(ctx, modal, "Le role indique n'existe pas sur ce serveur.").await;
        return true;
    }

    let delay_raw = modal_value(modal, ANCIEN_DELAY_INPUT_ID).unwrap_or_default();
    let Some(delay_seconds) = parse_delay_seconds(&delay_raw) else {
        respond_ephemeral_modal(
            ctx,
            modal,
            "Delai invalide. Exemples valides: 30j, 72h, 90m.",
        )
        .await;
        return true;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let current = db::get_or_create_old_member_settings(&pool, bot_id, guild_id.get() as i64)
        .await
        .ok();

    let Some(current) = current else {
        return true;
    };

    let updated = db::update_old_member_settings(
        &pool,
        bot_id,
        guild_id.get() as i64,
        Some(role_id.get() as i64),
        delay_seconds,
        current.enabled,
    )
    .await
    .ok();

    if let Some(updated) = updated {
        let _ = modal
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .embed(ancien_embed(&updated).color(theme_color(ctx).await))
                        .components(ancien_components(modal.user.id, &updated))
                        .ephemeral(true),
                ),
            )
            .await;
    }

    true
}

pub async fn maybe_assign_ancien_role(ctx: &Context, guild_id: GuildId, user_id: UserId) {
    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let settings = db::get_or_create_old_member_settings(&pool, bot_id, guild_id.get() as i64)
        .await
        .ok();

    let Some(settings) = settings else {
        return;
    };

    if !settings.enabled {
        return;
    }

    let Some(role_raw) = settings.role_id else {
        return;
    };

    let Ok(role_id_u64) = u64::try_from(role_raw) else {
        return;
    };

    let role_id = RoleId::new(role_id_u64);

    let Ok(member) = guild_id.member(&ctx.http, user_id).await else {
        return;
    };

    if member.user.bot || member.roles.contains(&role_id) {
        return;
    }

    let joined_at = member.joined_at.unwrap_or_else(|| member.user.created_at());
    let elapsed = Utc::now()
        .timestamp()
        .saturating_sub(joined_at.unix_timestamp());

    if elapsed < settings.delay_seconds.max(1) {
        return;
    }

    let _ = member.add_role(&ctx.http, role_id).await;
}

pub struct AncienCommand;
pub static COMMAND_DESCRIPTOR: AncienCommand = AncienCommand;

impl crate::commands::command_contract::CommandSpec for AncienCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "ancien",
            category: "roles",
            params: "aucun",
            description: "Definit au bout de combien de temps un membre est considere comme ancien et recoit le role configure.",
            examples: &["+ancien", "+help ancien"],
            default_aliases: &[],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
