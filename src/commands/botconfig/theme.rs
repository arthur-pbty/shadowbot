use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::botconfig_common::parse_color;
use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, set_bot_theme};

pub async fn handle_theme(ctx: &Context, msg: &Message, args: &[&str]) {
    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+theme <couleur>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let value = args.join(" ");
    let Some(color) = parse_color(&value) else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Couleur invalide. Ex: `#5865F2`, `bleu`, `0xFFAA00`.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    if let Some(pool) = pool {
        let _ = set_bot_theme(&pool, bot_id, color).await;
    }

    let embed = CreateEmbed::new()
        .title("Thème mis à jour")
        .description(format!("Nouvelle couleur: `#{:06X}`", color))
        .color(color);

    send_embed(ctx, msg, embed).await;
}

pub struct ThemeCommand;
pub static COMMAND_DESCRIPTOR: ThemeCommand = ThemeCommand;

impl crate::commands::command_contract::CommandSpec for ThemeCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "theme",
            category: "botconfig",
            params: "<couleur|#hex|0xhex>",
            description: "Met a jour la couleur principale des embeds du bot.",
            examples: &["+theme", "+te", "+help theme"],
            default_aliases: &["thm"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
