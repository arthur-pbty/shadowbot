use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;

pub async fn handle_serverbanner(ctx: &Context, msg: &Message, _args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Ok(guild) = guild_id.to_partial_guild(&ctx).await else {
        return;
    };

    let banner_url = guild.banner_url().unwrap_or_default();
    if banner_url.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Ce serveur n'a pas de banniere.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let embed = CreateEmbed::new()
        .title(format!("Banniere du serveur {}", guild.name))
        .image(banner_url)
        .color(0x5865F2);
    send_embed(ctx, msg, embed).await;
}

pub struct ServerbannerCommand;
pub static COMMAND_DESCRIPTOR: ServerbannerCommand = ServerbannerCommand;

impl crate::commands::command_contract::CommandSpec for ServerbannerCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "serverbanner",
            category: "info",
            params: "aucun",
            description: "Affiche la banniere du serveur courant.",
            examples: &["+serverbanner", "+sbn", "+help serverbanner"],
            default_aliases: &["sbn"],
            allow_in_dm: false,
            default_permission: 0,
        }
    }
}
