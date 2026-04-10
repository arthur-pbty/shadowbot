use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::commands::server::resolve_guild_target;

pub async fn handle_invite(ctx: &Context, msg: &Message, args: &[&str]) {
    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+invite <ID/nombre>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let Some(guild_id) = resolve_guild_target(ctx, args[0]).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Serveur introuvable.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let Ok(channels) = guild_id.channels(&ctx.http).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Impossible de lire les salons.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let mut invite_url = None;
    for channel in channels.values() {
        if matches!(channel.kind, ChannelType::Text | ChannelType::News) {
            if let Ok(invite) = channel
                .create_invite(
                    &ctx.http,
                    serenity::builder::CreateInvite::new().max_age(3600),
                )
                .await
            {
                invite_url = Some(invite.url());
                break;
            }
        }
    }

    let Some(url) = invite_url else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Aucun salon éligible pour créer une invitation.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let embed = CreateEmbed::new()
        .title("Invitation créée")
        .description(url)
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}
pub struct InviteCommand;
pub static COMMAND_DESCRIPTOR: InviteCommand = InviteCommand;

impl crate::commands::command_contract::CommandSpec for InviteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "invite",
            command: "invite",
            category: "admin",
            params: "<ID_serveur/index>",
            summary: "Cree une invitation serveur",
            description: "Cree une invitation temporaire sur un serveur cible accessible par le bot.",
            examples: &["+invite", "+ie", "+help invite"],
            alias_source_key: "invite",
            default_aliases: &["ivt"],
        }
    }
}
