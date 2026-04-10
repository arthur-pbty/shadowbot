use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::parse_user_id;
use crate::commands::common::{send_embed, theme_color};
use crate::db::DbPoolKey;

pub async fn handle_sanctions(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    let Some(target_raw) = args.first() else {
        let _ = send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Sanctions")
                .description("Usage: +sanctions <membre>")
                .color(0xED4245),
        )
        .await;
        return;
    };
    let Some(target) = parse_user_id(target_raw) else {
        return;
    };

    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };
    let Some(pool) = pool else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let rows = sqlx::query_as::<
        _,
        (
            i64,
            String,
            String,
            chrono::DateTime<Utc>,
            Option<chrono::DateTime<Utc>>,
            bool,
        ),
    >(
        r#"
        SELECT id, kind, reason, created_at, expires_at, active
        FROM bot_sanctions
        WHERE bot_id = $1 AND guild_id = $2 AND user_id = $3
        ORDER BY created_at DESC
        LIMIT 30;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(target.get() as i64)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let desc = if rows.is_empty() {
        "Aucune sanction.".to_string()
    } else {
        rows.into_iter()
            .map(|(id, kind, reason, created_at, expires_at, active)| {
                let until = expires_at
                    .map(|d| format!(" · jusqu'à <t:{}:R>", d.timestamp()))
                    .unwrap_or_default();
                format!(
                    "`#{}` `{}` {} · <t:{}:R>{} · {}",
                    id,
                    kind,
                    if active { "(active)" } else { "(inactive)" },
                    created_at.timestamp(),
                    until,
                    reason
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title(format!("Sanctions de <@{}>", target.get()))
            .description(desc)
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct SanctionsCommand;
pub static COMMAND_DESCRIPTOR: SanctionsCommand = SanctionsCommand;

impl crate::commands::command_contract::CommandSpec for SanctionsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "sanctions",
            category: "admin",
            params: "<@membre/ID>",
            summary: "Affiche les sanctions d un membre",
            description: "Liste l historique des sanctions d un membre.",
            examples: &["+sanctions @User"],
            default_aliases: &["sanct"],
            default_permission: 8,
        }
    }
}
