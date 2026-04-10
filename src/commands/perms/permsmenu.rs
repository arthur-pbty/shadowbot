use serenity::builder::{
    CreateActionRow, CreateButton, CreateEmbed, CreateInputText, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, CreateModal, CreateSelectMenu,
    CreateSelectMenuKind, CreateSelectMenuOption,
};
use serenity::model::application::{
    ActionRowComponent, ButtonStyle, ComponentInteraction, ComponentInteractionDataKind,
    InputTextStyle, ModalInteraction,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color, truncate_text};
use crate::commands::perms_helpers::{
    ensure_owner, get_pool, normalize_command_name, parse_user_or_role,
};
use crate::db::{
    clear_role_permissions, grant_command_access, grant_perm_level, list_role_scopes,
    remove_scope_permissions, reset_command_permissions, set_command_permission,
};
use crate::permissions::{all_command_keys, command_required_permission, default_permission};

const PERMSMENU_PREFIX: &str = "permsmenu";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MenuView {
    Overview,
    Overrides,
    Usage,
}

impl MenuView {
    fn from_action(action: &str) -> Option<Self> {
        match action {
            "overview" => Some(Self::Overview),
            "overrides" => Some(Self::Overrides),
            "usage" => Some(Self::Usage),
            _ => None,
        }
    }
}

#[derive(Default)]
struct AclSnapshot {
    total_commands: usize,
    overridden_commands: usize,
    role_scopes: usize,
    overridden_lines: Vec<String>,
}

fn parse_component_custom_id(custom_id: &str) -> Option<(String, String, u64)> {
    let mut parts = custom_id.split(':');
    if parts.next()? != PERMSMENU_PREFIX {
        return None;
    }

    let group = parts.next()?.to_string();
    let action = parts.next()?.to_string();
    let owner_id = parts.next()?.parse::<u64>().ok()?;

    if parts.next().is_some() {
        return None;
    }

    Some((group, action, owner_id))
}

fn parse_modal_custom_id(custom_id: &str) -> Option<(String, u64)> {
    let mut parts = custom_id.split(':');
    if parts.next()? != PERMSMENU_PREFIX {
        return None;
    }
    if parts.next()? != "submit" {
        return None;
    }

    let action = parts.next()?.to_string();
    let owner_id = parts.next()?.parse::<u64>().ok()?;

    if parts.next().is_some() {
        return None;
    }

    Some((action, owner_id))
}

fn parse_quick_value(value: &str) -> Option<(String, String)> {
    let mut parts = value.split(':');
    let group = parts.next()?.to_string();
    let action = parts.next()?.to_string();

    if parts.next().is_some() {
        return None;
    }

    Some((group, action))
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

fn scope_mention(scope_type: &str, scope_id: u64) -> String {
    if scope_type == "role" {
        format!("<@&{}>", scope_id)
    } else {
        format!("<@{}>", scope_id)
    }
}

async fn component_ephemeral(ctx: &Context, component: &ComponentInteraction, content: &str) {
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

async fn modal_ephemeral(ctx: &Context, modal: &ModalInteraction, content: &str) {
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

async fn acl_snapshot(ctx: &Context, pool: &sqlx::PgPool, bot_id: UserId) -> AclSnapshot {
    let mut commands = all_command_keys();
    commands.sort();

    let mut overridden_lines = Vec::new();
    for command in &commands {
        let required = command_required_permission(ctx, command).await;
        let default = default_permission(command);

        if required != default {
            overridden_lines.push(format!(
                "`{}` -> `{}` (defaut `{}`)",
                command, required, default
            ));
        }
    }

    let role_scopes = list_role_scopes(pool, bot_id)
        .await
        .unwrap_or_default()
        .len();

    AclSnapshot {
        total_commands: commands.len(),
        overridden_commands: overridden_lines.len(),
        role_scopes,
        overridden_lines,
    }
}

fn build_overview_embed(
    snapshot: &AclSnapshot,
    color: u32,
    last_action: Option<&str>,
) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title("Perms Menu v2")
        .description("Panel ACL v2 pour configurer commandes, niveaux et scopes depuis des composants message.")
        .field(
            "Etat ACL",
            format!(
                "Commandes connues: `{}`\nOverrides commandes: `{}`\nScopes roles configures: `{}`",
                snapshot.total_commands, snapshot.overridden_commands, snapshot.role_scopes
            ),
            false,
        )
        .field(
            "Actions",
            "- Set command permission\n- Grant permission or command access\n- Delete scope ACL\n- Reset command overrides\n- Clear role ACL",
            false,
        )
        .field(
            "Raccourcis",
            "`+allperms` `+perms` `+setperm` `+delperm` `+clearperms`",
            false,
        )
        .color(color);

    if let Some(action) = last_action {
        embed = embed.field("Derniere action", action, false);
    }

    embed
}

fn build_overrides_embed(snapshot: &AclSnapshot, color: u32) -> CreateEmbed {
    let details = if snapshot.overridden_lines.is_empty() {
        "Aucun override de commande actif.".to_string()
    } else {
        truncate_text(&snapshot.overridden_lines.join("\n"), 3800)
    };

    CreateEmbed::new()
        .title("Perms Menu v2 - Overrides")
        .description(format!(
            "{} override(s) detecte(s).",
            snapshot.overridden_commands
        ))
        .field("Liste", details, false)
        .color(color)
}

fn build_usage_embed(color: u32) -> CreateEmbed {
    CreateEmbed::new()
        .title("Perms Menu v2 - Aide")
        .description("Syntaxes utiles pour piloter le systeme ACL.")
        .field(
            "Commandes",
            "`+setperm 6 @Role`\n`+setperm mute @Role`\n`+delperm @Role`\n`+clearperms`\n`+allperms`",
            false,
        )
        .field(
            "Depuis le panel",
            "- Boutons: ouvrir modals et actions directes\n- Select: navigation rapide et operations",
            false,
        )
        .color(color)
}

fn build_components(owner_id: UserId, view: MenuView) -> Vec<CreateActionRow> {
    let overview_style = if view == MenuView::Overview {
        ButtonStyle::Success
    } else {
        ButtonStyle::Secondary
    };
    let overrides_style = if view == MenuView::Overrides {
        ButtonStyle::Success
    } else {
        ButtonStyle::Secondary
    };
    let usage_style = if view == MenuView::Usage {
        ButtonStyle::Success
    } else {
        ButtonStyle::Secondary
    };

    let row_actions = CreateActionRow::Buttons(vec![
        CreateButton::new(format!(
            "{}:modal:setcmd:{}",
            PERMSMENU_PREFIX,
            owner_id.get()
        ))
        .label("Set Command")
        .style(ButtonStyle::Primary),
        CreateButton::new(format!(
            "{}:modal:grant:{}",
            PERMSMENU_PREFIX,
            owner_id.get()
        ))
        .label("Grant Scope")
        .style(ButtonStyle::Primary),
        CreateButton::new(format!("{}:modal:del:{}", PERMSMENU_PREFIX, owner_id.get()))
            .label("Delete Scope")
            .style(ButtonStyle::Danger),
        CreateButton::new(format!(
            "{}:view:overview:{}",
            PERMSMENU_PREFIX,
            owner_id.get()
        ))
        .label("Dashboard")
        .style(overview_style),
    ]);

    let row_views = CreateActionRow::Buttons(vec![
        CreateButton::new(format!(
            "{}:view:overrides:{}",
            PERMSMENU_PREFIX,
            owner_id.get()
        ))
        .label("Overrides")
        .style(overrides_style),
        CreateButton::new(format!(
            "{}:view:usage:{}",
            PERMSMENU_PREFIX,
            owner_id.get()
        ))
        .label("Usage")
        .style(usage_style),
        CreateButton::new(format!(
            "{}:apply:resetcmd:{}",
            PERMSMENU_PREFIX,
            owner_id.get()
        ))
        .label("Reset Cmd")
        .style(ButtonStyle::Danger),
        CreateButton::new(format!(
            "{}:apply:clearroles:{}",
            PERMSMENU_PREFIX,
            owner_id.get()
        ))
        .label("Clear Roles")
        .style(ButtonStyle::Danger),
    ]);

    let quick_menu = CreateSelectMenu::new(
        format!("{}:quick:actions:{}", PERMSMENU_PREFIX, owner_id.get()),
        CreateSelectMenuKind::String {
            options: vec![
                CreateSelectMenuOption::new("View dashboard", "view:overview"),
                CreateSelectMenuOption::new("View overrides", "view:overrides"),
                CreateSelectMenuOption::new("View usage", "view:usage"),
                CreateSelectMenuOption::new("Apply reset command overrides", "apply:resetcmd"),
                CreateSelectMenuOption::new("Apply clear role ACL", "apply:clearroles"),
            ],
        },
    )
    .placeholder("Quick action ACL v2");

    vec![
        row_actions,
        row_views,
        CreateActionRow::SelectMenu(quick_menu),
    ]
}

fn set_command_modal(owner_id: u64) -> CreateModal {
    CreateModal::new(
        format!("{}:submit:setcmd:{}", PERMSMENU_PREFIX, owner_id),
        "Perms Menu v2 - Set Command",
    )
    .components(vec![
        CreateActionRow::InputText(
            CreateInputText::new(InputTextStyle::Short, "Command key", "command")
                .placeholder("mute, clearperms, ticketadd")
                .required(true),
        ),
        CreateActionRow::InputText(
            CreateInputText::new(InputTextStyle::Short, "Permission level (0-9)", "level")
                .placeholder("6")
                .required(true),
        ),
    ])
}

fn grant_scope_modal(owner_id: u64) -> CreateModal {
    CreateModal::new(
        format!("{}:submit:grant:{}", PERMSMENU_PREFIX, owner_id),
        "Perms Menu v2 - Grant Scope",
    )
    .components(vec![
        CreateActionRow::InputText(
            CreateInputText::new(InputTextStyle::Short, "Mode (level|command)", "mode")
                .placeholder("level")
                .required(false),
        ),
        CreateActionRow::InputText(
            CreateInputText::new(InputTextStyle::Short, "Value (0-9 or command)", "value")
                .placeholder("6 or mute")
                .required(true),
        ),
        CreateActionRow::InputText(
            CreateInputText::new(
                InputTextStyle::Short,
                "Target (role/user mention or id)",
                "target",
            )
            .placeholder("<@&123> or <@123>")
            .required(true),
        ),
    ])
}

fn delete_scope_modal(owner_id: u64) -> CreateModal {
    CreateModal::new(
        format!("{}:submit:del:{}", PERMSMENU_PREFIX, owner_id),
        "Perms Menu v2 - Delete Scope",
    )
    .components(vec![CreateActionRow::InputText(
        CreateInputText::new(
            InputTextStyle::Short,
            "Target (role/user mention or id)",
            "target",
        )
        .placeholder("<@&123> or <@123>")
        .required(true),
    )])
}

async fn update_panel_message(
    ctx: &Context,
    component: &ComponentInteraction,
    pool: &sqlx::PgPool,
    bot_id: UserId,
    view: MenuView,
    last_action: Option<&str>,
) {
    let snapshot = acl_snapshot(ctx, pool, bot_id).await;
    let color = theme_color(ctx).await;

    let embed = match view {
        MenuView::Overview => build_overview_embed(&snapshot, color, last_action),
        MenuView::Overrides => build_overrides_embed(&snapshot, color),
        MenuView::Usage => build_usage_embed(color),
    };

    let _ = component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embed(embed)
                    .components(build_components(component.user.id, view)),
            ),
        )
        .await;
}

async fn apply_set_command_modal(
    modal: &ModalInteraction,
    pool: &sqlx::PgPool,
    bot_id: UserId,
) -> Result<String, String> {
    let command_raw = modal_value(modal, "command")
        .unwrap_or_default()
        .trim()
        .to_string();
    if command_raw.is_empty() {
        return Err("Commande manquante.".to_string());
    }

    let level = modal_value(modal, "level")
        .unwrap_or_default()
        .trim()
        .parse::<u8>()
        .map_err(|_| "Niveau invalide. Valeurs attendues: 0..9".to_string())?;
    if level > 9 {
        return Err("Niveau invalide. Valeurs attendues: 0..9".to_string());
    }

    let command = normalize_command_name(&command_raw);
    let known_commands = all_command_keys();
    if !known_commands.iter().any(|cmd| cmd == &command) {
        return Err(format!("Commande inconnue: `{}`", command));
    }

    set_command_permission(pool, bot_id, &command, level)
        .await
        .map_err(|err| format!("Erreur DB: {}", err))?;

    Ok(format!(
        "Permission requise pour `{}` definie sur `{}`.",
        command, level
    ))
}

async fn apply_grant_modal(
    modal: &ModalInteraction,
    pool: &sqlx::PgPool,
    bot_id: UserId,
) -> Result<String, String> {
    let mode = modal_value(modal, "mode")
        .unwrap_or_default()
        .trim()
        .to_lowercase();
    let value = modal_value(modal, "value")
        .unwrap_or_default()
        .trim()
        .to_string();
    let target = modal_value(modal, "target")
        .unwrap_or_default()
        .trim()
        .to_string();

    if value.is_empty() || target.is_empty() {
        return Err("Value et target sont obligatoires.".to_string());
    }

    let Some((scope_type, scope_id)) = parse_user_or_role(&target) else {
        return Err("Target invalide. Utilise une mention role/user ou un id.".to_string());
    };

    let is_level_mode = matches!(mode.as_str(), "level" | "perm" | "permission" | "niveau")
        || (mode.is_empty() && value.parse::<u8>().is_ok());

    if is_level_mode {
        let level = value
            .parse::<u8>()
            .map_err(|_| "Niveau invalide. Valeurs attendues: 0..9".to_string())?;
        if level > 9 {
            return Err("Niveau invalide. Valeurs attendues: 0..9".to_string());
        }

        grant_perm_level(pool, bot_id, scope_type, scope_id, level)
            .await
            .map_err(|err| format!("Erreur DB: {}", err))?;

        return Ok(format!(
            "Permission `{}` accordee a {}.",
            level,
            scope_mention(scope_type, scope_id)
        ));
    }

    let command = normalize_command_name(&value);
    let known_commands = all_command_keys();
    if !known_commands.iter().any(|cmd| cmd == &command) {
        return Err(format!("Commande inconnue: `{}`", command));
    }

    grant_command_access(pool, bot_id, scope_type, scope_id, &command)
        .await
        .map_err(|err| format!("Erreur DB: {}", err))?;

    Ok(format!(
        "Acces direct `{}` accorde a {}.",
        command,
        scope_mention(scope_type, scope_id)
    ))
}

async fn apply_delete_modal(
    modal: &ModalInteraction,
    pool: &sqlx::PgPool,
    bot_id: UserId,
) -> Result<String, String> {
    let target = modal_value(modal, "target")
        .unwrap_or_default()
        .trim()
        .to_string();

    let Some((scope_type, scope_id)) = parse_user_or_role(&target) else {
        return Err("Target invalide. Utilise une mention role/user ou un id.".to_string());
    };

    let removed = remove_scope_permissions(pool, bot_id, scope_type, scope_id)
        .await
        .map_err(|err| format!("Erreur DB: {}", err))?;

    Ok(format!(
        "{} entree(s) ACL supprimee(s) pour {}.",
        removed,
        scope_mention(scope_type, scope_id)
    ))
}

pub async fn handle_component_interaction(ctx: &Context, component: &ComponentInteraction) -> bool {
    let Some((group, action, owner_id)) = parse_component_custom_id(&component.data.custom_id)
    else {
        return false;
    };

    if component.user.id.get() != owner_id {
        component_ephemeral(
            ctx,
            component,
            "Seul l'auteur de la commande peut utiliser ce panel.",
        )
        .await;
        return true;
    }

    let bot_id = ctx.cache.current_user().id;
    let Some(pool) = get_pool(ctx).await else {
        component_ephemeral(ctx, component, "DB indisponible.").await;
        return true;
    };

    if group == "modal" {
        let modal = match action.as_str() {
            "setcmd" => Some(set_command_modal(owner_id)),
            "grant" => Some(grant_scope_modal(owner_id)),
            "del" => Some(delete_scope_modal(owner_id)),
            _ => None,
        };

        if let Some(modal) = modal {
            let _ = component
                .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
                .await;
            return true;
        }

        return false;
    }

    if group == "quick" {
        if let ComponentInteractionDataKind::StringSelect { values } = &component.data.kind {
            if let Some(selected) = values.first() {
                if let Some((quick_group, quick_action)) = parse_quick_value(selected) {
                    if quick_group == "view" {
                        if let Some(view) = MenuView::from_action(&quick_action) {
                            update_panel_message(ctx, component, &pool, bot_id, view, None).await;
                            return true;
                        }
                    }

                    if quick_group == "apply" {
                        let status = match quick_action.as_str() {
                            "resetcmd" => {
                                let removed =
                                    reset_command_permissions(&pool, bot_id).await.unwrap_or(0);
                                format!(
                                    "Reset des permissions de commande termine: {} entree(s).",
                                    removed
                                )
                            }
                            "clearroles" => {
                                let removed =
                                    clear_role_permissions(&pool, bot_id).await.unwrap_or(0);
                                format!("Reset ACL roles termine: {} entree(s).", removed)
                            }
                            _ => "Action inconnue.".to_string(),
                        };

                        update_panel_message(
                            ctx,
                            component,
                            &pool,
                            bot_id,
                            MenuView::Overview,
                            Some(&status),
                        )
                        .await;
                        return true;
                    }
                }
            }
        }

        component_ephemeral(ctx, component, "Action rapide invalide.").await;
        return true;
    }

    if group == "view" {
        if let Some(view) = MenuView::from_action(&action) {
            update_panel_message(ctx, component, &pool, bot_id, view, None).await;
            return true;
        }

        component_ephemeral(ctx, component, "Vue inconnue.").await;
        return true;
    }

    if group == "apply" {
        let status = match action.as_str() {
            "resetcmd" => {
                let removed = reset_command_permissions(&pool, bot_id).await.unwrap_or(0);
                format!(
                    "Reset des permissions de commande termine: {} entree(s).",
                    removed
                )
            }
            "clearroles" => {
                let removed = clear_role_permissions(&pool, bot_id).await.unwrap_or(0);
                format!("Reset ACL roles termine: {} entree(s).", removed)
            }
            _ => "Action inconnue.".to_string(),
        };

        update_panel_message(
            ctx,
            component,
            &pool,
            bot_id,
            MenuView::Overview,
            Some(&status),
        )
        .await;
        return true;
    }

    false
}

pub async fn handle_modal_interaction(ctx: &Context, modal: &ModalInteraction) -> bool {
    let Some((action, owner_id)) = parse_modal_custom_id(&modal.data.custom_id) else {
        return false;
    };

    if modal.user.id.get() != owner_id {
        modal_ephemeral(
            ctx,
            modal,
            "Seul l'auteur de la commande peut utiliser ce panel.",
        )
        .await;
        return true;
    }

    let bot_id = ctx.cache.current_user().id;
    let Some(pool) = get_pool(ctx).await else {
        modal_ephemeral(ctx, modal, "DB indisponible.").await;
        return true;
    };

    let result = match action.as_str() {
        "setcmd" => apply_set_command_modal(modal, &pool, bot_id).await,
        "grant" => apply_grant_modal(modal, &pool, bot_id).await,
        "del" => apply_delete_modal(modal, &pool, bot_id).await,
        _ => Err("Action modal inconnue.".to_string()),
    };

    let response_text = match result {
        Ok(message) => message,
        Err(message) => message,
    };

    modal_ephemeral(ctx, modal, &response_text).await;
    true
}

pub async fn handle_permsmenu(ctx: &Context, msg: &Message, args: &[&str]) {
    let _ = args;

    if !ensure_owner(ctx, msg).await {
        return;
    }

    let bot_id = ctx.cache.current_user().id;
    let Some(pool) = get_pool(ctx).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let snapshot = acl_snapshot(ctx, &pool, bot_id).await;
    let color = theme_color(ctx).await;
    let embed = build_overview_embed(&snapshot, color, None);
    let components = build_components(msg.author.id, MenuView::Overview);

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new().embed(embed).components(components),
        )
        .await;
}

pub struct PermsmenuCommand;
pub static COMMAND_DESCRIPTOR: PermsmenuCommand = PermsmenuCommand;

impl crate::commands::command_contract::CommandSpec for PermsmenuCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "permsmenu",
            category: "perms",
            params: "aucun",
            description: "Ouvre un panel ACL v2 avec embed et composants pour gerer le systeme de permissions.",
            examples: &["+permsmenu", "+pmenu", "+help permsmenu"],
            default_aliases: &["pmenu", "prmenu"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
