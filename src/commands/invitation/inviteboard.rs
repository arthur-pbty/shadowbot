use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, list_invite_board};

pub async fn handle_inviteboard(ctx: &Context, msg: &Message, _args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = ({
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    }) else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let guild_id_raw = guild_id.get() as i64;

    let Ok(entries) = list_invite_board(&pool, bot_id, guild_id_raw, 10).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Impossible de lire le classement des invitations.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if entries.is_empty() {
        let embed = CreateEmbed::new()
            .title("InviteBoard")
            .description("Aucune invitation enregistree pour le moment.")
            .color(0x5865F2);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let lines = entries
        .iter()
        .enumerate()
        .map(|(idx, (user_id, count))| {
            format!("`#{}` <@{}> - `{}` invitation(s)", idx + 1, user_id, count)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let embed = CreateEmbed::new()
        .title("InviteBoard")
        .description(lines)
        .color(0x5865F2);
    send_embed(ctx, msg, embed).await;
}

pub struct InviteBoardCommand;
pub static COMMAND_DESCRIPTOR: InviteBoardCommand = InviteBoardCommand;

impl crate::commands::command_contract::CommandSpec for InviteBoardCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "inviteboard",
            category: "invitation",
            params: "aucun",
            description: "Affiche les 10 membres du serveur avec le plus d invitations.",
            examples: &["+inviteboard", "+iboard", "+help inviteboard"],
            default_aliases: &["iboard"],
            allow_in_dm: false,
            default_permission: 0,
        }
    }
}
