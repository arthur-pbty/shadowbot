use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{discord_ts, send_embed};

pub async fn handle_serverinfo(ctx: &Context, msg: &Message, _args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Ok(guild) = guild_id.to_partial_guild(&ctx).await else {
        return;
    };

    let members_count = guild
        .members(ctx, None, None)
        .await
        .ok()
        .map(|m| m.len())
        .unwrap_or(0);

    let channels = guild.channels(ctx).await.unwrap_or_default();
    let channels_count = channels.len();
    let roles_count = guild.roles.len();
    let boost_tier = format!("{:?}", guild.premium_tier);
    let boost_count = guild.premium_subscription_count.unwrap_or(0);

    let created_at = discord_ts(guild.id.created_at(), "F");
    let icon_url = guild.icon_url().unwrap_or_default();

    let mut embed = CreateEmbed::new()
        .title(&guild.name)
        .description(format!("ID: `{}`", guild.id.get()))
        .color(0x5865F2)
        .thumbnail(&icon_url)
        .field("Créé", created_at, true)
        .field("Membres", format!("{}", members_count), true)
        .field("Canaux", format!("{}", channels_count), true)
        .field("Rôles", format!("{}", roles_count), true)
        .field("Niveau Boost", boost_tier, true)
        .field("Boosts", format!("{}", boost_count), true);

    if guild.owner_id.get() != 0 {
        embed = embed.field(
            "Propriétaire",
            format!("<@{}>", guild.owner_id.get()),
            false,
        );
    }

    send_embed(ctx, msg, embed).await;
}

pub struct ServerinfoCommand;
pub static COMMAND_DESCRIPTOR: ServerinfoCommand = ServerinfoCommand;

impl crate::commands::command_contract::CommandSpec for ServerinfoCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "serverinfo",
            category: "infos",
            params: "[ID_serveur]",
            summary: "Affiche les infos dun serveur",
            description: "Affiche les informations principales dun serveur comme nom, id et statistiques.",
            examples: &["+serverinfo", "+so", "+help serverinfo"],
            default_aliases: &["svi"],
            allow_in_dm: false,
            default_permission: 0,
        }
    }
}
