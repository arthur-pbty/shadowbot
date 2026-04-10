use serenity::builder::{CreateEmbed, CreateMessage, EditMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Instant;

use crate::commands::common::{has_flag, theme_color};

pub async fn handle_ping(ctx: &Context, msg: &Message, args: &[&str]) {
    let detailed = has_flag(args, &["--details", "-d", "full"]);
    let color = theme_color(ctx).await;
    let start = Instant::now();

    let pending_embed = CreateEmbed::new()
        .title("Pong")
        .description("Mesure de la latence en cours...")
        .color(color);

    let sent = msg
        .channel_id
        .send_message(&ctx.http, CreateMessage::new().embed(pending_embed))
        .await;

    let Ok(mut sent_message) = sent else {
        return;
    };

    let latency_ms = start.elapsed().as_millis();

    let mut embed = CreateEmbed::new()
        .title("Pong")
        .description("Le bot répond correctement.")
        .color(color)
        .field("Latence", format!("{} ms", latency_ms), true);

    if detailed {
        embed = embed.field("Canal", format!("<#{}>", msg.channel_id.get()), true);
        if let Some(guild_id) = msg.guild_id {
            embed = embed.field("Serveur", guild_id.to_string(), true);
        }
    }

    let _ = sent_message
        .edit(&ctx.http, EditMessage::new().embed(embed))
        .await;
}

pub struct PingCommand;
pub static COMMAND_DESCRIPTOR: PingCommand = PingCommand;

impl crate::commands::command_contract::CommandSpec for PingCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "ping",
            category: "infos",
            params: "aucun",
            summary: "Mesure la latence du bot",
            description: "Affiche le temps de reponse du bot et met a jour un embed avec la latence calculee.",
            examples: &["+ping", "+pg", "+help ping"],
            default_aliases: &["pg"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
