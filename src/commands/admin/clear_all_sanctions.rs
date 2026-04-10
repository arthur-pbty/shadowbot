use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::db::DbPoolKey;

pub async fn handle_clear_all_sanctions(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
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

    let removed = sqlx::query(
        r#"
        DELETE FROM bot_sanctions
        WHERE bot_id = $1 AND guild_id = $2;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .execute(&pool)
    .await
    .ok()
    .map(|r| r.rows_affected())
    .unwrap_or(0);

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Sanctions")
            .description(format!(
                "{} sanction(s) supprimée(s) sur le serveur.",
                removed
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct ClearAllSanctionsCommand;
pub static COMMAND_DESCRIPTOR: ClearAllSanctionsCommand = ClearAllSanctionsCommand;

impl crate::commands::command_contract::CommandSpec for ClearAllSanctionsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "clear_all_sanctions",
            category: "admin",
            params: "aucun",
            summary: "Supprime toutes les sanctions du serveur",
            description: "Efface toutes les sanctions de tous les membres du serveur.",
            examples: &["+clear all sanctions"],
            default_aliases: &["casanctions"],
            default_permission: 8,
        }
    }
}
