use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_channel_id, send_embed, theme_color};

pub async fn handle_cleanup(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(channel_raw) = args.first() else {
        return;
    };
    let Some(channel_id) = parse_channel_id(channel_raw) else {
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
                if state.channel_id == Some(channel_id) {
                    Some(*uid)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };

    let mut kicked = 0usize;
    for user_id in user_ids {
        if guild_id.disconnect_member(&ctx.http, user_id).await.is_ok() {
            kicked += 1;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Cleanup")
            .description(format!("{} utilisateurs déconnectés.", kicked))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct CleanupCommand;
pub static COMMAND_DESCRIPTOR: CleanupCommand = CleanupCommand;

impl crate::commands::command_contract::CommandSpec for CleanupCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "cleanup",
            category: "moderation",
            params: "<salon_vocal>",
            description: "Deconnecte tous les utilisateurs presents dans un salon vocal cible.",
            examples: &["+cleanup #General"],
            default_aliases: &["vclean", "vcleanup"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
