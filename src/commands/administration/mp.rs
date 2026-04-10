use serenity::builder::{
    CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{
    add_list_fields, discord_ts, send_embed, theme_color, truncate_text,
};
use crate::db::{
    DbPoolKey, count_sent_mp_messages, get_mp_enabled, get_sent_mp_message, list_sent_mp_messages,
    log_sent_mp_message, mark_sent_mp_deleted, set_mp_enabled,
};

pub async fn handle_mp(ctx: &Context, msg: &Message, args: &[&str]) {
    if args
        .first()
        .map(|value| value.eq_ignore_ascii_case("settings"))
        .unwrap_or(false)
    {
        let bot_id = ctx.cache.current_user().id;
        let Some(pool) = pool(ctx).await else {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description("DB indisponible.")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        };

        if args.len() == 1 {
            let enabled = get_mp_enabled(&pool, bot_id)
                .await
                .ok()
                .flatten()
                .unwrap_or(true);
            let embed = CreateEmbed::new()
                .title("MP settings")
                .description(format!(
                    "Envoi de MP: `{}`\nUtilise `+mp settings on/off`.",
                    if enabled { "on" } else { "off" }
                ))
                .color(0x5865F2);
            send_embed(ctx, msg, embed).await;
            return;
        }

        let enabled = match args[1].to_lowercase().as_str() {
            "on" | "true" | "yes" => true,
            "off" | "false" | "no" => false,
            _ => {
                let embed = CreateEmbed::new()
                    .title("Erreur")
                    .description("Usage: `+mp settings <on/off>`")
                    .color(0xED4245);
                send_embed(ctx, msg, embed).await;
                return;
            }
        };

        let _ = set_mp_enabled(&pool, bot_id, enabled).await;
        let embed = CreateEmbed::new()
            .title("MP settings mis à jour")
            .description(format!(
                "Envoi de MP: `{}`",
                if enabled { "on" } else { "off" }
            ))
            .color(0x57F287);
        send_embed(ctx, msg, embed).await;
        return;
    }

    if args
        .first()
        .map(|value| value.eq_ignore_ascii_case("sent"))
        .unwrap_or(false)
    {
        let page = args
            .get(1)
            .and_then(|value| value.parse::<i64>().ok())
            .filter(|value| *value >= 1)
            .unwrap_or(1);
        let _ = send_mp_sent_page(ctx, msg, page).await;
        return;
    }

    if args
        .first()
        .map(|value| value.eq_ignore_ascii_case("delete") || value.eq_ignore_ascii_case("del"))
        .unwrap_or(false)
    {
        let Some(entry_id_raw) = args.get(1) else {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description("Usage: `+mp delete <id>`")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        };

        let Ok(entry_id) = entry_id_raw.parse::<i64>() else {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description("ID invalide.")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        };

        let bot_id = ctx.cache.current_user().id;
        let Some(pool) = pool(ctx).await else {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description("DB indisponible.")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        };

        let Some(entry) = get_sent_mp_message(&pool, bot_id, entry_id)
            .await
            .ok()
            .flatten()
        else {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description("Message MP introuvable.")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        };

        let delete_result = ChannelId::new(entry.dm_channel_id as u64)
            .delete_message(&ctx.http, MessageId::new(entry.message_id as u64))
            .await;

        let _ = mark_sent_mp_deleted(&pool, bot_id, entry_id).await;
        if delete_result.is_err() {
            let embed = CreateEmbed::new()
                .title("MP déjà supprimé ou inaccessible")
                .description(format!(
                    "Entrée `#{}` marquée supprimée en base (Discord a refusé la suppression).",
                    entry.entry_id
                ))
                .color(0xFEE75C);
            send_embed(ctx, msg, embed).await;
        } else {
            let embed = CreateEmbed::new()
                .title("MP supprimé")
                .description(format!("Entrée `#{}` supprimée.", entry.entry_id))
                .color(0x57F287);
            send_embed(ctx, msg, embed).await;
        }
        return;
    }

    if args.len() < 2 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+mp settings` ou `+mp <membre> <message>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let bot_id = ctx.cache.current_user().id;
    let Some(db_pool) = pool(ctx).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let enabled = get_mp_enabled(&db_pool, bot_id)
        .await
        .ok()
        .flatten()
        .unwrap_or(true);
    if !enabled {
        let embed = CreateEmbed::new()
            .title("MP désactivés")
            .description("Réactive-les avec `+mp settings on`.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let Some(user_id) = parse_user_id(args[0]) else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Membre invalide.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let content = args[1..].join(" ");
    let message = format!("{}\n\n- envoyé par {}", content, msg.author.tag());

    let result = user_id.create_dm_channel(&ctx.http).await;
    let Ok(channel) = result else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Impossible d'ouvrir le MP.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let Ok(sent_message) = channel.say(&ctx.http, message.clone()).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Impossible d'envoyer le MP.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if let Some(pool) = pool(ctx).await {
        let _ = log_sent_mp_message(
            &pool,
            bot_id,
            msg.author.id,
            user_id,
            channel.id,
            sent_message.id,
            &message,
        )
        .await;
    }

    let embed = CreateEmbed::new()
        .title("Message privé envoyé")
        .description(format!("À <@{}>.", user_id.get()))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub async fn handle_mp_component(ctx: &Context, component: &ComponentInteraction) -> bool {
    let Some((owner_id, page)) = parse_mp_sent_custom_id(&component.data.custom_id) else {
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

    let bot_id = ctx.cache.current_user().id;
    let Some(pool) = pool(ctx).await else {
        return true;
    };

    let total = count_sent_mp_messages(&pool, bot_id)
        .await
        .unwrap_or(0)
        .max(0);
    let total_pages = ((total + MP_SENT_PAGE_SIZE - 1) / MP_SENT_PAGE_SIZE).max(1);
    let safe_page = page.clamp(1, total_pages);
    let offset = (safe_page - 1) * MP_SENT_PAGE_SIZE;
    let items = list_sent_mp_messages(&pool, bot_id, MP_SENT_PAGE_SIZE, offset)
        .await
        .unwrap_or_default();

    let lines = items
        .iter()
        .map(|entry| {
            let status = if entry.deleted_at.is_some() {
                "supprimé"
            } else {
                "actif"
            };
            let sent_at = discord_ts(
                Timestamp::from_unix_timestamp(entry.sent_at.timestamp())
                    .unwrap_or_else(|_| Timestamp::now()),
                "F",
            );
            format!(
                "`#{}` · de <@{}> vers <@{}> · msg `{}` · {} · {} · {}",
                entry.entry_id,
                entry.sender_id,
                entry.recipient_id,
                entry.message_id,
                status,
                sent_at,
                truncate_text(&entry.content, 80)
            )
        })
        .collect::<Vec<_>>();

    let prev_page = if safe_page > 1 { safe_page - 1 } else { 1 };
    let next_page = if safe_page < total_pages {
        safe_page + 1
    } else {
        total_pages
    };
    let mut embed = CreateEmbed::new()
        .title("MP envoyés")
        .description(format!(
            "{} message(s) · Page {}/{}",
            total, safe_page, total_pages
        ))
        .color(theme_color(ctx).await);
    embed = add_list_fields(embed, &lines, "Messages");

    let components = vec![CreateActionRow::Buttons(vec![
        CreateButton::new(format!("mpsent:{}:{}", component.user.id.get(), prev_page))
            .label("◀ Précédent")
            .style(ButtonStyle::Primary)
            .disabled(safe_page <= 1),
        CreateButton::new(format!("mpsent:{}:{}", component.user.id.get(), next_page))
            .label("Suivant ▶")
            .style(ButtonStyle::Primary)
            .disabled(safe_page >= total_pages),
    ])];

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

const MP_SENT_PAGE_SIZE: i64 = 10;

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

async fn send_mp_sent_page(ctx: &Context, msg: &Message, page: i64) -> Result<(), ()> {
    let bot_id = ctx.cache.current_user().id;
    let Some(pool) = pool(ctx).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return Err(());
    };

    let total = count_sent_mp_messages(&pool, bot_id)
        .await
        .unwrap_or(0)
        .max(0);
    let total_pages = ((total + MP_SENT_PAGE_SIZE - 1) / MP_SENT_PAGE_SIZE).max(1);
    let safe_page = page.clamp(1, total_pages) as i64;
    let offset = (safe_page - 1) * MP_SENT_PAGE_SIZE;
    let items = list_sent_mp_messages(&pool, bot_id, MP_SENT_PAGE_SIZE, offset)
        .await
        .unwrap_or_default();

    let lines = items
        .iter()
        .map(|entry| {
            let status = if entry.deleted_at.is_some() {
                "supprimé"
            } else {
                "actif"
            };
            let sent_at = discord_ts(
                Timestamp::from_unix_timestamp(entry.sent_at.timestamp())
                    .unwrap_or_else(|_| Timestamp::now()),
                "F",
            );
            format!(
                "`#{}` · de <@{}> vers <@{}> · msg `{}` · {} · {} · {}",
                entry.entry_id,
                entry.sender_id,
                entry.recipient_id,
                entry.message_id,
                status,
                sent_at,
                truncate_text(&entry.content, 80)
            )
        })
        .collect::<Vec<_>>();

    let prev_page = if safe_page > 1 { safe_page - 1 } else { 1 };
    let next_page = if safe_page < total_pages {
        safe_page + 1
    } else {
        total_pages
    };
    let mut embed = CreateEmbed::new()
        .title("MP envoyés")
        .description(format!(
            "{} message(s) · Page {}/{}",
            total, safe_page, total_pages
        ))
        .color(theme_color(ctx).await);

    embed = add_list_fields(embed, &lines, "Messages");

    let components = vec![CreateActionRow::Buttons(vec![
        CreateButton::new(format!("mpsent:{}:{}", msg.author.id.get(), prev_page))
            .label("◀ Précédent")
            .style(ButtonStyle::Primary)
            .disabled(safe_page <= 1),
        CreateButton::new(format!("mpsent:{}:{}", msg.author.id.get(), next_page))
            .label("Suivant ▶")
            .style(ButtonStyle::Primary)
            .disabled(safe_page >= total_pages),
    ])];

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new().embed(embed).components(components),
        )
        .await;

    Ok(())
}

fn parse_mp_sent_custom_id(custom_id: &str) -> Option<(u64, i64)> {
    let parts = custom_id.split(':').collect::<Vec<_>>();
    if parts.len() != 3 || parts.first().copied()? != "mpsent" {
        return None;
    }

    let owner_id = parts.get(1)?.parse::<u64>().ok()?;
    let page = parts.get(2)?.parse::<i64>().ok()?;
    Some((owner_id, page))
}

fn parse_user_id(input: &str) -> Option<UserId> {
    let cleaned = input
        .trim()
        .trim_start_matches('<')
        .trim_end_matches('>')
        .trim_start_matches('@')
        .trim_start_matches('!');

    cleaned.parse::<u64>().ok().map(UserId::new)
}
pub struct MpCommand;
pub static COMMAND_DESCRIPTOR: MpCommand = MpCommand;

impl crate::commands::command_contract::CommandSpec for MpCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "mp",
            category: "administration",
            params: "settings [on|off] | sent [page] | delete <id> | <@membre/ID> <message...>",
            summary: "Gere lenvoi de messages prives",
            description: "Permet de configurer, envoyer, lister et supprimer des messages prives envoyes.",
            examples: &["+mp", "+help mp"],
            default_aliases: &["dmsg"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
