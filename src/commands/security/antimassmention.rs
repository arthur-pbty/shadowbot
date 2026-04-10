use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::automod_service::{parse_on_off, pool};
use crate::commands::common::send_embed;
use crate::db;

pub async fn handle_antimassmention(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(first) = args.first() else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("AntiMassMention")
                .description("Usage: +antimassmention <on/off> | +antimassmention <nombre>")
                .color(0xED4245),
        )
        .await;
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let Ok(current) =
        db::get_or_create_moderation_settings(&pool, bot_id, guild_id.get() as i64).await
    else {
        return;
    };

    let updated = if let Some(value) = parse_on_off(first) {
        db::set_antimassmention_settings(
            &pool,
            bot_id,
            guild_id.get() as i64,
            value,
            current.antimassmention_limit,
        )
        .await
        .ok()
    } else if let Ok(limit) = first.parse::<i32>() {
        db::set_antimassmention_settings(
            &pool,
            bot_id,
            guild_id.get() as i64,
            current.antimassmention_enabled,
            limit.clamp(1, 50),
        )
        .await
        .ok()
    } else {
        None
    };

    let Some(updated) = updated else {
        return;
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("AntiMassMention")
            .description(format!(
                "Etat: **{}**\nSeuil: **{} mention(s)**",
                if updated.antimassmention_enabled {
                    "ON"
                } else {
                    "OFF"
                },
                updated.antimassmention_limit
            ))
            .color(0x57F287),
    )
    .await;
}

pub struct AntimassmentionCommand;
pub static COMMAND_DESCRIPTOR: AntimassmentionCommand = AntimassmentionCommand;

impl crate::commands::command_contract::CommandSpec for AntimassmentionCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "antimassmention",
            category: "security",
            params: "<on/off> | <nombre>",
            description: "Active ou configure la protection anti spam de mentions.",
            examples: &[
                "+antimassmention on",
                "+antimassmention 6",
                "+help antimassmention",
            ],
            default_aliases: &["amm"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
