use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::automod_service::{format_duration, parse_on_off, parse_rate_limit, pool};
use crate::commands::common::send_embed;
use crate::db;

pub async fn handle_antispam(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(first) = args.first() else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("AntiSpam")
                .description("Usage: +antispam <on/off> | +antispam <nombre>/<duree>")
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
        db::set_antispam_settings(
            &pool,
            bot_id,
            guild_id.get() as i64,
            value,
            current.antispam_limit,
            current.antispam_window_seconds,
        )
        .await
        .ok()
    } else if let Some((limit, window)) = parse_rate_limit(first) {
        db::set_antispam_settings(
            &pool,
            bot_id,
            guild_id.get() as i64,
            current.antispam_enabled,
            limit,
            window,
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
            .title("AntiSpam")
            .description(format!(
                "Etat: **{}**\nSensibilite: **{}/{}**",
                if updated.antispam_enabled {
                    "ON"
                } else {
                    "OFF"
                },
                updated.antispam_limit,
                format_duration(updated.antispam_window_seconds as i64)
            ))
            .color(0x57F287),
    )
    .await;
}

pub struct AntispamCommand;
pub static COMMAND_DESCRIPTOR: AntispamCommand = AntispamCommand;

impl crate::commands::command_contract::CommandSpec for AntispamCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "antispam",
            category: "moderation",
            params: "<on/off> | <nombre>/<duree>",
            description: "Active ou configure la protection antispam du serveur.",
            examples: &["+antispam on", "+antispam 6/5s", "+help antispam"],
            default_aliases: &["aspam"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
