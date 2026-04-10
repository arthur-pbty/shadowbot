use std::collections::HashMap;

use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed, CreateMessage};
use serenity::model::application::ButtonStyle;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::db::DbPoolKey;

const LOGS_PER_PAGE: i64 = 10;

fn total_pages(total: i64) -> i64 {
    ((total + LOGS_PER_PAGE - 1) / LOGS_PER_PAGE).max(1)
}

pub async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

async fn fetch_log_channels(
    pool: &sqlx::PgPool,
    bot_id: UserId,
    guild_id: GuildId,
) -> HashMap<String, u64> {
    let rows = sqlx::query_as::<_, (String, Option<i64>)>(
        r#"
        SELECT log_type, channel_id
        FROM bot_log_channels
        WHERE bot_id = $1 AND guild_id = $2 AND enabled = TRUE;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .filter_map(|(log_type, channel_id)| {
            channel_id
                .and_then(|id| u64::try_from(id).ok())
                .map(|id| (log_type, id))
        })
        .collect()
}

fn extract_log_channel_id(
    log: &crate::db::AuditLog,
    channels: &HashMap<String, u64>,
) -> Option<u64> {
    let from_details = log
        .details
        .as_ref()
        .and_then(|details| details.get("log_channel_id"))
        .and_then(|value| value.as_i64())
        .and_then(|id| u64::try_from(id).ok());

    from_details.or_else(|| channels.get(&log.log_type).copied())
}

fn build_log_link(
    log: &crate::db::AuditLog,
    guild_id: GuildId,
    channels: &HashMap<String, u64>,
) -> Option<String> {
    let message_id = log.message_id.and_then(|id| u64::try_from(id).ok())?;
    let log_channel_id = extract_log_channel_id(log, channels)?;

    Some(format!(
        "https://discord.com/channels/{}/{}/{}",
        guild_id.get(),
        log_channel_id,
        message_id
    ))
}

fn append_log_fields(
    mut embed: CreateEmbed,
    logs: Vec<crate::db::AuditLog>,
    guild_id: GuildId,
    channels: &HashMap<String, u64>,
) -> CreateEmbed {
    for log in logs {
        let actor = log
            .user_id
            .map(|id| format!("<@{}>", id))
            .unwrap_or_else(|| "Systeme".to_string());

        let target = match (log.channel_id, log.role_id) {
            (Some(ch_id), _) => format!("<#{}>", ch_id),
            (_, Some(role_id)) => format!("<@&{}>", role_id),
            _ => "-".to_string(),
        };

        let link = build_log_link(&log, guild_id, channels)
            .map(|url| format!("[Voir]({})", url))
            .unwrap_or_else(|| "indisponible".to_string());

        embed = embed.field(
            format!("[{}] {}", log.log_type.to_uppercase(), log.action),
            format!(
                "Acteur: {}\nQuand: <t:{}:R>\nCible: {}\nLien: {}",
                actor,
                log.created_at.timestamp(),
                target,
                link
            ),
            false,
        );
    }

    embed
}

pub async fn handle_viewlogs(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("ViewLogs")
                .description("Cette commande ne fonctionne que sur les serveurs.")
                .color(0xED4245),
        )
        .await;
        return;
    };

    let Some(pool) = pool(ctx).await else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("ViewLogs")
                .description("Erreur de connexion à la DB.")
                .color(0xED4245),
        )
        .await;
        return;
    };

    let bot_id = ctx.cache.current_user().id;
    let page = args
        .first()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(1)
        .max(1);

    let total = crate::db::count_audit_logs(&pool, bot_id, guild_id)
        .await
        .unwrap_or(0);

    let total_pages = total_pages(total);

    if page > total_pages {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("ViewLogs")
                .description(format!("Page {} n'existe pas.", page))
                .color(0xED4245),
        )
        .await;
        return;
    }

    let offset = (page - 1) * LOGS_PER_PAGE;
    let logs = crate::db::get_audit_logs(&pool, bot_id, guild_id, LOGS_PER_PAGE, offset)
        .await
        .unwrap_or_default();

    let log_channels = fetch_log_channels(&pool, bot_id, guild_id).await;

    let embed = CreateEmbed::new()
        .title("Logs d'audit")
        .description(format!("Page {}/{} • {} logs", page, total_pages, total))
        .color(theme_color(ctx).await);

    let embed = append_log_fields(embed, logs, guild_id, &log_channels);

    let mut components = Vec::new();

    if total_pages > 1 {
        let prev_button =
            CreateButton::new(format!("viewlogs:prev:{}:{}", msg.author.id.get(), page))
                .label("◀ Précédent")
                .style(ButtonStyle::Primary)
                .disabled(page <= 1);

        let next_button =
            CreateButton::new(format!("viewlogs:next:{}:{}", msg.author.id.get(), page))
                .label("Suivant ▶")
                .style(ButtonStyle::Primary)
                .disabled(page >= total_pages);

        components.push(CreateActionRow::Buttons(vec![prev_button, next_button]));
    }

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new().embed(embed).components(components),
        )
        .await;
}

#[allow(dead_code)]
pub async fn handle_viewlogs_button(ctx: &Context, component: &ComponentInteraction) -> bool {
    let custom_id = &component.data.custom_id;
    if !custom_id.starts_with("viewlogs:") {
        return false;
    }

    let parts: Vec<&str> = custom_id.split(':').collect();
    if parts.len() < 4 {
        return false;
    }

    let direction = parts[1];
    let Ok(expected_user_id) = parts[2].parse::<u64>() else {
        return false;
    };
    let current_page = parts[3].parse::<i64>().unwrap_or(1).max(1);

    if component.user.id.get() != expected_user_id {
        let _ = component
            .create_response(
                &ctx.http,
                serenity::builder::CreateInteractionResponse::Message(
                    serenity::builder::CreateInteractionResponseMessage::new()
                        .content("Seul l'auteur peut utiliser ces boutons.")
                        .ephemeral(true),
                ),
            )
            .await;
        return true;
    }

    let new_page = match direction {
        "prev" => (current_page - 1).max(1),
        "next" => current_page + 1,
        _ => current_page,
    };

    let Some(guild_id) = component.guild_id else {
        return true;
    };
    let Some(pool) = pool(ctx).await else {
        return true;
    };

    let bot_id = ctx.cache.current_user().id;
    let total = crate::db::count_audit_logs(&pool, bot_id, guild_id)
        .await
        .unwrap_or(0);

    let total_pages = total_pages(total);
    let new_page = new_page.clamp(1, total_pages);

    let offset = (new_page - 1) * LOGS_PER_PAGE;
    let logs = crate::db::get_audit_logs(&pool, bot_id, guild_id, LOGS_PER_PAGE, offset)
        .await
        .unwrap_or_default();

    let log_channels = fetch_log_channels(&pool, bot_id, guild_id).await;

    let embed = CreateEmbed::new()
        .title("Logs d'audit")
        .description(format!(
            "Page {}/{} • {} logs",
            new_page, total_pages, total
        ))
        .color(theme_color(ctx).await);

    let embed = append_log_fields(embed, logs, guild_id, &log_channels);

    let mut components = Vec::new();
    if total_pages > 1 {
        let prev_button = CreateButton::new(format!(
            "viewlogs:prev:{}:{}",
            component.user.id.get(),
            new_page
        ))
        .label("◀ Précédent")
        .style(ButtonStyle::Primary)
        .disabled(new_page <= 1);

        let next_button = CreateButton::new(format!(
            "viewlogs:next:{}:{}",
            component.user.id.get(),
            new_page
        ))
        .label("Suivant ▶")
        .style(ButtonStyle::Primary)
        .disabled(new_page >= total_pages);

        components.push(CreateActionRow::Buttons(vec![prev_button, next_button]));
    }

    let _ = component
        .create_response(
            &ctx.http,
            serenity::builder::CreateInteractionResponse::UpdateMessage(
                serenity::builder::CreateInteractionResponseMessage::new()
                    .embed(embed)
                    .components(components),
            ),
        )
        .await;

    true
}

pub struct ViewLogsCommand;
pub static COMMAND_DESCRIPTOR: ViewLogsCommand = ViewLogsCommand;

impl crate::commands::command_contract::CommandSpec for ViewLogsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "viewlogs",
            category: "config",
            params: "[page]",
            description: "Affiche les derniers logs d'audit du serveur avec pagination. Les logs incluent tous les événements (modération, messages, rôles, salons, etc.)",
            examples: &["+viewlogs", "+viewlogs 2"],
            default_aliases: &["vlogs", "audit"],
            allow_in_dm: false,
            default_permission: 5,
        }
    }
}
