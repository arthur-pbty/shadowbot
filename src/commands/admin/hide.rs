use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_channel_id, send_embed, theme_color};
use crate::commands::moderation_channel_helpers::edit_channel_visibility;

pub async fn handle_hide(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let target = args
        .first()
        .and_then(|raw| parse_channel_id(raw))
        .unwrap_or(msg.channel_id);

    let ok = edit_channel_visibility(ctx, guild_id, target, None, Some(true)).await;

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Hide")
            .description(if ok {
                format!("Salon <#{}> mis a jour.", target.get())
            } else {
                "Echec de mise a jour du salon.".to_string()
            })
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct HideCommand;
pub static COMMAND_DESCRIPTOR: HideCommand = HideCommand;
impl crate::commands::command_contract::CommandSpec for HideCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "hide",
            category: "admin",
            params: "[salon]",
            summary: "Cache un salon",
            description: "Retire la visibilite d un salon.",
            examples: &["+hide", "+hide #general"],
            default_aliases: &["hd"],
            default_permission: 8,
        }
    }
}
