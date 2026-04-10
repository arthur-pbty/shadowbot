use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::commands::perms_helpers::{ensure_owner, get_pool};
use crate::db::clear_role_permissions;

pub async fn handle_clear_perms(ctx: &Context, msg: &Message) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    let bot_id = ctx.cache.current_user().id;
    let Some(pool) = get_pool(ctx).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let removed = clear_role_permissions(&pool, bot_id).await.unwrap_or(0);
    let embed = CreateEmbed::new()
        .title("Permissions roles supprimees")
        .description(format!("{} entree(s) supprimee(s).", removed))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub struct ClearPermsCommand;
pub static COMMAND_DESCRIPTOR: ClearPermsCommand = ClearPermsCommand;

impl crate::commands::command_contract::CommandSpec for ClearPermsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "clear_perms",
            category: "perms",
            params: "aucun",
            description: "Supprime toutes les permissions ACL configurees en base.",
            examples: &["+clear perms", "+cs", "+help clear perms"],
            default_aliases: &["cpm"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
