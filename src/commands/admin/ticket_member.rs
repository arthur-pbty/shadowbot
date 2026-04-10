use chrono::Utc;
use serenity::all::{PermissionOverwrite, PermissionOverwriteType, Permissions};
use serenity::builder::{CreateEmbed, EditChannel};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::parse_user_id;
use crate::commands::common::send_embed;
use crate::db;

const TICKET_ALLOW: Permissions = Permissions::VIEW_CHANNEL
    .union(Permissions::SEND_MESSAGES)
    .union(Permissions::READ_MESSAGE_HISTORY)
    .union(Permissions::ATTACH_FILES)
    .union(Permissions::EMBED_LINKS)
    .union(Permissions::ADD_REACTIONS);

fn ticket_member_id(args: &[&str], msg: &Message) -> Option<UserId> {
    msg.mentions
        .first()
        .map(|user| user.id)
        .or_else(|| args.first().and_then(|value| parse_user_id(value)))
}

async fn ticket_member_update(
    ctx: &Context,
    msg: &Message,
    args: &[&str],
    allow: bool,
) -> Result<(), ()> {
    let Some(guild_id) = msg.guild_id else {
        return Err(());
    };

    let Some(user_id) = ticket_member_id(args, msg) else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Erreur")
                .description("Utilisateur introuvable.")
                .color(0xED4245),
        )
        .await;
        return Err(());
    };

    let Some(pool) = ({
        let data = ctx.data.read().await;
        data.get::<db::DbPoolKey>().cloned()
    }) else {
        return Err(());
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
        return Err(());
    };

    let Ok(channel) = msg.channel_id.to_channel(&ctx.http).await else {
        return Err(());
    };

    let Channel::Guild(guild_channel) = channel else {
        return Err(());
    };

    let mut overwrites = guild_channel.permission_overwrites.clone();
    overwrites.retain(
        |overwrite| !matches!(overwrite.kind, PermissionOverwriteType::Member(id) if id == user_id),
    );

    if allow {
        overwrites.push(PermissionOverwrite {
            allow: TICKET_ALLOW,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(user_id),
        });
    }

    if msg
        .channel_id
        .edit(&ctx.http, EditChannel::new().permissions(overwrites))
        .await
        .is_err()
    {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Erreur")
                .description("Impossible de mettre à jour les permissions du ticket.")
                .color(0xED4245),
        )
        .await;
        return Err(());
    }

    if allow {
        let _ = db::add_ticket_member(&pool, ticket.id, user_id.get() as i64).await;
    } else {
        let _ = db::remove_ticket_member(&pool, ticket.id, user_id.get() as i64).await;
    }

    let title = if allow {
        "Membre ajouté"
    } else {
        "Membre retiré"
    };
    let description = if allow {
        format!("<@{}> a été ajouté au ticket.", user_id.get())
    } else {
        format!("<@{}> a été retiré du ticket.", user_id.get())
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title(title)
            .description(description)
            .colour(Colour::from_rgb(0, 200, 120))
            .timestamp(Utc::now()),
    )
    .await;

    Ok(())
}

pub async fn handle_ticket_add(ctx: &Context, msg: &Message, args: &[&str]) {
    let _ = ticket_member_update(ctx, msg, args, true).await;
}

pub async fn handle_ticket_remove(ctx: &Context, msg: &Message, args: &[&str]) {
    let _ = ticket_member_update(ctx, msg, args, false).await;
}
