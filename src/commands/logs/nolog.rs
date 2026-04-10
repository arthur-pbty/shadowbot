use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::logs_command_helpers::{parse_target_channel, pool};

pub async fn handle_nolog(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.is_empty() {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("NoLog")
                .description("Usage: +nolog <add/del> [salon] [message|voice|all]")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let action = args[0].to_lowercase();
    let channel = parse_target_channel(msg, args, 1).unwrap_or(msg.channel_id);
    let scope = args
        .get(2)
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "all".to_string());

    let set_message = scope == "all" || scope == "message";
    let set_voice = scope == "all" || scope == "voice";

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    if action == "add" {
        let _ = sqlx::query(
            r#"
            INSERT INTO bot_nolog_channels (bot_id, guild_id, channel_id, disable_message, disable_voice)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (bot_id, guild_id, channel_id)
            DO UPDATE SET disable_message = bot_nolog_channels.disable_message OR EXCLUDED.disable_message,
                          disable_voice = bot_nolog_channels.disable_voice OR EXCLUDED.disable_voice,
                          updated_at = NOW();
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(channel.get() as i64)
        .bind(set_message)
        .bind(set_voice)
        .execute(&pool)
        .await;
    } else if action == "del" {
        let _ = sqlx::query(
            r#"
            UPDATE bot_nolog_channels
            SET disable_message = CASE WHEN $4 THEN FALSE ELSE disable_message END,
                disable_voice = CASE WHEN $5 THEN FALSE ELSE disable_voice END,
                updated_at = NOW()
            WHERE bot_id = $1 AND guild_id = $2 AND channel_id = $3;
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(channel.get() as i64)
        .bind(set_message)
        .bind(set_voice)
        .execute(&pool)
        .await;

        let _ = sqlx::query(
            r#"
            DELETE FROM bot_nolog_channels
            WHERE bot_id = $1 AND guild_id = $2 AND channel_id = $3
              AND disable_message = FALSE AND disable_voice = FALSE;
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(channel.get() as i64)
        .execute(&pool)
        .await;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("NoLog")
            .description(format!(
                "{} applique sur <#{}> ({})",
                action,
                channel.get(),
                scope
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct NologCommand;
pub static COMMAND_DESCRIPTOR: NologCommand = NologCommand;

impl crate::commands::command_contract::CommandSpec for NologCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "nolog",
            category: "logs",
            params: "<add/del> [salon] [message|voice|all]",
            summary: "Exclut des salons des logs",
            description: "Desactive ou reactive les logs message/voice pour certains salons.",
            examples: &["+nolog add #secret all", "+nolog del #secret message"],
            default_aliases: &["nlg"],
            default_permission: 8,
        }
    }
}
