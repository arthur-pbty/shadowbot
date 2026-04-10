use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::Colour;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_channel_id, send_embed};
use crate::db;

pub async fn handle_autopublish(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    if !args.is_empty() {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Autopublish")
                .description("Utilisation: +autopublish")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let Some(pool) = ({
        let data = ctx.data.read().await;
        data.get::<db::DbPoolKey>().cloned()
    }) else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let guild_id_i64 = guild_id.get() as i64;

    let channels = db::get_autopublish_channels(&pool, bot_id, guild_id_i64)
        .await
        .unwrap_or_default();
    let description = if channels.is_empty() {
        "Aucun salon d'annonces configuré.".to_string()
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
            .title("Autopublish")
            .description(description)
            .colour(Colour::from_rgb(100, 150, 255)),
    )
    .await;
}

pub async fn handle_autopublishon(ctx: &Context, msg: &Message, args: &[&str]) {
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
    let channel_id = args
        .first()
        .and_then(|value| parse_channel_id(value))
        .unwrap_or(msg.channel_id);

    let result = db::add_autopublish_channel(&pool, bot_id, guild_id_i64, channel_id.get() as i64).await;

    if result.is_err() {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Autopublish")
                .description("Impossible de mettre à jour le salon d'annonces.")
                .color(0xED4245),
        )
        .await;
        return;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Autopublish activé")
            .description(format!("Salon: <#{}>", channel_id.get()))
            .colour(Colour::from_rgb(0, 200, 120))
            .timestamp(Utc::now()),
    )
    .await;
}

pub async fn handle_autopublishoff(ctx: &Context, msg: &Message, args: &[&str]) {
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
    let channel_id = args
        .first()
        .and_then(|value| parse_channel_id(value))
        .unwrap_or(msg.channel_id);

    let result =
        db::remove_autopublish_channel(&pool, bot_id, guild_id_i64, channel_id.get() as i64).await;

    if result.is_err() {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Autopublish")
                .description("Impossible de mettre à jour le salon d'annonces.")
                .color(0xED4245),
        )
        .await;
        return;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Autopublish désactivé")
            .description(format!("Salon: <#{}>", channel_id.get()))
            .colour(Colour::from_rgb(255, 120, 0))
            .timestamp(Utc::now()),
    )
    .await;
}

pub struct AutopublishCommand;
pub static COMMAND_DESCRIPTOR: AutopublishCommand = AutopublishCommand;

impl crate::commands::command_contract::CommandSpec for AutopublishCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "autopublish",
            category: "automation",
            params: "aucun",
            description: "Affiche les salons ou la publication automatique des annonces est active.",
            examples: &["+autopublish", "+help autopublish"],
            default_aliases: &["apb"],
            allow_in_dm: false,
            default_permission: 5,
        }
    }
}
