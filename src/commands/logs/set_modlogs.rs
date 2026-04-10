use std::collections::BTreeSet;

use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::logs_command_helpers::pool;

pub async fn handle_set_modlogs(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let row = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT modlog_events
        FROM bot_log_settings
        WHERE bot_id = $1 AND guild_id = $2
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .fetch_optional(&pool)
    .await
    .ok()
    .flatten();

    let mut events = row
        .map(|(s,)| {
            s.split(',')
                .map(|v| v.trim().to_lowercase())
                .filter(|v| !v.is_empty())
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_else(|| {
            [
                "warn",
                "mute",
                "tempmute",
                "unmute",
                "cmute",
                "tempcmute",
                "uncmute",
                "kick",
                "ban",
                "tempban",
                "unban",
                "lock",
                "unlock",
                "hide",
                "unhide",
                "addrole",
                "delrole",
                "derank",
                "clear",
                "sanctions",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
        });

    if args.len() >= 2 {
        let event = args[0].to_lowercase();
        let state = args[1].to_lowercase();
        if state == "on" {
            events.insert(event);
        } else if state == "off" {
            events.remove(&event);
        }

        let serialized = events.iter().cloned().collect::<Vec<_>>().join(",");
        let _ = sqlx::query(
            r#"
            INSERT INTO bot_log_settings (bot_id, guild_id, modlog_events)
            VALUES ($1, $2, $3)
            ON CONFLICT (bot_id, guild_id)
            DO UPDATE SET modlog_events = EXCLUDED.modlog_events, updated_at = NOW();
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(serialized)
        .execute(&pool)
        .await;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Set ModLogs")
            .description(format!(
                "Evenements actifs:\n{}\n\nUsage: +set modlogs <event> <on/off>",
                events.iter().cloned().collect::<Vec<_>>().join(", ")
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct SetModlogsCommand;
pub static COMMAND_DESCRIPTOR: SetModlogsCommand = SetModlogsCommand;

impl crate::commands::command_contract::CommandSpec for SetModlogsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "set_modlogs",
            category: "logs",
            params: "[event on/off]",
            summary: "Parametre les evenements de modlogs",
            description: "Affiche ou modifie les evenements qui apparaissent dans les logs de moderation.",
            examples: &["+set modlogs", "+set modlogs warn off"],
            default_aliases: &["smodlog"],
            default_permission: 8,
        }
    }
}
