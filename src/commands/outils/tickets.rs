use chrono::Utc;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use serenity::model::Colour;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::db;

pub async fn handle_tickets(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let page = args
        .first()
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(1)
        .max(1);

    let Some(pool) = ({
        let data = ctx.data.read().await;
        data.get::<db::DbPoolKey>().cloned()
    }) else {
        return;
    };

    let limit = 10i64;
    let offset = (page - 1) * limit;
    let bot_id = ctx.cache.current_user().id.get() as i64;

    let tickets = db::get_guild_tickets(&pool, bot_id, guild_id.get() as i64, limit, offset)
        .await
        .unwrap_or_default();

    if tickets.is_empty() {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Tickets")
                .description("Aucun ticket trouvé.")
                .colour(Colour::from_rgb(100, 100, 100)),
        )
        .await;
        return;
    }

    let mut description = String::new();
    for ticket in tickets {
        description.push_str(&format!(
            "**#{} - {}**\nAuteur: <@{}> | Statut: {}\n\n",
            ticket.id, ticket.title, ticket.creator_id, ticket.status
        ));
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Tickets")
            .description(description)
            .colour(Colour::from_rgb(0, 100, 200))
            .footer(CreateEmbedFooter::new(format!("Page {}", page)))
            .timestamp(Utc::now()),
    )
    .await;
}

pub struct TicketsCommand;
pub static COMMAND_DESCRIPTOR: TicketsCommand = TicketsCommand;

impl crate::commands::command_contract::CommandSpec for TicketsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "tickets",
            category: "outils",
            params: "[page]",
            summary: "Liste les tickets",
            description: "Affiche les tickets du serveur avec pagination.",
            examples: &["+tickets", "+tickets 2", "+help tickets"],
            default_aliases: &[],
            default_permission: 2,
        }
    }
}
