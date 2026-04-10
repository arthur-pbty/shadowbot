use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;

pub async fn handle_vocinfo(ctx: &Context, msg: &Message, _args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Ok(guild) = guild_id.to_partial_guild(&ctx).await else {
        return;
    };

    let channels = guild.channels(ctx).await.unwrap_or_default();

    let voice_channels: Vec<_> = channels
        .iter()
        .filter(|(_, c)| c.kind == ChannelType::Voice)
        .collect();

    let stage_channels: Vec<_> = channels
        .iter()
        .filter(|(_, c)| c.kind == ChannelType::Stage)
        .collect();

    let mut total_members = 0;
    let mut stage_members = 0;

    // Récupérer la guild depuis le cache pour accéder aux voice states
    if let Some(cached_guild) = ctx.cache.guild(guild_id) {
        for (_, voice_state) in &cached_guild.voice_states {
            if let Some(channel_id) = voice_state.channel_id {
                let channel_type = channels
                    .iter()
                    .find(|(cid, _)| **cid == channel_id)
                    .map(|(_, c)| c.kind);

                match channel_type {
                    Some(ChannelType::Voice) => total_members += 1,
                    Some(ChannelType::Stage) => stage_members += 1,
                    _ => {}
                }
            }
        }
    }

    let embed = CreateEmbed::new()
        .title(format!("Informations vocales - {}", guild.name))
        .color(0x5865F2)
        .field("Canaux vocaux", format!("{}", voice_channels.len()), true)
        .field("Canaux Stage", format!("{}", stage_channels.len()), true)
        .field("Membres en vocal", format!("{}", total_members), true)
        .field("Membres en Stage", format!("{}", stage_members), true)
        .field(
            "Total actif",
            format!("{}", total_members + stage_members),
            true,
        );

    send_embed(ctx, msg, embed).await;
}

pub struct VocinfoCommand;
pub static COMMAND_DESCRIPTOR: VocinfoCommand = VocinfoCommand;

impl crate::commands::command_contract::CommandSpec for VocinfoCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "vocinfo",
            command: "vocinfo",
            category: "general",
            params: "[ID_salon_vocal]",
            summary: "Affiche les infos vocales",
            description: "Affiche les informations dun salon vocal cible ou du salon vocal courant.",
            examples: &["+vocinfo", "+vo", "+help vocinfo"],
            alias_source_key: "vocinfo",
            default_aliases: &["vci"],
            default_permission: 0,
        }
    }
}
