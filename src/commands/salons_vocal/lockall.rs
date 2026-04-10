use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_channel_helpers::edit_channel_visibility;

pub async fn handle_lockall(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    let Ok(channels) = guild_id.channels(&ctx.http).await else {
        return;
    };

    let mut changed = 0usize;
    for channel_id in channels.keys() {
        if edit_channel_visibility(ctx, guild_id, *channel_id, Some(true), None).await {
            changed += 1;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("LockAll")
            .description(format!("{} salon(s) mis a jour.", changed))
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct LockallCommand;
pub static COMMAND_DESCRIPTOR: LockallCommand = LockallCommand;
impl crate::commands::command_contract::CommandSpec for LockallCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "lockall",
            category: "salons_vocal",
            params: "aucun",
            summary: "Ferme tous les salons",
            description: "Verrouille tous les salons du serveur.",
            examples: &["+lockall"],
            default_aliases: &["lka"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
