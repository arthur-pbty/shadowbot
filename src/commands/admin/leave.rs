use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::commands::server::resolve_guild_target;

pub async fn handle_leave(ctx: &Context, msg: &Message, args: &[&str]) {
    let target = if args.is_empty() {
        msg.guild_id
    } else {
        resolve_guild_target(ctx, args[0]).await
    };

    let Some(guild_id) = target else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Serveur introuvable.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let _ = guild_id.leave(&ctx.http).await;
    let embed = CreateEmbed::new()
        .title("Serveur quitté")
        .description(format!("Le bot a quitté `{}`.", guild_id.get()))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}
pub struct LeaveCommand;
pub static COMMAND_DESCRIPTOR: LeaveCommand = LeaveCommand;

impl crate::commands::command_contract::CommandSpec for LeaveCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "leave",
            category: "admin",
            params: "[ID_serveur/index]",
            summary: "Fait quitter un serveur",
            description: "Force le bot a quitter un serveur cible ou le serveur courant.",
            examples: &["+leave", "+le", "+help leave"],
            default_aliases: &["lvg"],
            default_permission: 9,
        }
    }
}
