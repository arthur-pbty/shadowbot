use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_channel_id, send_embed, theme_color};
use crate::commands::moderation_channel_helpers::edit_channel_visibility;

pub async fn handle_unlock(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let target = args
        .first()
        .and_then(|raw| parse_channel_id(raw))
        .unwrap_or(msg.channel_id);

    let ok = edit_channel_visibility(ctx, guild_id, target, Some(false), None).await;

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Unlock")
            .description(if ok {
                format!("Salon <#{}> mis a jour.", target.get())
            } else {
                "Echec de mise a jour du salon.".to_string()
            })
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct UnlockCommand;
pub static COMMAND_DESCRIPTOR: UnlockCommand = UnlockCommand;
impl crate::commands::command_contract::CommandSpec for UnlockCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "unlock",
            category: "channel",
            params: "[salon]",
            description: "Deverrouille un salon texte ou vocal.",
            examples: &["+unlock", "+unlock #general"],
            default_aliases: &["ulk"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
