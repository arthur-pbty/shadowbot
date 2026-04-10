use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;
use crate::commands::common::{send_embed, theme_color};
use crate::db::DbPoolKey;

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

pub async fn handle_autobackup(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(kind_raw) = args.first() else {
        return;
    };
    let Some(days_raw) = args.get(1) else {
        return;
    };

    let Some(kind) = advanced_tools::backup_kind_from_input(kind_raw) else {
        return;
    };

    let Ok(days) = days_raw.parse::<i32>() else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id;
    let _ = sqlx::query(
        r#"
        INSERT INTO bot_autobackups (bot_id, guild_id, kind, interval_days, next_run_at)
        VALUES ($1, $2, $3, $4, NOW() + make_interval(days => $4))
        ON CONFLICT (bot_id, guild_id, kind)
        DO UPDATE SET interval_days = EXCLUDED.interval_days,
                      next_run_at = NOW() + make_interval(days => EXCLUDED.interval_days);
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(kind)
    .bind(days.max(1))
    .execute(&pool)
    .await;

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("AutoBackup")
            .description(format!(
                "Auto-backup `{}` configuree toutes les {} jours.",
                kind,
                days.max(1)
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct AutoBackupCommand;
pub static COMMAND_DESCRIPTOR: AutoBackupCommand = AutoBackupCommand;

impl crate::commands::command_contract::CommandSpec for AutoBackupCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "autobackup",
            category: "outils",
            params: "<serveur/emoji> <jours>",
            summary: "Configure les backups automatiques",
            description: "Definit l'intervalle en jours des backups automatiques.",
            examples: &["+autobackup serveur 3", "+autobackup emoji 7"],
            default_aliases: &["abkp"],
            default_permission: 8,
        }
    }
}
