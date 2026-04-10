use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{add_list_fields, send_embed};
use crate::db::{
    DbPoolKey, get_command_alias, list_command_aliases, remove_command_alias, set_command_alias,
};
use crate::permissions::all_command_keys;

pub async fn handle_alias(ctx: &Context, msg: &Message, args: &[&str]) {
    let bot_id = ctx.cache.current_user().id;
    let Some(pool) = pool(ctx).await else {
        let embed = serenity::builder::CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if args.len() == 1 {
        let aliases = list_command_aliases(&pool, bot_id)
            .await
            .unwrap_or_default();
        let lines = aliases
            .into_iter()
            .map(|(alias, command)| format!("`{}` -> `{}`", alias, command))
            .collect::<Vec<_>>();

        let mut embed = serenity::builder::CreateEmbed::new()
            .title("Aliases")
            .color(0x5865F2);
        embed = add_list_fields(embed, &lines, "Aliases enregistrés");
        send_embed(ctx, msg, embed).await;
        return;
    }

    if args.len() < 2 {
        let embed = serenity::builder::CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+alias <commande> <alias>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    if args[0].eq_ignore_ascii_case("remove") || args[0].eq_ignore_ascii_case("delete") {
        let alias_name = args[1].trim_start_matches('+').to_lowercase();
        if alias_name.is_empty() {
            let embed = serenity::builder::CreateEmbed::new()
                .title("Erreur")
                .description("Alias invalide.")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        }

        let removed = remove_command_alias(&pool, bot_id, &alias_name)
            .await
            .unwrap_or(0);
        let embed = serenity::builder::CreateEmbed::new()
            .title("Alias supprimé")
            .description(format!("`{}` : {} suppression(s).", alias_name, removed))
            .color(0x57F287);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let command = args[0].trim_start_matches('+').to_lowercase();
    if !all_command_keys()
        .iter()
        .any(|candidate| candidate == &command)
    {
        let embed = serenity::builder::CreateEmbed::new()
            .title("Erreur")
            .description("Commande cible inconnue.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let alias_name = args[1].trim_start_matches('+').to_lowercase();
    if alias_name.is_empty() {
        let embed = serenity::builder::CreateEmbed::new()
            .title("Erreur")
            .description("Alias invalide.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let _ = set_command_alias(&pool, bot_id, &alias_name, &command).await;
    let embed = serenity::builder::CreateEmbed::new()
        .title("Alias créé")
        .description(format!(
            "`{}` devient un alias de `{}`",
            alias_name, command
        ))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<DbPoolKey>().cloned()
}

pub async fn resolve_alias(ctx: &Context, command: &str) -> Option<String> {
    let bot_id = ctx.cache.current_user().id;
    let pool = pool(ctx).await?;
    get_command_alias(&pool, bot_id, command)
        .await
        .ok()
        .flatten()
}

pub async fn resolve_command_alias_name(ctx: &Context, command: &str) -> Option<String> {
    resolve_alias(ctx, command).await
}
pub struct AliasCommand;
pub static COMMAND_DESCRIPTOR: AliasCommand = AliasCommand;

impl crate::commands::command_contract::CommandSpec for AliasCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "alias",
            category: "permissions",
            params: "<commande> <alias> | remove <alias> | list",
            summary: "Gere les aliases personnalises",
            description: "Liste, ajoute ou supprime des aliases de commandes stockes en base.",
            examples: &["+alias", "+as", "+help alias"],
            default_aliases: &["als"],
            default_permission: 9,
        }
    }
}
