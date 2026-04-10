use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::automod_service::pool;
use crate::commands::common::{parse_role, send_embed};
use crate::db;

pub async fn handle_noderank(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id.get() as i64;
    let guild_id_raw = guild_id.get() as i64;

    if args.is_empty() {
        let roles = db::list_noderank_roles(&pool, bot_id, guild_id_raw)
            .await
            .unwrap_or_default();
        let description = if roles.is_empty() {
            "Aucun role protege.".to_string()
        } else {
            roles
                .iter()
                .map(|role_id| format!("<@&{}>", role_id))
                .collect::<Vec<_>>()
                .join("\n")
        };

        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("NoDeRank")
                .description(description)
                .color(0x5865F2),
        )
        .await;
        return;
    }

    if args.len() < 2 {
        return;
    }

    let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await else {
        return;
    };
    let Some(role) = parse_role(&guild, args[1]) else {
        return;
    };

    if args[0].eq_ignore_ascii_case("add") {
        let _ = db::add_noderank_role(&pool, bot_id, guild_id_raw, role.id.get() as i64).await;
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("NoDeRank")
                .description(format!("Role protege ajoute: <@&{}>", role.id.get()))
                .color(0x57F287),
        )
        .await;
        return;
    }

    if args[0].eq_ignore_ascii_case("del") || args[0].eq_ignore_ascii_case("remove") {
        let _ = db::remove_noderank_role(&pool, bot_id, guild_id_raw, role.id.get() as i64).await;
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("NoDeRank")
                .description(format!("Role protege retire: <@&{}>", role.id.get()))
                .color(0x57F287),
        )
        .await;
    }
}

pub struct NoderankCommand;
pub static COMMAND_DESCRIPTOR: NoderankCommand = NoderankCommand;

impl crate::commands::command_contract::CommandSpec for NoderankCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "noderank",
            category: "roles",
            params: "[add/del <@role/ID/nom>]",
            description: "Definit des roles proteges qui ne sont pas retires par +derank.",
            examples: &["+noderank", "+noderank add @VIP", "+noderank del @VIP"],
            default_aliases: &["ndr"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
