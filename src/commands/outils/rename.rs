use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::Colour;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::db;

fn sanitize_ticket_name(input: &str) -> String {
    let mut out = String::new();
    let mut previous_dash = false;

    for ch in input.to_lowercase().chars() {
        let keep = ch.is_ascii_alphanumeric();
        let dash = ch.is_whitespace() || ch == '-' || ch == '_';

        if keep {
            out.push(ch);
            previous_dash = false;
        } else if dash && !previous_dash {
            out.push('-');
            previous_dash = true;
        }
    }

    out.trim_matches('-').to_string()
}

pub async fn handle_rename(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let new_name_raw = args.join(" ").trim().to_string();
    if new_name_raw.is_empty() {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Erreur")
                .description("Utilisation: +rename <nom>")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let new_name = sanitize_ticket_name(&new_name_raw);
    if new_name.is_empty() {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Erreur")
                .description("Nom invalide.")
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

    if msg
        .channel_id
        .edit(
            &ctx.http,
            serenity::builder::EditChannel::new().name(new_name.clone()),
        )
        .await
        .is_err()
    {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Erreur")
                .description("Impossible de renommer le salon.")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let _ = db::update_ticket_title(&pool, ticket.id, &new_name).await;

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Ticket renommé")
            .description(format!("Nouveau nom: `{}`", new_name))
            .colour(Colour::from_rgb(0, 200, 120))
            .timestamp(Utc::now()),
    )
    .await;
}

pub struct RenameCommand;
pub static COMMAND_DESCRIPTOR: RenameCommand = RenameCommand;

impl crate::commands::command_contract::CommandSpec for RenameCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "rename",
            category: "outils",
            params: "<nom...>",
            summary: "Renomme le ticket courant",
            description: "Renomme le salon du ticket et met a jour son titre en base.",
            examples: &["+rename support-client", "+help rename"],
            default_aliases: &[],
            default_permission: 2,
        }
    }
}
