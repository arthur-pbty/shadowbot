use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::automod_service::pool;
use crate::commands::common::send_embed;
use crate::db;

pub async fn handle_clear_limit(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(raw_value) = args.first() else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Clear Limit")
                .description("Usage: +clearlimit <nombre>")
                .color(0xED4245),
        )
        .await;
        return;
    };

    let Ok(value) = raw_value.parse::<i32>() else {
        return;
    };

    let clamped = value.clamp(1, 1_000);
    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    if db::set_clear_limit(&pool, bot_id, guild_id.get() as i64, clamped)
        .await
        .is_err()
    {
        return;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Clear Limit")
            .description(format!(
                "Limite de suppression definie a **{}** message(s) par commande clear.",
                clamped
            ))
            .color(0x57F287),
    )
    .await;
}

pub struct ClearLimitCommand;
pub static COMMAND_DESCRIPTOR: ClearLimitCommand = ClearLimitCommand;

impl crate::commands::command_contract::CommandSpec for ClearLimitCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "clearlimit",
            category: "mod",
            params: "<nombre>",
            description: "Definit la limite max de messages supprimables avec +clear.",
            examples: &["+clearlimit 100", "+help clearlimit"],
            default_aliases: &["climit"],
            allow_in_dm: false,
            default_permission: 7,
        }
    }
}
