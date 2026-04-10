use serenity::model::prelude::*;
use serenity::prelude::*;

use serenity::builder::CreateEmbed;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::logs_command_helpers::{parse_target_channel, pool};

pub async fn handle_leave_settings(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    if args.is_empty() {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("leave settings")
                .description("Usage: +leavesettings [on/off] [salon] [message...]")
                .color(0xED4245),
        )
        .await;
        return;
    }

    if args.len() == 1 {
        let row = sqlx::query_as::<_, (bool, Option<i64>, Option<String>)>(
            r#"
            SELECT enabled, channel_id, custom_message
            FROM bot_join_leave_settings
            WHERE bot_id = $1 AND guild_id = $2 AND kind = $3
            LIMIT 1;
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind("leave")
        .fetch_optional(&pool)
        .await
        .ok()
        .flatten();

        let desc = if let Some((enabled, channel_id, custom_message)) = row {
            format!(
                "Etat: {}\nSalon: {}\nMessage: {}",
                if enabled { "on" } else { "off" },
                channel_id
                    .map(|id| format!("<#{}>", id))
                    .unwrap_or_else(|| "non defini".to_string()),
                custom_message.unwrap_or_else(|| "(defaut)".to_string())
            )
        } else {
            "Aucun reglage configure.".to_string()
        };

        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("leave settings")
                .description(desc)
                .color(theme_color(ctx).await),
        )
        .await;
        return;
    }

    let action = args[0].to_lowercase();
    if action != "on" && action != "off" {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("leave settings")
                .description("Usage: +leavesettings [on/off] [salon] [message...]")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let enabled = action == "on";
    let channel = if enabled {
        parse_target_channel(msg, args, 1)
    } else {
        None
    };
    let message_start = if enabled { 2 } else { 1 };
    let custom_message = if args.len() > message_start {
        Some(args[message_start..].join(" "))
    } else {
        None
    };

    let _ = sqlx::query(
        r#"
        INSERT INTO bot_join_leave_settings (bot_id, guild_id, kind, enabled, channel_id, custom_message)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (bot_id, guild_id, kind)
        DO UPDATE SET enabled = EXCLUDED.enabled, channel_id = EXCLUDED.channel_id,
                      custom_message = EXCLUDED.custom_message, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind("leave")
    .bind(enabled)
    .bind(channel.map(|c| c.get() as i64))
    .bind(custom_message)
    .execute(&pool)
    .await;

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("leave settings")
            .description(format!(
                "{} {}",
                if enabled { "Active" } else { "Desactive" },
                channel
                    .map(|c| format!("dans <#{}>", c.get()))
                    .unwrap_or_default()
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct LeaveSettingsCommand;
pub static COMMAND_DESCRIPTOR: LeaveSettingsCommand = LeaveSettingsCommand;

impl crate::commands::command_contract::CommandSpec for LeaveSettingsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "leavesettings",
            category: "config",
            params: "[on/off] [salon] [message]",
            description: "Configure les actions a executer quand un membre quitte le serveur.",
            examples: &[
                "+leavesettings",
                "+leavesettings on #logs {user} a quitte",
            ],
            default_aliases: &["lset"],
            allow_in_dm: false,
            default_permission: 5,
        }
    }
}
