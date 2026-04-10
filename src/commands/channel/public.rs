use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::automod_service::{parse_on_off, pool};
use crate::commands::common::{parse_channel_id, send_embed};
use crate::db;

pub async fn handle_public(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(first) = args.first() else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Public")
                .description("Usage: +public <on/off> | +public <allow/deny/reset> [#salon]")
                .color(0xED4245),
        )
        .await;
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let guild_id_raw = guild_id.get() as i64;

    if let Some(enabled) = parse_on_off(first) {
        let _ = db::set_public_commands_enabled(&pool, bot_id, guild_id_raw, enabled).await;
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Public")
                .description(format!(
                    "Commandes publiques sur le serveur: **{}**",
                    if enabled { "ON" } else { "OFF" }
                ))
                .color(0x57F287),
        )
        .await;
        return;
    }

    let channel_id = args
        .get(1)
        .and_then(|raw| parse_channel_id(raw))
        .unwrap_or(msg.channel_id);

    let description = if first.eq_ignore_ascii_case("allow") {
        let _ = db::set_moderation_channel_override(
            &pool,
            bot_id,
            guild_id_raw,
            channel_id.get() as i64,
            "public",
            "allow",
        )
        .await;
        format!("Commandes publiques forcees dans <#{}>.", channel_id.get())
    } else if first.eq_ignore_ascii_case("deny") {
        let _ = db::set_moderation_channel_override(
            &pool,
            bot_id,
            guild_id_raw,
            channel_id.get() as i64,
            "public",
            "deny",
        )
        .await;
        format!(
            "Commandes publiques desactivees dans <#{}>.",
            channel_id.get()
        )
    } else if first.eq_ignore_ascii_case("reset") {
        let _ = db::remove_moderation_channel_override(
            &pool,
            bot_id,
            guild_id_raw,
            channel_id.get() as i64,
            "public",
        )
        .await;
        format!("Override public supprime dans <#{}>.", channel_id.get())
    } else {
        return;
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Public")
            .description(description)
            .color(0x57F287),
    )
    .await;
}

pub struct PublicCommand;
pub static COMMAND_DESCRIPTOR: PublicCommand = PublicCommand;

impl crate::commands::command_contract::CommandSpec for PublicCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "public",
            category: "channel",
            params: "<on/off> | <allow/deny/reset> [#salon]",
            description: "Active/desactive les commandes publiques globalement ou par salon.",
            examples: &[
                "+public on",
                "+public deny #annonces",
                "+public reset #annonces",
            ],
            default_aliases: &["pubc"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
