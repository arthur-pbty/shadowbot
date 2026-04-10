use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::parse_user_id;
use crate::commands::common::{send_embed, theme_color};

pub async fn handle_voicekick(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    if args.is_empty() {
        return;
    }

    let mut kicked = 0usize;
    for raw in args {
        if let Some(user_id) = parse_user_id(raw) {
            if guild_id.disconnect_member(&ctx.http, user_id).await.is_ok() {
                kicked += 1;
            }
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("VoiceKick")
            .description(format!("{} membres déconnectés.", kicked))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct VoiceKickCommand;
pub static COMMAND_DESCRIPTOR: VoiceKickCommand = VoiceKickCommand;

impl crate::commands::command_contract::CommandSpec for VoiceKickCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "voicekick",
            category: "channel",
            params: "<membre...>",
            description: "Deconnecte un ou plusieurs membres de leur salon vocal actuel.",
            examples: &["+voicekick @User", "+voicekick @U1 @U2"],
            default_aliases: &["vk", "vdisconnect"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
