use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::parse_channel_id;
use crate::commands::common::{discord_ts, send_embed};

pub async fn handle_channel(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Ok(guild) = guild_id.to_partial_guild(&ctx).await else {
        return;
    };

    let channels = guild.channels(ctx).await.unwrap_or_default();

    let channel_id = if !args.is_empty() {
        let search = args.join(" ").to_lowercase();
        parse_channel_id(args[0]).or_else(|| {
            // Chercher par nom de canal
            channels
                .iter()
                .find(|(_, c)| c.name.to_lowercase().contains(&search))
                .map(|(id, _)| *id)
        })
    } else {
        Some(msg.channel_id)
    };

    let Some(channel_id) = channel_id else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Impossible de parser le canal.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let Ok(channel) = channel_id.to_channel(&ctx).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Canal non trouvé.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    match channel {
        Channel::Guild(gc) => {
            let created_at = discord_ts(gc.id.created_at(), "F");
            let channel_type = match gc.kind {
                ChannelType::Text => "Texte",
                ChannelType::Voice => "Vocal",
                ChannelType::Private => "Privé",
                ChannelType::Category => "Catégorie",
                ChannelType::News => "Annonces",
                ChannelType::Stage => "Stage",
                ChannelType::Directory => "Répertoire",
                ChannelType::Forum => "Forum",
                ChannelType::Unknown(_) => "Inconnu",
                _ => "Autre",
            };

            let mut embed = CreateEmbed::new()
                .title(&gc.name)
                .description(format!("ID: `{}`", gc.id.get()))
                .color(0x5865F2)
                .field("Type", channel_type, true)
                .field("Créé", created_at, true);

            if let Some(topic) = &gc.topic {
                embed = embed.field("Sujet", topic, false);
            }

            if let Some(bitrate) = gc.bitrate {
                embed = embed.field("Bitrate", format!("{} kbps", bitrate / 1000), true);
            }

            embed = embed.field("NSFW", if gc.nsfw { "Oui" } else { "Non" }, true);

            send_embed(ctx, msg, embed).await;
        }
        _ => {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description("Ce type de canal n'est pas supporté.")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
        }
    }
}

pub struct ChannelCommand;
pub static COMMAND_DESCRIPTOR: ChannelCommand = ChannelCommand;

impl crate::commands::command_contract::CommandSpec for ChannelCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "channel",
            category: "infos",
            params: "<#salon/ID>",
            description: "Affiche les informations utiles dun salon texte ou vocal cible.",
            examples: &["+channel", "+cl", "+help channel"],
            default_aliases: &["chl"],
            allow_in_dm: false,
            default_permission: 0,
        }
    }
}
