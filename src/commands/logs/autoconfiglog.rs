use std::collections::HashMap;

use serenity::builder::{
    CreateActionRow, CreateButton, CreateChannel, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage,
};
use serenity::model::application::{ButtonStyle, ComponentInteraction};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::logs_command_helpers::{pool, set_log_channel};

const LOG_TYPES: &[&str] = &["moderation", "message", "voice", "boost", "role", "raid", "channel"];
const LOG_CATEGORY_NAME: &str = "📁 ➜ Espace Logs";
const LOG_CHANNEL_PREFIX: &str = "📁・";
const AUTOCONFIGLOG_COMPONENT_PREFIX: &str = "autoconfiglog";

struct AutoconfigResult {
    configured: Vec<String>,
    failed: Vec<String>,
}

fn log_channel_name(log_type: &str) -> String {
    format!("{}{}-logs", LOG_CHANNEL_PREFIX, log_type)
}

async fn get_or_create_logs_category(ctx: &Context, guild_id: GuildId) -> Option<ChannelId> {
    let channels = guild_id.channels(&ctx.http).await.ok()?;

    if let Some(category) = channels
        .values()
        .find(|ch| ch.kind == ChannelType::Category && ch.name == LOG_CATEGORY_NAME)
    {
        return Some(category.id);
    }

    guild_id
        .create_channel(
            &ctx.http,
            CreateChannel::new(LOG_CATEGORY_NAME).kind(ChannelType::Category),
        )
        .await
        .ok()
        .map(|category| category.id)
}

async fn create_and_configure_log_channels(
    ctx: &Context,
    guild_id: GuildId,
    force_recreate: bool,
) -> AutoconfigResult {
    let Some(category_id) = get_or_create_logs_category(ctx, guild_id).await else {
        return AutoconfigResult {
            configured: Vec::new(),
            failed: vec!["Impossible de creer ou trouver la categorie des logs.".to_string()],
        };
    };

    let existing_configured = if force_recreate {
        HashMap::new()
    } else {
        let rows = if let Some(pool) = pool(ctx).await {
            let bot_id = ctx.cache.current_user().id;
            sqlx::query_as::<_, (String, Option<i64>, bool)>(
                r#"
                SELECT log_type, channel_id, enabled
                FROM bot_log_channels
                WHERE bot_id = $1 AND guild_id = $2;
                "#,
            )
            .bind(bot_id.get() as i64)
            .bind(guild_id.get() as i64)
            .fetch_all(&pool)
            .await
            .unwrap_or_default()
        } else {
            Vec::new()
        };

        rows.into_iter()
            .map(|(log_type, channel_id, enabled)| (log_type, (channel_id, enabled)))
            .collect::<HashMap<String, (Option<i64>, bool)>>()
    };

    let channels = guild_id.channels(&ctx.http).await.unwrap_or_default();

    let mut configured = Vec::new();
    let mut failed = Vec::new();

    for log_type in LOG_TYPES {
        if !force_recreate {
            if let Some((channel_id, enabled)) = existing_configured.get(*log_type) {
                if *enabled {
                    if let Some(channel_id) = *channel_id {
                        if let Ok(channel_u64) = u64::try_from(channel_id) {
                            let existing_channel_id = ChannelId::new(channel_u64);
                            if channels.contains_key(&existing_channel_id) {
                                configured.push(format!(
                                    "{} -> deja configure <#{}>",
                                    log_type,
                                    existing_channel_id.get()
                                ));
                                continue;
                            }
                        }
                    }
                }
            }
        }

        let name = log_channel_name(log_type);
        let builder = CreateChannel::new(name)
            .kind(ChannelType::Text)
            .category(category_id);

        match guild_id.create_channel(&ctx.http, builder).await {
            Ok(channel) => {
                set_log_channel(ctx, guild_id, log_type, Some(channel.id), true).await;
                configured.push(format!("{} -> <#{}>", log_type, channel.id.get()));
            }
            Err(_) => failed.push(format!("{} -> echec creation", log_type)),
        }
    }

    AutoconfigResult { configured, failed }
}

async fn all_log_types_configured_and_existing(ctx: &Context, guild_id: GuildId) -> bool {
    let Some(pool) = pool(ctx).await else {
        return false;
    };

    let bot_id = ctx.cache.current_user().id;

    let rows = sqlx::query_as::<_, (String, Option<i64>, bool)>(
        r#"
        SELECT log_type, channel_id, enabled
        FROM bot_log_channels
        WHERE bot_id = $1 AND guild_id = $2;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let configured: HashMap<String, (Option<i64>, bool)> = rows
        .into_iter()
        .map(|(log_type, channel_id, enabled)| (log_type, (channel_id, enabled)))
        .collect();

    let Ok(channels) = guild_id.channels(&ctx.http).await else {
        return false;
    };

    LOG_TYPES.iter().all(|log_type| {
        let Some((channel_id, enabled)) = configured.get(*log_type) else {
            return false;
        };

        if !*enabled {
            return false;
        }

        let Some(channel_id) = *channel_id else {
            return false;
        };

        let Ok(channel_u64) = u64::try_from(channel_id) else {
            return false;
        };

        channels.contains_key(&ChannelId::new(channel_u64))
    })
}

async fn build_result_embed(ctx: &Context, title: &str, result: &AutoconfigResult) -> CreateEmbed {
    let mut lines = Vec::new();

    if result.configured.is_empty() {
        lines.push("Aucun salon configure.".to_string());
    } else {
        lines.extend(result.configured.iter().cloned());
    }

    if !result.failed.is_empty() {
        lines.push(String::new());
        lines.push("Erreurs:".to_string());
        for err in &result.failed {
            lines.push(format!("- {}", err));
        }
    }

    CreateEmbed::new()
        .title(title)
        .description(lines.join("\n"))
        .color(theme_color(ctx).await)
}

fn parse_component_custom_id(custom_id: &str) -> Option<(&str, u64)> {
    let mut parts = custom_id.split(':');
    let prefix = parts.next()?;
    let action = parts.next()?;
    let owner_id = parts.next()?.parse::<u64>().ok()?;

    if prefix != AUTOCONFIGLOG_COMPONENT_PREFIX || parts.next().is_some() {
        return None;
    }

    Some((action, owner_id))
}

pub async fn handle_autoconfiglog(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    if all_log_types_configured_and_existing(ctx, guild_id).await {
        let embed = CreateEmbed::new()
            .title("AutoConfigLog")
            .description(
                "Tous les types de logs sont deja configures et les salons existent.\nConfirmer va recreer les salons de logs et remplacer la configuration actuelle.",
            )
            .color(theme_color(ctx).await);

        let components = vec![CreateActionRow::Buttons(vec![
            CreateButton::new(format!(
                "{}:confirm:{}",
                AUTOCONFIGLOG_COMPONENT_PREFIX,
                msg.author.id.get()
            ))
            .label("Confirmer")
            .style(ButtonStyle::Danger),
            CreateButton::new(format!(
                "{}:cancel:{}",
                AUTOCONFIGLOG_COMPONENT_PREFIX,
                msg.author.id.get()
            ))
            .label("Annuler")
            .style(ButtonStyle::Secondary),
        ])];

        let _ = msg
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().embed(embed).components(components),
            )
            .await;
        return;
    }

    let result = create_and_configure_log_channels(ctx, guild_id, false).await;

    send_embed(
        ctx,
        msg,
        build_result_embed(ctx, "AutoConfigLog", &result).await,
    )
    .await;
}

pub async fn handle_component_interaction(ctx: &Context, component: &ComponentInteraction) -> bool {
    let Some((action, owner_id)) = parse_component_custom_id(&component.data.custom_id) else {
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

    match action {
        "cancel" => {
            let _ = component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .embed(
                                CreateEmbed::new()
                                    .title("AutoConfigLog")
                                    .description("Action annulee. Aucun salon n'a ete cree.")
                                    .color(theme_color(ctx).await),
                            )
                            .components(vec![]),
                    ),
                )
                .await;

            true
        }
        "confirm" => {
            let Some(guild_id) = component.guild_id else {
                let _ = component
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("Cette action doit etre utilisee sur un serveur.")
                                .ephemeral(true),
                        ),
                    )
                    .await;
                return true;
            };

            let result = create_and_configure_log_channels(ctx, guild_id, true).await;
            let embed = build_result_embed(ctx, "AutoConfigLog execute", &result).await;

            let _ = component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .embed(embed)
                            .components(vec![]),
                    ),
                )
                .await;

            true
        }
        _ => false,
    }
}

pub struct AutoconfiglogCommand;
pub static COMMAND_DESCRIPTOR: AutoconfiglogCommand = AutoconfiglogCommand;

impl crate::commands::command_contract::CommandSpec for AutoconfiglogCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "autoconfiglog",
            category: "logs",
            params: "aucun",
            description: "Cree automatiquement les salons de logs et les configure.",
            examples: &["+autoconfiglog"],
            default_aliases: &["acl"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
