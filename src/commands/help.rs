use std::collections::BTreeMap;

use serenity::builder::{
    CreateActionRow, CreateButton, CreateCommand, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption,
};
use serenity::model::application::{
    Command, CommandInteraction, ComponentInteractionDataKind, Interaction,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::alias::resolve_alias;
use crate::commands::common::{add_list_fields, truncate_text};
use crate::db::{
    DbPoolKey, get_help_aliases_enabled, get_help_perms_enabled, get_help_type,
    list_command_aliases,
};
use crate::permissions;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HelpLayout {
    Button,
    Select,
    Hybrid,
}

impl HelpLayout {
    fn from_str(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "select" => Self::Select,
            "hybrid" => Self::Hybrid,
            _ => Self::Button,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Select => "select",
            Self::Hybrid => "hybrid",
        }
    }
}

#[derive(Clone)]
struct HelpPage {
    key: &'static str,
    title: &'static str,
    description: &'static str,
}

#[derive(Clone, Copy)]
struct CommandDoc {
    key: &'static str,
    command: &'static str,
    params: &'static str,
    summary: &'static str,
    description: &'static str,
    examples: &'static [&'static str],
    alias_source_key: Option<&'static str>,
}

#[derive(Clone, Copy)]
struct HelpState {
    layout: HelpLayout,
    aliases_enabled: bool,
    perms_enabled: bool,
}

const HELP_FALLBACK_EXAMPLES: &[&str] = &["+help", "+help ping"];
const HELP_PAGES: &[HelpPage] = &[
    HelpPage {
        key: "infos",
        title: "Infos",
        description: "Informations sur le serveur, les membres et les profils.",
    },
    HelpPage {
        key: "logs",
        title: "Logs",
        description: "Configuration des logs de modération, messages, vocal et boosts.",
    },
    HelpPage {
        key: "moderation",
        title: "Modération",
        description: "Sanctions, nettoyage et commandes de modération générale.",
    },
    HelpPage {
        key: "roles",
        title: "Rôles",
        description: "Gestion des rôles individuels, temporaires et massifs.",
    },
    HelpPage {
        key: "salons_vocal",
        title: "Salons & Vocal",
        description: "Gestion des salons texte et commandes de déplacement vocal.",
    },
    HelpPage {
        key: "outils",
        title: "Outils",
        description: "Giveaways, utilitaires, embeds et automatisations de contenu.",
    },
    HelpPage {
        key: "bot",
        title: "Bot & Présence",
        description: "Configuration du bot, thème, activité et présence.",
    },
    HelpPage {
        key: "administration",
        title: "Administration",
        description: "Owners, blacklist, préfixes, MP et configuration globale.",
    },
    HelpPage {
        key: "permissions",
        title: "Permissions & Aide",
        description: "Permissions, alias et configuration de l'interface d'aide.",
    },
];

fn help_page_for_command(
    meta: &crate::commands::command_contract::CommandMetadata,
) -> &'static str {
    match meta.key {
        "modlog" | "messagelog" | "voicelog" | "boostlog" | "rolelog" | "raidlog"
        | "autoconfiglog" | "nolog" | "join" | "boostembed" | "set_modlogs" | "set_boostembed"
        | "leave_settings" | "viewlogs" => "logs",
        "warn"
        | "mute"
        | "tempmute"
        | "unmute"
        | "cmute"
        | "tempcmute"
        | "uncmute"
        | "mutelist"
        | "unmuteall"
        | "kick"
        | "ban"
        | "tempban"
        | "unban"
        | "banlist"
        | "unbanall"
        | "sanctions"
        | "del_sanction"
        | "clear_sanctions"
        | "clear_all_sanctions"
        | "cleanup"
        | "renew"
        | "clear_messages" => "moderation",
        "addrole" | "delrole" | "derank" | "massiverole" | "unmassiverole" | "temprole"
        | "untemprole" | "sync" => "roles",
        "lock" | "unlock" | "lockall" | "unlockall" | "hide" | "unhide" | "hideall"
        | "unhideall" | "voicemove" | "voicekick" | "bringall" => "salons_vocal",
        "giveaway" | "end" | "reroll" | "choose" | "calc" | "emoji" | "embed" | "say"
        | "create" | "newsticker" | "button" | "autoreact" | "snipe" | "loading" | "backup"
        | "autobackup" => "outils",
        "shadowbot" | "set" | "theme" | "playto" | "listen" | "watch" | "compet" | "stream"
        | "remove_activity" | "online" | "idle" | "dnd" | "invisible" | "change" | "changeall" => {
            "bot"
        }
        "owner" | "unowner" | "clear_owners" | "bl" | "unbl" | "blinfo" | "clear_bl"
        | "allbots" | "alladmins" | "botadmins" | "mainprefix" | "prefix" | "mp" | "invite"
        | "leave" | "discussion" => "administration",
        "perms" | "del" | "clear_perms" | "allperms" | "alias" | "help" | "helptype"
        | "helpalias" => "permissions",
        _ => match meta.category {
            "general" => "infos",
            "profile" => "bot",
            "admin" => "administration",
            "permissions" => "permissions",
            _ => "infos",
        },
    }
}

fn help_page_title(key: &str) -> &'static str {
    HELP_PAGES
        .iter()
        .find(|page| page.key == key)
        .map(|page| page.title)
        .unwrap_or("Infos")
}

fn help_page_title_for_command_key(key: &str) -> &'static str {
    crate::commands::command_metadata_by_key(key)
        .map(|meta| help_page_title(help_page_for_command(&meta)))
        .unwrap_or("Infos")
}

fn help_metadata_lookup_key(input: &str) -> Option<&'static str> {
    let normalized = help_lookup_key(input);
    let underscored = normalized.replace(' ', "_");

    crate::commands::all_command_metadata()
        .into_iter()
        .find(|meta| {
            meta.key.eq_ignore_ascii_case(&normalized)
                || meta.key.eq_ignore_ascii_case(&underscored)
                || meta.command.eq_ignore_ascii_case(&normalized)
        })
        .map(|meta| meta.key)
}

fn help_page_matches_input(page: &HelpPage, input: &str) -> bool {
    let normalized = help_lookup_key(input);
    let aliases = match page.key {
        "infos" => &["general", "info", "informations"][..],
        "logs" => &["log", "journal"][..],
        "moderation" => &["mod", "sanction"][..],
        "roles" => &["role", "roles"][..],
        "salons_vocal" => &["salon", "salons", "vocal", "voice", "channels"][..],
        "outils" => &["utilitaires", "tools", "giveaway"][..],
        "bot" => &["profil", "presence", "activite", "activity"][..],
        "administration" => &["admin", "admins"][..],
        "permissions" => &["permission", "perms", "aide", "help"][..],
        _ => &[][..],
    };

    page.key.eq_ignore_ascii_case(&normalized)
        || help_lookup_key(page.title).eq_ignore_ascii_case(&normalized)
        || aliases
            .iter()
            .any(|alias| alias.eq_ignore_ascii_case(&normalized))
}

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

async fn current_help_state(ctx: &Context) -> HelpState {
    let bot_id = ctx.cache.current_user().id;
    let pool = pool(ctx).await;

    let layout = if let Some(pool) = &pool {
        get_help_type(pool, bot_id)
            .await
            .ok()
            .flatten()
            .map(|value| HelpLayout::from_str(&value))
            .unwrap_or(HelpLayout::Button)
    } else {
        HelpLayout::Button
    };

    let aliases_enabled = if let Some(pool) = &pool {
        get_help_aliases_enabled(pool, bot_id)
            .await
            .ok()
            .flatten()
            .unwrap_or(true)
    } else {
        true
    };

    let perms_enabled = if let Some(pool) = &pool {
        get_help_perms_enabled(pool, bot_id)
            .await
            .ok()
            .flatten()
            .unwrap_or(true)
    } else {
        true
    };

    HelpState {
        layout,
        aliases_enabled,
        perms_enabled,
    }
}

async fn aliases_map(ctx: &Context) -> BTreeMap<String, Vec<String>> {
    let bot_id = ctx.cache.current_user().id;
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for meta in crate::commands::all_command_metadata() {
        if !meta.default_aliases.is_empty() {
            out.entry(meta.alias_source_key.to_string())
                .or_default()
                .extend(meta.default_aliases.iter().map(|alias| alias.to_string()));
        }
    }

    if let Some(pool) = pool(ctx).await {
        if let Ok(rows) = list_command_aliases(&pool, bot_id).await {
            for (alias, command) in rows {
                out.entry(command).or_default().push(alias);
            }
        }
    }

    for aliases in out.values_mut() {
        aliases.sort();
        aliases.dedup();
    }

    out
}

fn command_doc(key: &str) -> Option<CommandDoc> {
    let meta = match key {
        "mp_settings" | "mp_sent" | "mp_delete" => crate::commands::command_metadata_by_key("mp")?,
        "server_list" => crate::commands::command_metadata_by_key("server")?,
        "change_reset" => crate::commands::command_metadata_by_key("change")?,
        "set_perm" => crate::commands::command_metadata_by_key("set")?,
        "del_perm" => crate::commands::command_metadata_by_key("del")?,
        other => crate::commands::command_metadata_by_key(other)?,
    };

    Some(CommandDoc {
        key: meta.key,
        command: meta.command,
        params: meta.params,
        summary: meta.summary,
        description: meta.description,
        examples: if meta.examples.is_empty() {
            HELP_FALLBACK_EXAMPLES
        } else {
            meta.examples
        },
        alias_source_key: Some(meta.alias_source_key),
    })
}

fn help_lookup_key(input: &str) -> String {
    input
        .trim()
        .trim_start_matches('+')
        .to_lowercase()
        .replace('_', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn help_lookup_to_key(input: &str) -> Option<&'static str> {
    let matched = match help_lookup_key(input).as_str() {
        "help" => Some("help"),
        "ping" => Some("ping"),
        "allbots" => Some("allbots"),
        "alladmins" => Some("alladmins"),
        "botadmins" => Some("botadmins"),
        "boosters" => Some("boosters"),
        "rolemembers" => Some("rolemembers"),
        "serverinfo" => Some("serverinfo"),
        "vocinfo" => Some("vocinfo"),
        "role" => Some("role"),
        "channel" => Some("channel"),
        "user" => Some("user"),
        "member" => Some("member"),
        "pic" => Some("pic"),
        "banner" => Some("banner"),
        "server" => Some("server"),
        "server list" => Some("server_list"),
        "snipe" => Some("snipe"),
        "emoji" => Some("emoji"),
        "giveaway" => Some("giveaway"),
        "end" | "end giveaway" => Some("end"),
        "reroll" => Some("reroll"),
        "choose" => Some("choose"),
        "embed" => Some("embed"),
        "backup" | "backup list" | "backup delete" | "backup load" => Some("backup"),
        "autobackup" => Some("autobackup"),
        "loading" => Some("loading"),
        "create" => Some("create"),
        "newsticker" => Some("newsticker"),
        "massiverole" => Some("massiverole"),
        "unmassiverole" => Some("unmassiverole"),
        "voicemove" => Some("voicemove"),
        "voicekick" => Some("voicekick"),
        "cleanup" => Some("cleanup"),
        "bringall" => Some("bringall"),
        "renew" => Some("renew"),
        "unbanall" => Some("unbanall"),
        "temprole" => Some("temprole"),
        "untemprole" => Some("untemprole"),
        "sync" => Some("sync"),
        "button" => Some("button"),
        "autoreact" => Some("autoreact"),
        "calc" => Some("calc"),
        "shadowbot" => Some("shadowbot"),
        "set" => Some("set"),
        "theme" => Some("theme"),
        "playto" => Some("playto"),
        "listen" => Some("listen"),
        "watch" => Some("watch"),
        "compet" => Some("compet"),
        "stream" => Some("stream"),
        "remove activity" => Some("remove_activity"),
        "online" => Some("online"),
        "idle" => Some("idle"),
        "dnd" => Some("dnd"),
        "invisible" => Some("invisible"),
        "mp" => Some("mp"),
        "mp settings" => Some("mp_settings"),
        "mp sent" => Some("mp_sent"),
        "mp delete" | "mp del" => Some("mp_delete"),
        "discussion" => Some("discussion"),
        "owner" => Some("owner"),
        "unowner" => Some("unowner"),
        "clear owners" => Some("clear_owners"),
        "bl" => Some("bl"),
        "unbl" => Some("unbl"),
        "blinfo" => Some("blinfo"),
        "clear bl" => Some("clear_bl"),
        "say" => Some("say"),
        "invite" => Some("invite"),
        "leave" => Some("leave"),
        "change" => Some("change"),
        "change reset" => Some("change_reset"),
        "changeall" => Some("changeall"),
        "mainprefix" => Some("mainprefix"),
        "prefix" => Some("prefix"),
        "perms" => Some("perms"),
        "allperms" => Some("allperms"),
        "set perm" => Some("set_perm"),
        "del perm" => Some("del_perm"),
        "clear perms" => Some("clear_perms"),
        "alias" => Some("alias"),
        "helpsetting" => Some("helpsetting"),
        _ => None,
    };

    matched.or_else(|| help_metadata_lookup_key(input))
}

fn help_page_index(key: &str) -> Option<usize> {
    HELP_PAGES
        .iter()
        .position(|page| help_page_matches_input(page, key))
}

fn help_page_from_input(input: &str) -> Option<usize> {
    if let Ok(index) = input.parse::<usize>() {
        if index >= 1 && index <= HELP_PAGES.len() {
            return Some(index - 1);
        }
    }

    help_page_index(input)
}

fn format_permission_level(level: u8) -> String {
    format!("[{}]", level)
}

fn permission_level_description(level: u8) -> &'static str {
    match level {
        0 => "[0] Public",
        2 => "[2] Accès spécial",
        8 => "[8] Modérateur+",
        9 => "[9] Propriétaire",
        _ => "[?] Inconnu",
    }
}

fn help_page_content(
    page: &HelpPage,
    alias_map: &BTreeMap<String, Vec<String>>,
    aliases_enabled: bool,
    perms_enabled: bool,
) -> Vec<String> {
    let mut commands = crate::commands::all_command_metadata()
        .into_iter()
        .filter(|meta| help_page_for_command(meta).eq_ignore_ascii_case(page.key))
        .collect::<Vec<_>>();
    commands.sort_by(|a, b| a.command.to_lowercase().cmp(&b.command.to_lowercase()));

    let mut lines = Vec::with_capacity(commands.len());

    for meta in commands {
        let label = meta.command;
        let summary = meta.summary;
        let alias_key = meta.alias_source_key;
        let permission = if perms_enabled {
            format!(" {}", format_permission_level(permissions::default_permission(meta.key)))
        } else {
            String::new()
        };

        if aliases_enabled {
            if let Some(aliases) = alias_map.get(alias_key) {
                if aliases.is_empty() {
                    lines.push(format!("`+{}`{} - {}", label, permission, summary));
                } else {
                    lines.push(format!(
                        "`+{}`{} - {} · alias: `{}`",
                        label, permission, summary,
                        aliases.join("`, `")
                    ));
                }
                continue;
            }
        }

        lines.push(format!("`+{}`{} - {}", label, permission, summary));
    }

    if lines.is_empty() {
        lines.push("Aucune commande dans cette catégorie.".to_string());
    }

    lines
}

fn build_help_embed(
    page_index: usize,
    state: &HelpState,
    alias_map: &BTreeMap<String, Vec<String>>,
) -> CreateEmbed {
    let page = &HELP_PAGES[page_index];
    let lines = help_page_content(page, alias_map, state.aliases_enabled, state.perms_enabled);

    let mut embed = CreateEmbed::new()
        .title(format!("Aide · {}", page.title))
        .description(format!(
            "Page {}/{} · mode `{}` · aliases {} · perms {}\n{}",
            page_index + 1,
            HELP_PAGES.len(),
            state.layout.as_str(),
            if state.aliases_enabled {
                "activés"
            } else {
                "désactivés"
            },
            if state.perms_enabled {
                "activées"
            } else {
                "désactivées"
            },
            page.description,
        ))
        .color(0x5865F2);

    embed = add_list_fields(embed, &lines, "Commandes");
    embed
}

fn help_components(owner_id: UserId, page_index: usize, state: &HelpState) -> Vec<CreateActionRow> {
    let total = HELP_PAGES.len().max(1);
    let prev_page = page_index.saturating_sub(1);
    let next_page = (page_index + 1).min(total - 1);
    let custom_prev = format!("help:nav:{}:{}", owner_id.get(), prev_page);
    let custom_next = format!("help:nav:{}:{}", owner_id.get(), next_page);

    let mut rows = Vec::new();

    match state.layout {
        HelpLayout::Button | HelpLayout::Hybrid => {
            rows.push(CreateActionRow::Buttons(vec![
                CreateButton::new(custom_prev)
                    .label("◀ Précédent")
                    .style(ButtonStyle::Primary)
                    .disabled(page_index == 0),
                CreateButton::new(custom_next)
                    .label("Suivant ▶")
                    .style(ButtonStyle::Primary)
                    .disabled(page_index + 1 >= total),
            ]));
        }
        HelpLayout::Select => {}
    }

    match state.layout {
        HelpLayout::Select | HelpLayout::Hybrid => {
            let options = HELP_PAGES
                .iter()
                .enumerate()
                .map(|(index, page)| {
                    CreateSelectMenuOption::new(page.title, index.to_string())
                        .description(truncate_text(page.description, 100))
                })
                .collect::<Vec<_>>();

            let menu = CreateSelectMenu::new(
                format!("help:select:{}", owner_id.get()),
                CreateSelectMenuKind::String { options },
            )
            .placeholder("Choisir une page d'aide");

            rows.push(CreateActionRow::SelectMenu(menu));
        }
        HelpLayout::Button => {}
    }

    rows
}

fn parse_help_component_id(custom_id: &str) -> Option<(&str, u64, Option<usize>)> {
    let parts = custom_id.split(':').collect::<Vec<_>>();
    if parts.len() < 3 || parts.first().copied()? != "help" {
        return None;
    }

    let kind = parts.get(1).copied()?;
    let owner_id = parts.get(2)?.parse::<u64>().ok()?;
    let page = parts.get(3).and_then(|value| value.parse::<usize>().ok());
    Some((kind, owner_id, page))
}

pub async fn register_slash_help(ctx: &Context) {
    let _ = Command::create_global_command(
        &ctx.http,
        CreateCommand::new("help").description("Affiche l'aide du bot"),
    )
    .await;
}

pub async fn handle_help_slash(ctx: &Context, command: &CommandInteraction) {
    let state = current_help_state(ctx).await;
    let alias_map = aliases_map(ctx).await;
    let embed = build_help_embed(0, &state, &alias_map);
    let components = help_components(command.user.id, 0, &state);

    let _ = command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(embed)
                    .components(components),
            ),
        )
        .await;
}

pub async fn handle_slash_interaction(ctx: &Context, interaction: &Interaction) -> bool {
    let Interaction::Command(command) = interaction else {
        return false;
    };

    if command.data.name != "help" {
        return false;
    }

    handle_help_slash(ctx, command).await;
    true
}

pub async fn handle_help_component(ctx: &Context, component: &ComponentInteraction) -> bool {
    let Some((kind, owner_id, page)) = parse_help_component_id(&component.data.custom_id) else {
        return false;
    };

    if component.user.id.get() != owner_id {
        let _ = component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Seul l'auteur de l'aide peut utiliser ces contrôles.")
                        .ephemeral(true),
                ),
            )
            .await;
        return true;
    }

    let state = current_help_state(ctx).await;
    let alias_map = aliases_map(ctx).await;
    let page_index = match kind {
        "nav" => page.unwrap_or(0).min(HELP_PAGES.len().saturating_sub(1)),
        "select" => match &component.data.kind {
            ComponentInteractionDataKind::StringSelect { values } => values
                .first()
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(0)
                .min(HELP_PAGES.len().saturating_sub(1)),
            _ => 0,
        },
        _ => 0,
    };

    let embed = build_help_embed(page_index, &state, &alias_map);
    let components = help_components(component.user.id, page_index, &state);

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

pub async fn handle_help(ctx: &Context, msg: &Message, args: &[&str]) {
    let state = current_help_state(ctx).await;
    let alias_map = aliases_map(ctx).await;

    if !args.is_empty() {
        let joined = args.join(" ");

        let mut resolved_key = help_lookup_to_key(&joined).map(|s| s.to_string());
        if resolved_key.is_none() {
            let first = args[0];
            if let Some(key) = help_lookup_to_key(first) {
                resolved_key = Some(key.to_string());
            }
        }

        if resolved_key.is_none() {
            let alias_input = help_lookup_key(&joined).replace(' ', "_");
            if let Some(alias_target) = resolve_alias(ctx, &alias_input).await {
                resolved_key = Some(alias_target);
            }
        }

        if resolved_key.is_none() {
            resolved_key = help_metadata_lookup_key(&joined).map(|key| key.to_string());
        }

        if let Some(key) = resolved_key {
            if let Some(doc) = command_doc(&key) {
                let aliases = doc
                    .alias_source_key
                    .and_then(|alias_key| alias_map.get(alias_key))
                    .cloned()
                    .unwrap_or_default();

                let alias_text = if aliases.is_empty() {
                    "Aucun alias".to_string()
                } else {
                    aliases
                        .iter()
                        .map(|alias| format!("`{}`", alias))
                        .collect::<Vec<_>>()
                        .join(", ")
                };

                let examples = doc
                    .examples
                    .iter()
                    .map(|ex| format!("`{}`", ex))
                    .collect::<Vec<_>>()
                    .join("\n");

                let embed = CreateEmbed::new()
                    .title(format!("Aide commande · +{}", doc.command))
                    .description(doc.description)
                    .field("Commande", format!("`+{}`", doc.command), false)
                    .field("Clé ACL", format!("`{}`", doc.key), false)
                    .field("Catégorie", help_page_title_for_command_key(doc.key), false)
                    .field("Permission", permission_level_description(permissions::default_permission(doc.key)), false)
                    .field("Alias", alias_text, false)
                    .field("Paramètres", doc.params, false)
                    .field("Résumé", doc.summary, false)
                    .field("Exemples", truncate_text(&examples, 1024), false)
                    .color(crate::commands::common::theme_color(ctx).await);

                let _ = msg
                    .channel_id
                    .send_message(&ctx.http, CreateMessage::new().embed(embed))
                    .await;
                return;
            }
        }
    }

    let page_index = args
        .first()
        .and_then(|input| help_page_from_input(input))
        .unwrap_or(0)
        .min(HELP_PAGES.len().saturating_sub(1));

    let embed = build_help_embed(page_index, &state, &alias_map);
    let components = help_components(msg.author.id, page_index, &state);

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new().embed(embed).components(components),
        )
        .await;
}
pub struct HelpCommand;
pub static COMMAND_DESCRIPTOR: HelpCommand = HelpCommand;

impl crate::commands::command_contract::CommandSpec for HelpCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "help",
            command: "help",
            category: "general",
            params: "[commande|page]",
            summary: "Affiche laide des commandes",
            description: "Affiche les pages daide du bot ou la fiche detaillee dune commande avec parametres, aliases et exemples.",
            examples: &["+help", "+hp", "+help help"],
            alias_source_key: "help",
            default_aliases: &["hp"],
        }
    }
}
