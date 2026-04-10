use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::Colour;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::db;

pub async fn handle_close(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let reason = if args.is_empty() {
        None
    } else {
        Some(args.join(" "))
    };

    let Some(pool) = ({
        let data = ctx.data.read().await;
        data.get::<db::DbPoolKey>().cloned()
    }) else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let guild_id_i64 = guild_id.get() as i64;
    let channel_id = msg.channel_id.get() as i64;

    let Some(ticket) = db::get_ticket_by_channel(&pool, bot_id, guild_id_i64, channel_id)
        .await
        .ok()
        .flatten()
    else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Erreur")
                .description("Ce salon n'est pas reconnu comme un ticket.")
                .color(0xED4245),
        )
        .await;
        return;
    };

    if db::close_ticket(&pool, ticket.id, reason.clone())
        .await
        .is_err()
    {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Erreur")
                .description("Impossible de fermer ce ticket.")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let mut embed = CreateEmbed::new()
        .title("Ticket fermé")
        .description(format!("Le ticket #{} a été fermé.", ticket.id))
        .colour(Colour::from_rgb(255, 120, 0))
        .timestamp(Utc::now());

    if let Some(reason) = reason {
        embed = embed.field("Raison", reason, false);
    }

    send_embed(ctx, msg, embed).await;
}

pub struct CloseCommand;
pub static COMMAND_DESCRIPTOR: CloseCommand = CloseCommand;

impl crate::commands::command_contract::CommandSpec for CloseCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "close",
            category: "ticket",
            params: "[raison...]",
            description: "Ferme le ticket courant et enregistre optionnellement une raison.",
            examples: &["+close", "+close Raison", "+help close"],
            default_aliases: &[],
            allow_in_dm: false,
            default_permission: 5,
        }
    }
}
