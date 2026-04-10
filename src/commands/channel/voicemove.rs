use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_channel_id, send_embed, theme_color};

pub async fn handle_voicemove(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.len() < 2 {
        return;
    }

    let Some(from_channel) = parse_channel_id(args[0]) else {
        return;
    };
    let Some(to_channel) = parse_channel_id(args[1]) else {
        return;
    };

    let user_ids = {
        let Some(guild) = guild_id.to_guild_cached(&ctx.cache) else {
            return;
        };

        guild
            .voice_states
            .iter()
            .filter_map(|(uid, state)| {
                if state.channel_id == Some(from_channel) {
                    Some(*uid)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };

    let mut moved = 0usize;
    for user_id in user_ids {
        if guild_id
            .move_member(&ctx.http, user_id, to_channel)
            .await
            .is_ok()
        {
            moved += 1;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("VoiceMove")
            .description(format!("{} membres déplacés.", moved))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct VoiceMoveCommand;
pub static COMMAND_DESCRIPTOR: VoiceMoveCommand = VoiceMoveCommand;

impl crate::commands::command_contract::CommandSpec for VoiceMoveCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "voicemove",
            category: "channel",
            params: "<salon_source> <salon_destination>",
            description: "Deplace tous les membres d'un salon vocal vers un autre salon.",
            examples: &["+voicemove #General #Event"],
            default_aliases: &["vmove", "vmoveall"],
            allow_in_dm: false,
            default_permission: 7,
        }
    }
}
