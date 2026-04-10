use serenity::builder::{CreateEmbed, GetMessages};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::parse_user_id;
use crate::commands::common::{send_embed, theme_color};
use crate::db::{self, DbPoolKey};

pub async fn handle_clear_messages(ctx: &Context, msg: &Message, args: &[&str]) {
    let Ok(mut amount) = args.first().unwrap_or(&"0").parse::<u64>() else {
        return;
    };
    if amount == 0 {
        return;
    }

    let max_limit = if let Some(guild_id) = msg.guild_id {
        let pool = {
            let data = ctx.data.read().await;
            data.get::<DbPoolKey>().cloned()
        };

        if let Some(pool) = pool {
            let bot_id = ctx.cache.current_user().id.get() as i64;
            db::get_or_create_moderation_settings(&pool, bot_id, guild_id.get() as i64)
                .await
                .ok()
                .map(|settings| settings.clear_limit.max(1) as u64)
                .unwrap_or(100)
        } else {
            100
        }
    } else {
        100
    };

    amount = amount.clamp(1, max_limit);

    let filter_user = args.get(1).and_then(|raw| parse_user_id(raw));

    let mut deleted = 0usize;
    if let Ok(messages) = msg
        .channel_id
        .messages(&ctx.http, GetMessages::new().limit(amount as u8 + 1))
        .await
    {
        for m in messages {
            if m.id == msg.id {
                continue;
            }
            if let Some(user_id) = filter_user {
                if m.author.id != user_id {
                    continue;
                }
            }
            if msg.channel_id.delete_message(&ctx.http, m.id).await.is_ok() {
                deleted += 1;
            }
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Clear")
            .description(format!("{} message(s) supprime(s).", deleted))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct ClearMessagesCommand;
pub static COMMAND_DESCRIPTOR: ClearMessagesCommand = ClearMessagesCommand;

impl crate::commands::command_contract::CommandSpec for ClearMessagesCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "clear_messages",
            category: "moderation",
            params: "<nombre> [@membre/ID]",
            description: "Supprime un nombre de messages, optionnellement filtres par membre.",
            examples: &["+clear 20", "+clear 20 @User"],
            default_aliases: &["purge"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
