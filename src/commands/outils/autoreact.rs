use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed, CreateMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_channel_id, send_embed, theme_color};
use crate::db::DbPoolKey;

fn owned_component_id(action: &str, owner_id: UserId) -> String {
    format!("{}:{}", action, owner_id.get())
}

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

pub async fn handle_autoreact(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(action) = args.first().map(|s| s.to_lowercase()) else {
        let embed = CreateEmbed::new()
            .title("AutoReact")
            .description("Utilise les boutons pour ajouter/supprimer/lister via UI.")
            .color(theme_color(ctx).await);
        let components = vec![CreateActionRow::Buttons(vec![
            CreateButton::new(owned_component_id("adv:autoreact:add_modal", msg.author.id))
                .label("Ajouter")
                .style(ButtonStyle::Success),
            CreateButton::new(owned_component_id("adv:autoreact:del_modal", msg.author.id))
                .label("Supprimer")
                .style(ButtonStyle::Danger),
            CreateButton::new(owned_component_id("adv:autoreact:list", msg.author.id))
                .label("Lister")
                .style(ButtonStyle::Primary),
        ])];

        let _ = msg
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().embed(embed).components(components),
            )
            .await;
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id;

    if action == "list" {
        let rows = sqlx::query_as::<_, (i64, String)>(
            r#"
            SELECT channel_id, emoji
            FROM bot_autoreacts
            WHERE bot_id = $1 AND guild_id = $2
            ORDER BY channel_id ASC, emoji ASC;
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let desc = if rows.is_empty() {
            "Aucun autoreact configure.".to_string()
        } else {
            rows.into_iter()
                .map(|(channel_id, emoji)| format!("- <#{}> -> {}", channel_id, emoji))
                .collect::<Vec<_>>()
                .join("\n")
        };

        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("AutoReact")
                .description(desc)
                .color(theme_color(ctx).await),
        )
        .await;
        return;
    }

    if args.len() < 3 {
        return;
    }

    let Some(channel_id) = parse_channel_id(args[1]) else {
        return;
    };
    let emoji = args[2];

    if action == "add" {
        let _ = sqlx::query(
            r#"
            INSERT INTO bot_autoreacts (bot_id, guild_id, channel_id, emoji)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (bot_id, guild_id, channel_id, emoji) DO NOTHING;
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(channel_id.get() as i64)
        .bind(emoji)
        .execute(&pool)
        .await;
    } else if action == "del" {
        let _ = sqlx::query(
            r#"
            DELETE FROM bot_autoreacts
            WHERE bot_id = $1 AND guild_id = $2 AND channel_id = $3 AND emoji = $4;
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(channel_id.get() as i64)
        .bind(emoji)
        .execute(&pool)
        .await;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("AutoReact")
            .description("Configuration mise a jour.")
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct AutoReactCommand;
pub static COMMAND_DESCRIPTOR: AutoReactCommand = AutoReactCommand;

impl crate::commands::command_contract::CommandSpec for AutoReactCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "autoreact",
            category: "outils",
            params: "<add/del> <salon> <emoji> | list",
            summary: "Configure les reactions automatiques",
            description: "Ajoute, retire et liste les reactions automatiquement appliquees aux messages d'un salon.",
            examples: &["+autoreact add #general 😀", "+autoreact list"],
            default_aliases: &["ar", "reactauto"],
            default_permission: 8,
        }
    }
}
