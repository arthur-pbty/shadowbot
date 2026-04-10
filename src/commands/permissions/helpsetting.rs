use serenity::builder::{
    CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption,
};
use serenity::model::application::ComponentInteractionDataKind;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::db::{
    DbPoolKey, get_help_aliases_enabled, get_help_perms_enabled, get_help_type,
    set_help_aliases_enabled, set_help_perms_enabled, set_help_type,
};

#[derive(Clone)]
struct HelpSettingsData {
    layout: String,
    aliases_enabled: bool,
    perms_enabled: bool,
}

fn normalize_layout(value: &str) -> Option<&'static str> {
    match value.to_lowercase().as_str() {
        "button" => Some("button"),
        "select" => Some("select"),
        "hybrid" => Some("hybrid"),
        _ => None,
    }
}

fn owner_id_from_custom_id(custom_id: &str) -> Option<u64> {
    custom_id.rsplit(':').next()?.parse::<u64>().ok()
}

fn mode_from_custom_id(custom_id: &str) -> Option<&str> {
    let parts = custom_id.split(':').collect::<Vec<_>>();
    if parts.len() != 4 || parts[0] != "helpsetting" || parts[1] != "setmode" {
        return None;
    }
    Some(parts[2])
}

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

async fn read_settings(pool: &sqlx::PgPool, bot_id: UserId) -> HelpSettingsData {
    let layout = get_help_type(pool, bot_id)
        .await
        .ok()
        .flatten()
        .unwrap_or_else(|| "button".to_string());
    let aliases_enabled = get_help_aliases_enabled(pool, bot_id)
        .await
        .ok()
        .flatten()
        .unwrap_or(true);
    let perms_enabled = get_help_perms_enabled(pool, bot_id)
        .await
        .ok()
        .flatten()
        .unwrap_or(true);

    HelpSettingsData {
        layout,
        aliases_enabled,
        perms_enabled,
    }
}

fn build_embed(settings: &HelpSettingsData) -> CreateEmbed {
    CreateEmbed::new()
        .title("Configuration de l'aide")
        .description("Tu peux modifier ces paramètres via boutons ou menu select.")
        .field("Mode d'affichage", format!("`{}`", settings.layout), true)
        .field(
            "Aliases",
            format!(
                "`{}`",
                if settings.aliases_enabled {
                    "on"
                } else {
                    "off"
                }
            ),
            true,
        )
        .field(
            "Permissions",
            format!("`{}`", if settings.perms_enabled { "on" } else { "off" }),
            true,
        )
        .color(0x5865F2)
}

fn build_components(owner_id: UserId, settings: &HelpSettingsData) -> Vec<CreateActionRow> {
    let mode_button_style = if settings.layout == "button" {
        ButtonStyle::Success
    } else {
        ButtonStyle::Secondary
    };
    let mode_select_style = if settings.layout == "select" {
        ButtonStyle::Success
    } else {
        ButtonStyle::Secondary
    };
    let mode_hybrid_style = if settings.layout == "hybrid" {
        ButtonStyle::Success
    } else {
        ButtonStyle::Secondary
    };

    let aliases_style = if settings.aliases_enabled {
        ButtonStyle::Success
    } else {
        ButtonStyle::Danger
    };
    let perms_style = if settings.perms_enabled {
        ButtonStyle::Success
    } else {
        ButtonStyle::Danger
    };

    let mode_row = CreateActionRow::Buttons(vec![
        CreateButton::new(format!("helpsetting:setmode:button:{}", owner_id.get()))
            .label("Mode Button")
            .style(mode_button_style),
        CreateButton::new(format!("helpsetting:setmode:select:{}", owner_id.get()))
            .label("Mode Select")
            .style(mode_select_style),
        CreateButton::new(format!("helpsetting:setmode:hybrid:{}", owner_id.get()))
            .label("Mode Hybrid")
            .style(mode_hybrid_style),
    ]);

    let toggle_row = CreateActionRow::Buttons(vec![
        CreateButton::new(format!("helpsetting:toggle:aliases:{}", owner_id.get()))
            .label(if settings.aliases_enabled {
                "Aliases: on"
            } else {
                "Aliases: off"
            })
            .style(aliases_style),
        CreateButton::new(format!("helpsetting:toggle:perms:{}", owner_id.get()))
            .label(if settings.perms_enabled {
                "Perms: on"
            } else {
                "Perms: off"
            })
            .style(perms_style),
        CreateButton::new(format!("helpsetting:refresh:{}", owner_id.get()))
            .label("Rafraîchir")
            .style(ButtonStyle::Primary),
    ]);

    let quick_options = vec![
        CreateSelectMenuOption::new("Mode button", "mode:button"),
        CreateSelectMenuOption::new("Mode select", "mode:select"),
        CreateSelectMenuOption::new("Mode hybrid", "mode:hybrid"),
        CreateSelectMenuOption::new("Aliases on", "aliases:on"),
        CreateSelectMenuOption::new("Aliases off", "aliases:off"),
        CreateSelectMenuOption::new("Perms on", "perms:on"),
        CreateSelectMenuOption::new("Perms off", "perms:off"),
    ];

    let quick_menu = CreateSelectMenu::new(
        format!("helpsetting:quick:{}", owner_id.get()),
        CreateSelectMenuKind::String {
            options: quick_options,
        },
    )
    .placeholder("Action rapide (select)");

    vec![
        mode_row,
        toggle_row,
        CreateActionRow::SelectMenu(quick_menu),
    ]
}

async fn send_settings_panel(
    ctx: &Context,
    msg: &Message,
    pool: &sqlx::PgPool,
    bot_id: UserId,
    owner_id: UserId,
) {
    let settings = read_settings(pool, bot_id).await;
    let embed = build_embed(&settings);
    let components = build_components(owner_id, &settings);

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new().embed(embed).components(components),
        )
        .await;
}

async fn apply_quick_action(pool: &sqlx::PgPool, bot_id: UserId, action: &str) {
    match action {
        "mode:button" => {
            let _ = set_help_type(pool, bot_id, "button").await;
        }
        "mode:select" => {
            let _ = set_help_type(pool, bot_id, "select").await;
        }
        "mode:hybrid" => {
            let _ = set_help_type(pool, bot_id, "hybrid").await;
        }
        "aliases:on" => {
            let _ = set_help_aliases_enabled(pool, bot_id, true).await;
        }
        "aliases:off" => {
            let _ = set_help_aliases_enabled(pool, bot_id, false).await;
        }
        "perms:on" => {
            let _ = set_help_perms_enabled(pool, bot_id, true).await;
        }
        "perms:off" => {
            let _ = set_help_perms_enabled(pool, bot_id, false).await;
        }
        _ => {}
    }
}

pub async fn handle_component_interaction(ctx: &Context, component: &ComponentInteraction) -> bool {
    let custom_id = &component.data.custom_id;
    if !custom_id.starts_with("helpsetting:") {
        return false;
    }

    let Some(owner_id) = owner_id_from_custom_id(custom_id) else {
        return false;
    };

    if component.user.id.get() != owner_id {
        let _ = component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Seul l'auteur de la commande peut utiliser ces contrôles.")
                        .ephemeral(true),
                ),
            )
            .await;
        return true;
    }

    let bot_id = ctx.cache.current_user().id;
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

    if custom_id.starts_with("helpsetting:setmode:") {
        if let Some(mode) = mode_from_custom_id(custom_id).and_then(normalize_layout) {
            let _ = set_help_type(&pool, bot_id, mode).await;
        }
    } else if custom_id.starts_with("helpsetting:toggle:aliases:") {
        let settings = read_settings(&pool, bot_id).await;
        let _ = set_help_aliases_enabled(&pool, bot_id, !settings.aliases_enabled).await;
    } else if custom_id.starts_with("helpsetting:toggle:perms:") {
        let settings = read_settings(&pool, bot_id).await;
        let _ = set_help_perms_enabled(&pool, bot_id, !settings.perms_enabled).await;
    } else if custom_id.starts_with("helpsetting:quick:") {
        if let ComponentInteractionDataKind::StringSelect { values } = &component.data.kind {
            if let Some(action) = values.first() {
                apply_quick_action(&pool, bot_id, action).await;
            }
        }
    }

    let settings = read_settings(&pool, bot_id).await;
    let embed = build_embed(&settings);
    let components = build_components(component.user.id, &settings);

    let _ = component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embed(embed)
                    .components(components),
            ),
        )
        .await;

    true
}

pub async fn handle_helpsetting(ctx: &Context, msg: &Message, args: &[&str]) {
    let bot_id = ctx.cache.current_user().id;
    let Some(pool) = pool(ctx).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if args.is_empty() {
        send_settings_panel(ctx, msg, &pool, bot_id, msg.author.id).await;
        return;
    }

    match args[0].to_lowercase().as_str() {
        "type" | "mode" => {
            let Some(value) = args.get(1).and_then(|value| normalize_layout(value)) else {
                let embed = CreateEmbed::new()
                    .title("Erreur")
                    .description("Usage: `+helpsetting type <button|select|hybrid>`")
                    .color(0xED4245);
                send_embed(ctx, msg, embed).await;
                return;
            };
            let _ = set_help_type(&pool, bot_id, value).await;
            send_settings_panel(ctx, msg, &pool, bot_id, msg.author.id).await;
        }
        "aliases" | "alias" => {
            let Some(value) = args.get(1) else {
                let embed = CreateEmbed::new()
                    .title("Erreur")
                    .description("Usage: `+helpsetting aliases <on|off>`")
                    .color(0xED4245);
                send_embed(ctx, msg, embed).await;
                return;
            };
            let enabled = match value.to_lowercase().as_str() {
                "on" | "true" | "yes" => true,
                "off" | "false" | "no" => false,
                _ => {
                    let embed = CreateEmbed::new()
                        .title("Erreur")
                        .description("Valeurs valides: `on`, `off`")
                        .color(0xED4245);
                    send_embed(ctx, msg, embed).await;
                    return;
                }
            };
            let _ = set_help_aliases_enabled(&pool, bot_id, enabled).await;
            send_settings_panel(ctx, msg, &pool, bot_id, msg.author.id).await;
        }
        "perms" | "permissions" => {
            let Some(value) = args.get(1) else {
                let embed = CreateEmbed::new()
                    .title("Erreur")
                    .description("Usage: `+helpsetting perms <on|off>`")
                    .color(0xED4245);
                send_embed(ctx, msg, embed).await;
                return;
            };
            let enabled = match value.to_lowercase().as_str() {
                "on" | "true" | "yes" => true,
                "off" | "false" | "no" => false,
                _ => {
                    let embed = CreateEmbed::new()
                        .title("Erreur")
                        .description("Valeurs valides: `on`, `off`")
                        .color(0xED4245);
                    send_embed(ctx, msg, embed).await;
                    return;
                }
            };
            let _ = set_help_perms_enabled(&pool, bot_id, enabled).await;
            send_settings_panel(ctx, msg, &pool, bot_id, msg.author.id).await;
        }
        _ => {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description("Sous-commandes: `type`, `aliases`, `perms`")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
        }
    }
}

pub struct HelpsettingCommand;
pub static COMMAND_DESCRIPTOR: HelpsettingCommand = HelpsettingCommand;

impl crate::commands::command_contract::CommandSpec for HelpsettingCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "helpsetting",
            category: "permissions",
            params: "[type|aliases|perms] [value]",
            summary: "Configure l'affichage du système d'aide",
            description: "Permet de configurer le mode d'affichage, l'affichage des alias et l'affichage des permissions du système d'aide.",
            examples: &[
                "+helpsetting",
                "+helpsetting type hybrid",
                "+helpsetting perms off",
            ],
            default_aliases: &["hs", "helpetting"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
