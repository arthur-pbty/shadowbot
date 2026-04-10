use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::{app_owner_id, ensure_owner};
use crate::commands::common::{add_list_fields, send_embed, theme_color};
use crate::db::{DbPoolKey, list_bot_owners};

pub async fn handle_owner(ctx: &Context, msg: &Message, _args: &[&str]) {
    if ensure_owner(ctx, msg).await.is_err() {
        return;
    }

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    let mut lines: Vec<String> = Vec::new();

    if let Some(app_owner) = app_owner_id(ctx).await {
        lines.push(format!("<@{}> (owner application)", app_owner.get()));
    }

    if let Some(pool) = pool {
        if let Ok(extra) = list_bot_owners(&pool, bot_id).await {
            for uid in extra {
                lines.push(format!("<@{}>", uid));
            }
        }
    }

    let color = theme_color(ctx).await;
    let mut embed = serenity::builder::CreateEmbed::new()
        .title("Owners du bot")
        .color(color);
    embed = add_list_fields(embed, &lines, "Owners");

    send_embed(ctx, msg, embed).await;
}

pub struct OwnerCommand;
pub static COMMAND_DESCRIPTOR: OwnerCommand = OwnerCommand;

impl crate::commands::command_contract::CommandSpec for OwnerCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "owner",
            category: "administration",
            params: "aucun",
            description: "Affiche l owner application et les owners ajoutes en base.",
            examples: &["+owner", "+or", "+help owner"],
            default_aliases: &["own"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
