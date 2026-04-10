use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::parse_user_id;
use crate::commands::common::{send_embed, theme_color};
use crate::db::DbPoolKey;

pub async fn handle_del_sanction(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.len() < 3 {
        return;
    }

    let Some(target) = parse_user_id(args[1]) else {
        return;
    };
    let Ok(index) = args[2].parse::<usize>() else {
        return;
    };
    if index == 0 {
        return;
    }

    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };
    let Some(pool) = pool else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let rows = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT id
        FROM bot_sanctions
        WHERE bot_id = $1 AND guild_id = $2 AND user_id = $3
        ORDER BY created_at DESC;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(target.get() as i64)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let Some((sanction_id,)) = rows.get(index - 1).copied() else {
        return;
    };

    let _ = sqlx::query(
        r#"
        DELETE FROM bot_sanctions
        WHERE id = $1 AND bot_id = $2 AND guild_id = $3;
        "#,
    )
    .bind(sanction_id)
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .execute(&pool)
    .await;

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Sanctions")
            .description(format!(
                "Sanction #{} supprimée pour <@{}>.",
                sanction_id,
                target.get()
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct DelSanctionCommand;
pub static COMMAND_DESCRIPTOR: DelSanctionCommand = DelSanctionCommand;

impl crate::commands::command_contract::CommandSpec for DelSanctionCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "del_sanction",
            category: "moderation",
            params: "<@membre/ID> <nombre>",
            summary: "Supprime une sanction d un membre",
            description: "Supprime une sanction specifique dans l historique d un membre.",
            examples: &["+del sanction @User 1"],
            default_aliases: &["delsanction"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
