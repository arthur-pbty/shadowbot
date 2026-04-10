use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_channel_id, send_embed, theme_color};
use crate::commands::moderation_channel_helpers::edit_channel_visibility;

pub async fn handle_unhide(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let target = args
        .first()
        .and_then(|raw| parse_channel_id(raw))
        .unwrap_or(msg.channel_id);

    let ok = edit_channel_visibility(ctx, guild_id, target, None, Some(false)).await;

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("UnHide")
            .description(if ok {
                format!("Salon <#{}> mis a jour.", target.get())
            } else {
                "Echec de mise a jour du salon.".to_string()
            })
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct UnhideCommand;
pub static COMMAND_DESCRIPTOR: UnhideCommand = UnhideCommand;
impl crate::commands::command_contract::CommandSpec for UnhideCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "unhide",
            category: "salons_vocal",
            params: "[salon]",
            summary: "Affiche un salon",
            description: "Rend a nouveau visible un salon.",
            examples: &["+unhide", "+unhide #general"],
            default_aliases: &["uhd"],
            default_permission: 8,
        }
    }
}
