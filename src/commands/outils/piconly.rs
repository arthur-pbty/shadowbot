use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_channel_id, send_embed};
use crate::db;

fn is_image_filename(filename: &str) -> bool {
    let extension = filename
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();

    matches!(
        extension.as_str(),
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "heic" | "heif"
    )
}

fn has_only_photo_attachments(msg: &Message) -> bool {
    !msg.attachments.is_empty()
        && msg
            .attachments
            .iter()
            .all(|attachment| is_image_filename(&attachment.filename))
}

fn is_piconly_command_message(content: &str, prefix: &str) -> bool {
    if !content.starts_with(prefix) {
        return false;
    }

    let without_prefix = content.trim_start_matches(prefix).trim();
    without_prefix
        .split_whitespace()
        .next()
        .map(|command| command.eq_ignore_ascii_case("piconly"))
        .unwrap_or(false)
}

pub async fn enforce_piconly_message(
    ctx: &Context,
    msg: &Message,
    content: &str,
    prefix: &str,
) -> bool {
    let Some(guild_id) = msg.guild_id else {
        return false;
    };

    let Some(pool) = ({
        let data = ctx.data.read().await;
        data.get::<db::DbPoolKey>().cloned()
    }) else {
        return false;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let is_selfie_channel = db::is_piconly_channel(
        &pool,
        bot_id,
        guild_id.get() as i64,
        msg.channel_id.get() as i64,
    )
    .await
    .unwrap_or(false);

    if !is_selfie_channel || is_piconly_command_message(content, prefix) {
        return false;
    }

    if has_only_photo_attachments(msg) {
        return false;
    }

    let _ = msg.delete(&ctx.http).await;

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Salon selfie")
            .description("Seules les photos sont autorisees dans ce salon.")
            .color(0xED4245)
            .timestamp(Utc::now()),
    )
    .await;

    true
}

pub async fn handle_piconly(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = ({
        let data = ctx.data.read().await;
        data.get::<db::DbPoolKey>().cloned()
    }) else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let guild_id_i64 = guild_id.get() as i64;

    if args.is_empty() {
        let channels = db::get_piconly_channels(&pool, bot_id, guild_id_i64)
            .await
            .unwrap_or_default();

        let description = if channels.is_empty() {
            "Aucun salon selfie configure.".to_string()
        } else {
            channels
                .into_iter()
                .map(|channel| format!("<#{}>", channel.channel_id))
                .collect::<Vec<_>>()
                .join("\n")
        };

        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("PicOnly")
                .description(description)
                .timestamp(Utc::now()),
        )
        .await;
        return;
    }

    let adding = args[0].eq_ignore_ascii_case("add");
    let deleting = args[0].eq_ignore_ascii_case("del")
        || args[0].eq_ignore_ascii_case("remove")
        || args[0].eq_ignore_ascii_case("delete");

    if !adding && !deleting {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("PicOnly")
                .description("Utilisation: +piconly <add/del> [#salon]")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let channel_id = args
        .get(1)
        .and_then(|raw| parse_channel_id(raw))
        .unwrap_or(msg.channel_id);

    let result = if adding {
        db::add_piconly_channel(&pool, bot_id, guild_id_i64, channel_id.get() as i64).await
    } else {
        db::remove_piconly_channel(&pool, bot_id, guild_id_i64, channel_id.get() as i64).await
    };

    if result.is_err() {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("PicOnly")
                .description("Impossible de mettre a jour le salon selfie.")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let embed = if adding {
        CreateEmbed::new()
            .title("Salon selfie ajoute")
            .description(format!("Salon: <#{}>", channel_id.get()))
            .timestamp(Utc::now())
    } else {
        CreateEmbed::new()
            .title("Salon selfie retire")
            .description(format!("Salon: <#{}>", channel_id.get()))
            .timestamp(Utc::now())
    };

    send_embed(ctx, msg, embed).await;
}

pub struct PiconlyCommand;
pub static COMMAND_DESCRIPTOR: PiconlyCommand = PiconlyCommand;

impl crate::commands::command_contract::CommandSpec for PiconlyCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "piconly",
            category: "outils",
            params: "<add/del> [salon]",
            description: "Definit ou supprime un salon selfie, ou les membres ne peuvent envoyer que des photos.",
            examples: &["+piconly", "+piconly add #selfie", "+piconly del #selfie"],
            default_aliases: &["selfieonly"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
