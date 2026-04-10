use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_channel_helpers::edit_channel_visibility;

pub async fn handle_hideall(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    let Ok(channels) = guild_id.channels(&ctx.http).await else {
        return;
    };

    let mut changed = 0usize;
    for channel_id in channels.keys() {
        if edit_channel_visibility(ctx, guild_id, *channel_id, None, Some(true)).await {
            changed += 1;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("HideAll")
            .description(format!("{} salon(s) mis a jour.", changed))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct HideallCommand;
pub static COMMAND_DESCRIPTOR: HideallCommand = HideallCommand;

impl crate::commands::command_contract::CommandSpec for HideallCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "hideall",
            category: "salons_vocal",
            params: "aucun",
            summary: "Cache tous les salons",
            description: "Retire la visibilite de tous les salons.",
            examples: &["+hideall"],
            default_aliases: &["hda"],
            default_permission: 8,
        }
    }
}
