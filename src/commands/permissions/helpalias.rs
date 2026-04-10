use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::db::{DbPoolKey, get_help_aliases_enabled, set_help_aliases_enabled};

pub async fn handle_helpalias(ctx: &Context, msg: &Message, args: &[&str]) {
    let bot_id = ctx.cache.current_user().id;
    let Some(pool) = pool(ctx).await else {
        let embed = serenity::builder::CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if args.is_empty() {
        let enabled = get_help_aliases_enabled(&pool, bot_id)
            .await
            .ok()
            .flatten()
            .unwrap_or(true);

        let embed = serenity::builder::CreateEmbed::new()
            .title("Aliases dans help")
            .description(format!(
                "État actuel: `{}`",
                if enabled { "on" } else { "off" }
            ))
            .color(0x5865F2);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let enabled = match args[0].to_lowercase().as_str() {
        "on" | "true" | "yes" => true,
        "off" | "false" | "no" => false,
        _ => {
            let embed = serenity::builder::CreateEmbed::new()
                .title("Erreur")
                .description("Usage: `+helpalias <on/off>`")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        }
    };

    let _ = set_help_aliases_enabled(&pool, bot_id, enabled).await;
    let embed = serenity::builder::CreateEmbed::new()
        .title("Help aliases mis à jour")
        .description(format!(
            "Aliases dans l'aide: `{}`",
            if enabled { "on" } else { "off" }
        ))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}
pub struct HelpaliasCommand;
pub static COMMAND_DESCRIPTOR: HelpaliasCommand = HelpaliasCommand;

impl crate::commands::command_contract::CommandSpec for HelpaliasCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "helpalias",
            category: "permissions",
            params: "<on|off>",
            summary: "Active ou coupe les aliases help",
            description: "Active ou desactive laffichage des aliases dans laide.",
            examples: &["+helpalias", "+hs", "+help helpalias"],
            default_aliases: &["hal"],
            default_permission: 0,
        }
    }
}
