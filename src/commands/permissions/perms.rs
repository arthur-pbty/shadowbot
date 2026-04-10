use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{add_list_fields, send_embed, truncate_text};
use crate::commands::perms_helpers::{ensure_owner, get_pool};
use crate::db::{list_role_command_access, list_role_perm_levels, list_role_scopes};

pub async fn handle_perms(ctx: &Context, msg: &Message, args: &[&str]) {
    let _ = args;

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

    let roles = list_role_scopes(&pool, bot_id).await.unwrap_or_default();
    let mut lines = Vec::new();

    for rid in roles {
        let perm_levels = list_role_perm_levels(&pool, bot_id, rid as u64)
            .await
            .unwrap_or_default();
        let command_access = list_role_command_access(&pool, bot_id, rid as u64)
            .await
            .unwrap_or_default();

        let perms = if perm_levels.is_empty() {
            "aucun".to_string()
        } else {
            perm_levels
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(",")
        };

        let commands = if command_access.is_empty() {
            "aucune".to_string()
        } else {
            truncate_text(&command_access.join(", "), 80)
        };

        lines.push(format!(
            "<@&{}> · perms [{}] · cmd [{}]",
            rid, perms, commands
        ));
    }

    let mut embed = CreateEmbed::new().title("Permissions du bot");
    embed = add_list_fields(embed, &lines, "Roles configures");
    send_embed(ctx, msg, embed).await;
}

pub struct PermsCommand;
pub static COMMAND_DESCRIPTOR: PermsCommand = PermsCommand;

impl crate::commands::command_contract::CommandSpec for PermsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "perms",
            category: "permissions",
            params: "aucun",
            description: "Affiche les permissions ACL configurees par role ou scope.",
            examples: &["+perms", "+ps", "+help perms"],
            default_aliases: &["prm"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
