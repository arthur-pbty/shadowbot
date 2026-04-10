use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_channel_id, send_embed, theme_color};
use crate::commands::moderation_channel_helpers::edit_channel_visibility;

pub async fn handle_lock(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let target = args
        .first()
        .and_then(|raw| parse_channel_id(raw))
        .unwrap_or(msg.channel_id);

    let ok = edit_channel_visibility(ctx, guild_id, target, Some(true), None).await;

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Lock")
            .description(if ok {
                format!("Salon <#{}> mis a jour.", target.get())
            } else {
                "Echec de mise a jour du salon.".to_string()
            })
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct LockCommand;
pub static COMMAND_DESCRIPTOR: LockCommand = LockCommand;
impl crate::commands::command_contract::CommandSpec for LockCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "lock",
            category: "salons_vocal",
            params: "[salon]",
            summary: "Ferme un salon",
            description: "Verrouille un salon texte ou vocal.",
            examples: &["+lock", "+lock #general"],
            default_aliases: &["lk"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
