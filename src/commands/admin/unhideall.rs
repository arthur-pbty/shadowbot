use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_channel_helpers::edit_channel_visibility;

pub async fn handle_unhideall(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    let Ok(channels) = guild_id.channels(&ctx.http).await else {
        return;
    };

    let mut changed = 0usize;
    for channel_id in channels.keys() {
        if edit_channel_visibility(ctx, guild_id, *channel_id, None, Some(false)).await {
            changed += 1;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("UnHideAll")
            .description(format!("{} salon(s) mis a jour.", changed))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct UnhideallCommand;
pub static COMMAND_DESCRIPTOR: UnhideallCommand = UnhideallCommand;

impl crate::commands::command_contract::CommandSpec for UnhideallCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "unhideall",
            category: "admin",
            params: "aucun",
            summary: "Affiche tous les salons",
            description: "Rend visibles tous les salons du serveur.",
            examples: &["+unhideall"],
            default_aliases: &["uhda"],
            default_permission: 8,
        }
    }
}
