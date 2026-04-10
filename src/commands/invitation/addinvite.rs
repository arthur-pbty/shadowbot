use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::parse_user_id;
use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, add_invite_count};

pub async fn handle_addinvite(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    if args.is_empty() || args.len() > 2 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+addinvite [@user/id] <nombre>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let (target_user, amount_arg) = if args.len() == 1 {
        (msg.author.id, args[0])
    } else {
        let Some(user_id) = parse_user_id(args[0]) else {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description("Utilisateur invalide. Utilise une mention ou un ID.")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        };
        (user_id, args[1])
    };

    let Ok(amount) = amount_arg.parse::<i64>() else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Le nombre doit etre un entier positif.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if amount <= 0 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Le nombre doit etre superieur a 0.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let Some(pool) = ({
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    }) else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let guild_id_raw = guild_id.get() as i64;
    let user_id_raw = target_user.get() as i64;

    let Ok(new_total) = add_invite_count(&pool, bot_id, guild_id_raw, user_id_raw, amount).await
    else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Impossible de mettre a jour les invitations.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let embed = CreateEmbed::new()
        .title("AddInvite")
        .description(format!(
            "{} invitation(s) ajoutee(s) a <@{}>.\nNouveau total: `{}`.",
            amount,
            target_user.get(),
            new_total
        ))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub struct AddInviteCommand;
pub static COMMAND_DESCRIPTOR: AddInviteCommand = AddInviteCommand;

impl crate::commands::command_contract::CommandSpec for AddInviteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "addinvite",
            category: "invitation",
            params: "[@membre/ID] <nombre>",
            description: "Ajoute un nombre specifique d invitations a un utilisateur.",
            examples: &[
                "+addinvite @User 3",
                "+addinvite 123456789012345678 2",
                "+help addinvite",
            ],
            default_aliases: &["ainv"],
            allow_in_dm: false,
            default_permission: 5,
        }
    }
}
