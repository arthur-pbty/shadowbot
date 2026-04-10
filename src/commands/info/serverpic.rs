use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;

pub async fn handle_serverpic(ctx: &Context, msg: &Message, _args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Ok(guild) = guild_id.to_partial_guild(&ctx).await else {
        return;
    };

    let icon_url = guild.icon_url().unwrap_or_default();
    if icon_url.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Ce serveur n'a pas d'icone.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let embed = CreateEmbed::new()
        .title(format!("Icone du serveur {}", guild.name))
        .image(icon_url)
        .color(0x5865F2);
    send_embed(ctx, msg, embed).await;
}

pub struct ServerpicCommand;
pub static COMMAND_DESCRIPTOR: ServerpicCommand = ServerpicCommand;

impl crate::commands::command_contract::CommandSpec for ServerpicCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "serverpic",
            category: "info",
            params: "aucun",
            description: "Affiche l'icone du serveur courant.",
            examples: &["+serverpic", "+spi", "+help serverpic"],
            default_aliases: &["spi"],
            allow_in_dm: false,
            default_permission: 0,
        }
    }
}
