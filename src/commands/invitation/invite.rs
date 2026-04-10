use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::parse_user_id;
use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, get_invite_count};

pub async fn handle_invite(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    if args.len() > 1 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+invite [@user/id]`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let target_user = if args.is_empty() {
        msg.author.id
    } else {
        let Some(user_id) = parse_user_id(args[0]) else {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description("Utilisateur invalide. Utilise une mention ou un ID.")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        };

        user_id
    };

    let Some(pool) = ({
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    }) else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let guild_id_raw = guild_id.get() as i64;
    let user_id_raw = target_user.get() as i64;

    let Ok(invite_count) = get_invite_count(&pool, bot_id, guild_id_raw, user_id_raw).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Impossible de lire le compteur d invitations.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let embed = CreateEmbed::new()
        .title("Invite")
        .description(format!(
            "<@{}> a actuellement `{}` invitation(s).",
            target_user.get(),
            invite_count
        ))
        .color(0x5865F2);
    send_embed(ctx, msg, embed).await;
}

pub struct InviteCommand;
pub static COMMAND_DESCRIPTOR: InviteCommand = InviteCommand;

impl crate::commands::command_contract::CommandSpec for InviteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "invite",
            category: "invitation",
            params: "[@membre/ID]",
            description: "Affiche le nombre d invitations d un utilisateur.",
            examples: &["+invite", "+invite @User", "+help invite"],
            default_aliases: &["inv"],
            allow_in_dm: false,
            default_permission: 0,
        }
    }
}
