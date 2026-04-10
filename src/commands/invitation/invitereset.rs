use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::parse_user_id;
use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, reset_invite_count_for_user, reset_invite_counts_for_guild};

pub async fn handle_invitereset(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    if args.len() != 1 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+invitereset <@user|guild>`")
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
    let target = args[0].to_lowercase();

    if matches!(target.as_str(), "guild" | "serveur" | "server") {
        let Ok(affected) = reset_invite_counts_for_guild(&pool, bot_id, guild_id_raw).await else {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description("Impossible de reinitialiser les invitations du serveur.")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        };

        let embed = CreateEmbed::new()
            .title("InviteReset")
            .description(format!(
                "Compteurs d invitations du serveur reinitialises. Entrees supprimees: `{}`.",
                affected
            ))
            .color(0x57F287);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let Some(user_id) = parse_user_id(args[0]) else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Cible invalide. Utilise une mention utilisateur, un ID, ou `guild`.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let Ok(_) =
        reset_invite_count_for_user(&pool, bot_id, guild_id_raw, user_id.get() as i64).await
    else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Impossible de reinitialiser les invitations de cet utilisateur.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let embed = CreateEmbed::new()
        .title("InviteReset")
        .description(format!(
            "Compteur d invitations reinitialise pour <@{}>.",
            user_id.get()
        ))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub struct InviteResetCommand;
pub static COMMAND_DESCRIPTOR: InviteResetCommand = InviteResetCommand;

impl crate::commands::command_contract::CommandSpec for InviteResetCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "invitereset",
            category: "invitation",
            params: "<@membre/ID|guild>",
            description: "Reinitialise le compteur d invitations pour un utilisateur ou le serveur.",
            examples: &[
                "+invitereset @User",
                "+invitereset guild",
                "+help invitereset",
            ],
            default_aliases: &["invreset"],
            allow_in_dm: false,
            default_permission: 5,
        }
    }
}
