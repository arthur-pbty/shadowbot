use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::automod_service::{parse_on_off, pool};
use crate::commands::common::{add_list_fields, send_embed};
use crate::db;

pub async fn handle_badwords(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id.get() as i64;

    if args.is_empty() {
        let Ok(settings) =
            db::get_or_create_moderation_settings(&pool, bot_id, guild_id.get() as i64).await
        else {
            return;
        };

        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("BadWords")
                .description(format!(
                    "Etat: **{}**\nUsage: +badwords <on/off|add/del/list>",
                    if settings.badwords_enabled {
                        "ON"
                    } else {
                        "OFF"
                    }
                ))
                .color(0x5865F2),
        )
        .await;
        return;
    }

    let action = args[0].to_lowercase();

    if action == "list" {
        let words = db::list_badwords(&pool, bot_id, guild_id.get() as i64)
            .await
            .unwrap_or_default();
        let lines = words
            .into_iter()
            .map(|word| format!("- {}", word))
            .collect::<Vec<_>>();

        let mut embed = CreateEmbed::new().title("BadWords list").color(0x5865F2);
        embed = add_list_fields(embed, &lines, "Mots interdits");
        send_embed(ctx, msg, embed).await;
        return;
    }

    if let Some(value) = parse_on_off(&action) {
        let _ = db::set_badwords_enabled(&pool, bot_id, guild_id.get() as i64, value).await;
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("BadWords")
                .description(format!(
                    "Protection badwords: **{}**",
                    if value { "ON" } else { "OFF" }
                ))
                .color(0x57F287),
        )
        .await;
        return;
    }

    if action == "add" {
        let Some(word) = args.get(1) else {
            return;
        };
        let _ = db::add_badword(&pool, bot_id, guild_id.get() as i64, word).await;
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("BadWords")
                .description(format!("Mot ajoute: **{}**", word))
                .color(0x57F287),
        )
        .await;
        return;
    }

    if action == "del" || action == "remove" {
        let Some(word) = args.get(1) else {
            return;
        };
        let removed = db::remove_badword(&pool, bot_id, guild_id.get() as i64, word)
            .await
            .unwrap_or(0);
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("BadWords")
                .description(format!("Mot supprime: **{}** ({}).", word, removed))
                .color(0x57F287),
        )
        .await;
        return;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("BadWords")
            .description("Usage: +badwords <on/off|add <mot>|del <mot>|list>")
            .color(0xED4245),
    )
    .await;
}

pub struct BadwordsCommand;
pub static COMMAND_DESCRIPTOR: BadwordsCommand = BadwordsCommand;

impl crate::commands::command_contract::CommandSpec for BadwordsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "badwords",
            category: "security",
            params: "<on/off|add <mot>|del <mot>|list>",
            description: "Active la protection badwords et gere la liste des mots interdits.",
            examples: &["+badwords on", "+badwords add insulte", "+badwords list"],
            default_aliases: &["bw"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
