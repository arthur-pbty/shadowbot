use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::ensure_owner;

pub async fn handle_say(ctx: &Context, msg: &Message, args: &[&str]) {
    if ensure_owner(ctx, msg).await.is_err() {
        return;
    }

    if args.is_empty() {
        let embed = serenity::builder::CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+say <message>`")
            .color(0xED4245);
        crate::commands::common::send_embed(ctx, msg, embed).await;
        return;
    }

    let text = args.join(" ");
    let _ = msg.channel_id.say(&ctx.http, text).await;
}

pub struct SayCommand;
pub static COMMAND_DESCRIPTOR: SayCommand = SayCommand;

impl crate::commands::command_contract::CommandSpec for SayCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "say",
            category: "outils",
            params: "<message...>",
            summary: "Fait parler le bot",
            description: "Envoie un message brut dans le salon courant via le bot.",
            examples: &["+say", "+sy", "+help say"],
            default_aliases: &["sym"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
