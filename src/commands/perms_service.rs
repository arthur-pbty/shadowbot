use serenity::builder::{
    CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{add_list_fields, send_embed, theme_color, truncate_text};
use crate::db::{
    DbPoolKey, clear_role_permissions, grant_command_access, grant_perm_level,
    list_role_command_access, list_role_perm_levels, list_role_scopes, remove_scope_permissions,
    reset_command_permissions, set_command_permission, set_guild_prefix, set_main_prefix,
};
use crate::permissions::{
    all_command_keys, command_required_permission, default_permission, is_owner_user,
};

const ALLPERMS_PAGE_SIZE: usize = 12;
const ALLPERMS_CUSTOM_ID_PREFIX: &str = "allperms";

fn parse_user_or_role(input: &str) -> Option<(&'static str, u64)> {
    let trimmed = input.trim();
    if trimmed.starts_with("<@&") && trimmed.ends_with('>') {
        return trimmed
            .trim_start_matches("<@&")
            .trim_end_matches('>')
            .parse::<u64>()
            .ok()
            .map(|id| ("role", id));
    }

    if (trimmed.starts_with("<@") && trimmed.ends_with('>')) || trimmed.parse::<u64>().is_ok() {
        let cleaned = trimmed
            .trim_start_matches('<')
            .trim_end_matches('>')
            .trim_start_matches('@')
            .trim_start_matches('!');
        if let Ok(id) = cleaned.parse::<u64>() {
            return Some(("user", id));
        }
    }

    None
}

fn normalize_command_name(input: &str) -> String {
    input
        .trim_start_matches('+')
        .replace(' ', "_")
        .to_lowercase()
}

async fn ensure_owner(ctx: &Context, msg: &Message) -> bool {
    if is_owner_user(ctx, msg.author.id).await {
        true
    } else {
        let embed = CreateEmbed::new()
            .title("Accès refusé")
            .description("Commande réservée aux owners.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        false
    }
}

pub async fn handle_change(ctx: &Context, msg: &Message, args: &[&str]) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    let Some(pool) = pool else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if args
        .first()
        .map(|s| s.eq_ignore_ascii_case("reset"))
        .unwrap_or(false)
    {
        let removed = reset_command_permissions(&pool, bot_id).await.unwrap_or(0);
        let embed = CreateEmbed::new()
            .title("Permissions réinitialisées")
            .description(format!("Overrides supprimés: {}", removed))
            .color(0x57F287);
        send_embed(ctx, msg, embed).await;
        return;
    }

    if args.len() < 2 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `change <commande> <permission>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let command = normalize_command_name(args[0]);
    let Ok(level) = args[1].parse::<u8>() else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Permission invalide (0..9).`).")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if level > 9 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Permission invalide (0..9).`).")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let _ = set_command_permission(&pool, bot_id, &command, level).await;
    let embed = CreateEmbed::new()
        .title("Permission modifiée")
        .description(format!("`{}` -> niveau `{}`", command, level))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub async fn handle_changeall(ctx: &Context, msg: &Message, args: &[&str]) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    if args.len() < 2 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `changeall <permission> <permission>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let Ok(from) = args[0].parse::<u8>() else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Permission source invalide.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };
    let Ok(to) = args[1].parse::<u8>() else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Permission cible invalide.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if from > 9 || to > 9 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Permissions valides: 0..9")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    let Some(pool) = pool else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let mut updated = 0usize;
    for cmd in all_command_keys() {
        let current = command_required_permission(ctx, &cmd).await;
        if current == from {
            let _ = set_command_permission(&pool, bot_id, &cmd, to).await;
            updated += 1;
        }
    }

    let embed = CreateEmbed::new()
        .title("Changeall appliqué")
        .description(format!("{} commande(s): {} -> {}", updated, from, to))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub async fn handle_mainprefix(ctx: &Context, msg: &Message, args: &[&str]) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `mainprefix <préfixe>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let prefix = args[0].trim();
    if prefix.is_empty() || prefix.len() > 5 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Préfixe invalide (1 à 5 caractères).`).")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };
    if let Some(pool) = pool {
        let _ = set_main_prefix(&pool, bot_id, prefix).await;
    }

    let embed = CreateEmbed::new()
        .title("Préfixe principal mis à jour")
        .description(format!("Nouveau préfixe principal: `{}`", prefix))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub async fn handle_prefix(ctx: &Context, msg: &Message, args: &[&str]) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    let Some(guild_id) = msg.guild_id else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Commande disponible uniquement sur un serveur.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `prefix <préfixe>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let prefix = args[0].trim();
    if prefix.is_empty() || prefix.len() > 5 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Préfixe invalide (1 à 5 caractères).`).")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    if let Some(pool) = pool {
        let _ = set_guild_prefix(&pool, bot_id, guild_id, prefix).await;
    }

    let embed = CreateEmbed::new()
        .title("Préfixe serveur mis à jour")
        .description(format!("Nouveau préfixe ici: `{}`", prefix))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub async fn handle_set_perm(ctx: &Context, msg: &Message, args: &[&str]) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    if args.len() < 3 || !args[0].eq_ignore_ascii_case("perm") {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `set perm <permission/commande> <rôle/membre>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let target = parse_user_or_role(args[2]);
    let Some((scope_type, scope_id)) = target else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Rôle/membre invalide.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };
    let Some(pool) = pool else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if let Ok(level) = args[1].parse::<u8>() {
        if level > 9 {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description("Permission invalide (0..9).`).")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        }

        let _ = grant_perm_level(&pool, bot_id, scope_type, scope_id, level).await;
        let who = if scope_type == "role" {
            format!("<@&{}>", scope_id)
        } else {
            format!("<@{}>", scope_id)
        };
        let embed = CreateEmbed::new()
            .title("Permission attribuée")
            .description(format!("{} reçoit la permission `{}`", who, level))
            .color(0x57F287);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let command = normalize_command_name(args[1]);
    let _ = grant_command_access(&pool, bot_id, scope_type, scope_id, &command).await;
    let who = if scope_type == "role" {
        format!("<@&{}>", scope_id)
    } else {
        format!("<@{}>", scope_id)
    };

    let embed = CreateEmbed::new()
        .title("Accès commande attribué")
        .description(format!("{} reçoit l'accès direct à `{}`", who, command))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub async fn handle_del_perm(ctx: &Context, msg: &Message, args: &[&str]) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    if args.len() < 2 || !args[0].eq_ignore_ascii_case("perm") {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `del perm <rôle>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let target = parse_user_or_role(args[1]);
    let Some((scope_type, scope_id)) = target else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Rôle/membre invalide.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };
    let Some(pool) = pool else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let removed = remove_scope_permissions(&pool, bot_id, scope_type, scope_id)
        .await
        .unwrap_or(0);

    let embed = CreateEmbed::new()
        .title("Permissions supprimées")
        .description(format!("{} entrée(s) supprimée(s).", removed))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub async fn handle_clear_perms(ctx: &Context, msg: &Message) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };
    let Some(pool) = pool else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let removed = clear_role_permissions(&pool, bot_id).await.unwrap_or(0);
    let embed = CreateEmbed::new()
        .title("Permissions rôles supprimées")
        .description(format!("{} entrée(s) supprimée(s).", removed))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub async fn handle_perms(ctx: &Context, msg: &Message, _args: &[&str]) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };
    let Some(pool) = pool else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let roles = list_role_scopes(&pool, bot_id).await.unwrap_or_default();
    let mut lines = Vec::new();

    for rid in roles {
        let perm_levels = list_role_perm_levels(&pool, bot_id, rid as u64)
            .await
            .unwrap_or_default();
        let command_access = list_role_command_access(&pool, bot_id, rid as u64)
            .await
            .unwrap_or_default();

        let perms = if perm_levels.is_empty() {
            "aucun".to_string()
        } else {
            perm_levels
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(",")
        };

        let commands = if command_access.is_empty() {
            "aucune".to_string()
        } else {
            truncate_text(&command_access.join(", "), 80)
        };

        lines.push(format!(
            "<@&{}> · perms [{}] · cmd [{}]",
            rid, perms, commands
        ));
    }

    let mut embed = CreateEmbed::new().title("Permissions du bot");
    embed = add_list_fields(embed, &lines, "Rôles configurés");
    send_embed(ctx, msg, embed).await;
}

pub async fn handle_allperms(ctx: &Context, msg: &Message, _args: &[&str]) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    let lines = collect_allperms_lines(ctx).await;
    let total_pages = total_pages_for(lines.len());
    let requested_page = _args
        .first()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1)
        .saturating_sub(1);
    let page = requested_page.min(total_pages.saturating_sub(1));

    let color = theme_color(ctx).await;
    let embed = build_allperms_embed(&lines, page, color);
    let components = allperms_components(msg.author.id, page, total_pages);

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new().embed(embed).components(components),
        )
        .await;
}

pub async fn handle_allperms_component(ctx: &Context, component: &ComponentInteraction) -> bool {
    let Some((owner_id, requested_page)) = parse_allperms_custom_id(&component.data.custom_id)
    else {
        return false;
    };

    if component.user.id.get() != owner_id {
        let _ = component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Seul l'auteur de la commande peut utiliser ces boutons.")
                        .ephemeral(true),
                ),
            )
            .await;
        return true;
    }

    let lines = collect_allperms_lines(ctx).await;
    let total_pages = total_pages_for(lines.len());
    let page = requested_page.min(total_pages.saturating_sub(1));
    let color = theme_color(ctx).await;
    let embed = build_allperms_embed(&lines, page, color);
    let components = allperms_components(component.user.id, page, total_pages);

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

async fn collect_allperms_lines(ctx: &Context) -> Vec<String> {
    let mut commands = all_command_keys();
    if !commands.iter().any(|c| c == "allperms") {
        commands.push("allperms".to_string());
    }
    commands.sort();

    let mut lines = Vec::with_capacity(commands.len());
    for cmd in commands {
        let required = command_required_permission(ctx, &cmd).await;
        let default = default_permission(&cmd);

        if required == default {
            lines.push(format!("`{}` -> `{}`", cmd, required));
        } else {
            lines.push(format!(
                "`{}` -> `{}` (défaut `{}`)",
                cmd, required, default
            ));
        }
    }

    lines
}

fn total_pages_for(total_items: usize) -> usize {
    ((total_items + ALLPERMS_PAGE_SIZE.saturating_sub(1)) / ALLPERMS_PAGE_SIZE).max(1)
}

fn build_allperms_embed(lines: &[String], page: usize, color: u32) -> CreateEmbed {
    let total_pages = total_pages_for(lines.len());
    let safe_page = page.min(total_pages.saturating_sub(1));
    let start = safe_page * ALLPERMS_PAGE_SIZE;
    let end = (start + ALLPERMS_PAGE_SIZE).min(lines.len());
    let chunk = if start < end { &lines[start..end] } else { &[] };

    let value = if chunk.is_empty() {
        "Aucune commande.".to_string()
    } else {
        truncate_text(&chunk.join("\n"), 1024)
    };

    CreateEmbed::new()
        .title("Permissions de toutes les commandes")
        .description(format!(
            "{} commande(s) · Page {}/{}",
            lines.len(),
            safe_page + 1,
            total_pages
        ))
        .field("Niveaux requis", value, false)
        .color(color)
}

fn allperms_components(owner_id: UserId, page: usize, total_pages: usize) -> Vec<CreateActionRow> {
    let safe_total = total_pages.max(1);
    let safe_page = page.min(safe_total.saturating_sub(1));
    let prev_page = safe_page.saturating_sub(1);
    let next_page = (safe_page + 1).min(safe_total.saturating_sub(1));

    vec![CreateActionRow::Buttons(vec![
        CreateButton::new(format!(
            "{}:{}:{}",
            ALLPERMS_CUSTOM_ID_PREFIX,
            owner_id.get(),
            prev_page
        ))
        .label("◀ Précédent")
        .style(ButtonStyle::Primary)
        .disabled(safe_page == 0),
        CreateButton::new(format!(
            "{}:{}:{}",
            ALLPERMS_CUSTOM_ID_PREFIX,
            owner_id.get(),
            next_page
        ))
        .label("Suivant ▶")
        .style(ButtonStyle::Primary)
        .disabled(safe_page + 1 >= safe_total),
    ])]
}

fn parse_allperms_custom_id(custom_id: &str) -> Option<(u64, usize)> {
    let mut parts = custom_id.split(':');
    let prefix = parts.next()?;
    if prefix != ALLPERMS_CUSTOM_ID_PREFIX {
        return None;
    }

    let owner_id = parts.next()?.parse::<u64>().ok()?;
    let page = parts.next()?.parse::<usize>().ok()?;
    Some((owner_id, page))
}
