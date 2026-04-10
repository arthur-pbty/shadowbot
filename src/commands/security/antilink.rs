use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::automod_service::{parse_on_off, pool};
use crate::commands::common::send_embed;
use crate::db;

pub async fn handle_antilink(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(first) = args.first() else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("AntiLink")
                .description("Usage: +antilink <on/off> | +antilink <invite/all>")
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
        db::set_antilink_settings(
            &pool,
            bot_id,
            guild_id.get() as i64,
            value,
            &current.antilink_mode,
        )
        .await
        .ok()
    } else if first.eq_ignore_ascii_case("invite") || first.eq_ignore_ascii_case("all") {
        db::set_antilink_settings(
            &pool,
            bot_id,
            guild_id.get() as i64,
            current.antilink_enabled,
            &first.to_lowercase(),
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
            .title("AntiLink")
            .description(format!(
                "Etat: **{}**\nMode: **{}**",
                if updated.antilink_enabled {
                    "ON"
                } else {
                    "OFF"
                },
                updated.antilink_mode
            ))
            .color(0x57F287),
    )
    .await;
}

pub struct AntilinkCommand;
pub static COMMAND_DESCRIPTOR: AntilinkCommand = AntilinkCommand;

impl crate::commands::command_contract::CommandSpec for AntilinkCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "antilink",
            category: "security",
            params: "<on/off> | <invite/all>",
            description: "Active ou configure la protection anti liens.",
            examples: &["+antilink on", "+antilink invite", "+help antilink"],
            default_aliases: &["alink"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
