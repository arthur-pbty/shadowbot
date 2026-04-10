use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::parse_user_id;
use crate::commands::common::{send_embed, theme_color};
use crate::db::DbPoolKey;

pub async fn handle_clear_sanctions(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.len() < 2 {
        return;
    }

    let Some(target) = parse_user_id(args[1]) else {
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
        WHERE bot_id = $1 AND guild_id = $2 AND user_id = $3;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(target.get() as i64)
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
                "{} sanction(s) supprimée(s) pour <@{}>.",
                removed,
                target.get()
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct ClearSanctionsCommand;
pub static COMMAND_DESCRIPTOR: ClearSanctionsCommand = ClearSanctionsCommand;

impl crate::commands::command_contract::CommandSpec for ClearSanctionsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "clear_sanctions",
            category: "moderation",
            params: "<@membre/ID>",
            summary: "Supprime toutes les sanctions d un membre",
            description: "Efface completement les sanctions d un membre cible.",
            examples: &["+clear sanctions @User"],
            default_aliases: &["csanctions"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
