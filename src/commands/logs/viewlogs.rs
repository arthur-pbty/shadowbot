use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed, CreateMessage};
use serenity::model::application::ButtonStyle;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::db::DbPoolKey;

const LOGS_PER_PAGE: i64 = 10;

pub async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
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

    if page > (total / LOGS_PER_PAGE) + 1 && total > 0 {
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

    let mut embed = CreateEmbed::new()
        .title("Logs d'audit")
        .description(format!(
            "Page {}/{} ({} logs total)",
            page,
            (total / LOGS_PER_PAGE) + if total % LOGS_PER_PAGE > 0 { 1 } else { 0 },
            total
        ))
        .color(theme_color(ctx).await);

    for log in logs {
        let user_mention = log
            .user_id
            .map(|id| format!("<@{}>", id))
            .unwrap_or_else(|| "Système".to_string());

        let extra = match (log.channel_id, log.role_id) {
            (Some(ch_id), _) => format!(" · <#{}>", ch_id),
            (_, Some(role_id)) => format!(" · <@&{}>", role_id),
            _ => String::new(),
        };

        embed = embed.field(
            format!("[{}] {} {}", &log.log_type, user_mention, log.action),
            format!("<t:{}:R>{}", log.created_at.timestamp(), extra),
            false,
        );
    }

    let total_pages = (total / LOGS_PER_PAGE) + if total % LOGS_PER_PAGE > 0 { 1 } else { 0 };
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
    let user_id_str = parts[2];
    let current_page = parts[3].parse::<i64>().unwrap_or(1);

    if component.user.id.get().to_string() != user_id_str {
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

    let guild_id = component.guild_id.unwrap_or_else(|| GuildId::new(0));
    let Some(pool) = pool(ctx).await else {
        return true;
    };

    let bot_id = ctx.cache.current_user().id;
    let total = crate::db::count_audit_logs(&pool, bot_id, guild_id)
        .await
        .unwrap_or(0);

    let total_pages = (total / LOGS_PER_PAGE) + if total % LOGS_PER_PAGE > 0 { 1 } else { 0 };
    let new_page = new_page.min(total_pages);

    let offset = (new_page - 1) * LOGS_PER_PAGE;
    let logs = crate::db::get_audit_logs(&pool, bot_id, guild_id, LOGS_PER_PAGE, offset)
        .await
        .unwrap_or_default();

    let mut embed = CreateEmbed::new()
        .title("Logs d'audit")
        .description(format!(
            "Page {}/{} ({} logs total)",
            new_page, total_pages, total
        ))
        .color(theme_color(ctx).await);

    for log in logs {
        let user_mention = log
            .user_id
            .map(|id| format!("<@{}>", id))
            .unwrap_or_else(|| "Système".to_string());

        let extra = match (log.channel_id, log.role_id) {
            (Some(ch_id), _) => format!(" · <#{}>", ch_id),
            (_, Some(role_id)) => format!(" · <@&{}>", role_id),
            _ => String::new(),
        };

        embed = embed.field(
            format!("[{}] {} {}", &log.log_type, user_mention, log.action),
            format!("<t:{}:R>{}", log.created_at.timestamp(), extra),
            false,
        );
    }

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
            category: "logs",
            params: "[page]",
            summary: "Affiche les logs d'audit du serveur",
            description: "Affiche les derniers logs d'audit du serveur avec pagination. Les logs incluent tous les événements (modération, messages, rôles, salons, etc.)",
            examples: &["+viewlogs", "+viewlogs 2"],
            default_aliases: &["vlogs", "audit"],
            allow_in_dm: false,
            default_permission: 0,
        }
    }
}
