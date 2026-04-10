use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::logs_command_helpers::pool;

pub async fn handle_set_boostembed(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.len() < 2 {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Set BoostEmbed")
                .description("Usage: +set boostembed <title|description|color> <valeur>")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let field = args[0].to_lowercase();
    let value = args[1..].join(" ");
    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id;

    let _ = sqlx::query(
        r#"
        INSERT INTO bot_boost_embed (bot_id, guild_id, enabled, title, description, color)
        VALUES ($1, $2, TRUE, NULL, NULL, NULL)
        ON CONFLICT (bot_id, guild_id)
        DO NOTHING;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .execute(&pool)
    .await;

    match field.as_str() {
        "title" => {
            let _ = sqlx::query(
                "UPDATE bot_boost_embed SET title = $3, updated_at = NOW() WHERE bot_id = $1 AND guild_id = $2",
            )
            .bind(bot_id.get() as i64)
            .bind(guild_id.get() as i64)
            .bind(value)
            .execute(&pool)
            .await;
        }
        "description" => {
            let _ = sqlx::query(
                "UPDATE bot_boost_embed SET description = $3, updated_at = NOW() WHERE bot_id = $1 AND guild_id = $2",
            )
            .bind(bot_id.get() as i64)
            .bind(guild_id.get() as i64)
            .bind(value)
            .execute(&pool)
            .await;
        }
        "color" => {
            let normalized = value
                .trim()
                .trim_start_matches('#')
                .trim_start_matches("0x");
            if let Ok(color) = u32::from_str_radix(normalized, 16) {
                let _ = sqlx::query(
                    "UPDATE bot_boost_embed SET color = $3, updated_at = NOW() WHERE bot_id = $1 AND guild_id = $2",
                )
                .bind(bot_id.get() as i64)
                .bind(guild_id.get() as i64)
                .bind(color as i32)
                .execute(&pool)
                .await;
            }
        }
        _ => {}
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Set BoostEmbed")
            .description("Configuration mise a jour.")
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct SetBoostembedCommand;
pub static COMMAND_DESCRIPTOR: SetBoostembedCommand = SetBoostembedCommand;

impl crate::commands::command_contract::CommandSpec for SetBoostembedCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "set_boostembed",
            category: "admin",
            params: "<title|description|color> <valeur>",
            summary: "Parametre l embed de boost",
            description: "Configure le titre, la description et la couleur de l embed boost.",
            examples: &[
                "+set boostembed title Merci",
                "+set boostembed color #FF66CC",
            ],
            default_aliases: &["sboostembed"],
            default_permission: 8,
        }
    }
}
