use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::commands::perms_helpers::{ensure_owner, get_pool, parse_user_or_role};
use crate::db::remove_scope_permissions;

pub async fn handle_del(ctx: &Context, msg: &Message, args: &[&str]) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    if args.len() < 2 || !args[0].eq_ignore_ascii_case("perm") {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `del perm <role>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let Some((scope_type, scope_id)) = parse_user_or_role(args[1]) else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Role/membre invalide.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let bot_id = ctx.cache.current_user().id;
    let Some(pool) = get_pool(ctx).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let removed = remove_scope_permissions(&pool, bot_id, scope_type, scope_id)
        .await
        .unwrap_or(0);

    let embed = CreateEmbed::new()
        .title("Permissions supprimees")
        .description(format!("{} entree(s) supprimee(s).", removed))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub struct DelCommand;
pub static COMMAND_DESCRIPTOR: DelCommand = DelCommand;

impl crate::commands::command_contract::CommandSpec for DelCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "del",
            category: "permissions",
            params: "perm <@&rôle/@membre/ID>",
            description: "Supprime les permissions ACL associees a un role ou utilisateur.",
            examples: &["+del", "+dl", "+help del"],
            default_aliases: &["dlp"],
            allow_in_dm: false,
            default_permission: 0,
        }
    }
}
