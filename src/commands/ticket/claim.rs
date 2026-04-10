use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::Colour;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::db;

pub async fn handle_claim(ctx: &Context, msg: &Message, _args: &[&str]) {
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

    if db::claim_ticket(&pool, ticket.id, msg.author.id.get() as i64)
        .await
        .is_err()
    {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Erreur")
                .description("Impossible de revendiquer ce ticket.")
                .color(0xED4245),
        )
        .await;
        return;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Ticket revendiqué")
            .description(format!("Le ticket #{} a été revendiqué.", ticket.id))
            .colour(Colour::from_rgb(0, 200, 120))
            .timestamp(Utc::now()),
    )
    .await;
}

pub struct ClaimCommand;
pub static COMMAND_DESCRIPTOR: ClaimCommand = ClaimCommand;

impl crate::commands::command_contract::CommandSpec for ClaimCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "claim",
            category: "ticket",
            params: "aucun",
            description: "Assigne le ticket courant au moderateur qui execute la commande.",
            examples: &["+claim", "+help claim"],
            default_aliases: &[],
            allow_in_dm: false,
            default_permission: 5,
        }
    }
}
