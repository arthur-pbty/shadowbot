use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::commands::server::resolve_guild_target;

pub async fn handle_discussion(ctx: &Context, msg: &Message, args: &[&str]) {
    if args.len() < 2 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+discussion <ID/nombre> <message>`")
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

    let content = args[1..].join(" ");
    let Ok(channels) = guild_id.channels(&ctx.http).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Impossible de lire les salons.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    for channel in channels.values() {
        if matches!(channel.kind, ChannelType::Text | ChannelType::News) {
            let _ = channel
                .say(
                    &ctx.http,
                    format!("[Discussion via {}] {}", msg.author.tag(), content),
                )
                .await;
            let embed = CreateEmbed::new()
                .title("Discussion envoyée")
                .description(format!("Message transmis dans `{}`.", guild_id.get()))
                .color(0x57F287);
            send_embed(ctx, msg, embed).await;
            return;
        }
    }

    let embed = CreateEmbed::new()
        .title("Erreur")
        .description("Aucun salon texte trouvable sur ce serveur.")
        .color(0xED4245);
    send_embed(ctx, msg, embed).await;
}
pub struct DiscussionCommand;
pub static COMMAND_DESCRIPTOR: DiscussionCommand = DiscussionCommand;

impl crate::commands::command_contract::CommandSpec for DiscussionCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "discussion",
            command: "discussion",
            category: "profile",
            params: "<ID_serveur/index> <message...>",
            summary: "Diffuse un message serveur",
            description: "Envoie un message de discussion sur un serveur cible.",
            examples: &["+discussion", "+dn", "+help discussion"],
            alias_source_key: "discussion",
            default_aliases: &["dsc"],
            default_permission: 8,
        }
    }
}
