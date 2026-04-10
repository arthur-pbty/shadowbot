use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_unbanall(ctx: &Context, msg: &Message, _args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let bans = guild_id
        .bans(&ctx.http, None, None)
        .await
        .unwrap_or_default();

    let mut unbanned = 0usize;
    for ban in bans {
        if guild_id.unban(&ctx.http, ban.user.id).await.is_ok() {
            unbanned += 1;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("UnbanAll")
            .description(format!("{} bannissements retirés.", unbanned))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct UnbanAllCommand;
pub static COMMAND_DESCRIPTOR: UnbanAllCommand = UnbanAllCommand;

impl crate::commands::command_contract::CommandSpec for UnbanAllCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "unbanall",
            category: "mod",
            params: "aucun",
            description: "Supprime tous les bans du serveur cible.",
            examples: &["+unbanall"],
            default_aliases: &["uball", "clearbans"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
