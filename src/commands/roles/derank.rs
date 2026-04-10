use std::collections::HashSet;

use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::automod_service::pool;
use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_sanction_helpers::parse_targets;
use crate::db;

pub async fn handle_derank(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.is_empty() {
        return;
    }

    let targets = parse_targets(args[0]).await;
    if targets.is_empty() {
        return;
    }

    let protected_roles: HashSet<u64> = if let Some(pool) = pool(ctx).await {
        let bot_id = ctx.cache.current_user().id.get() as i64;
        db::list_noderank_roles(&pool, bot_id, guild_id.get() as i64)
            .await
            .unwrap_or_default()
            .into_iter()
            .filter_map(|id| u64::try_from(id).ok())
            .collect()
    } else {
        HashSet::new()
    };

    let mut done = 0usize;
    for uid in &targets {
        if let Ok(member) = guild_id.member(&ctx.http, *uid).await {
            let roles = member.roles.clone();
            let mut ok = true;
            for role_id in roles {
                if protected_roles.contains(&role_id.get()) {
                    continue;
                }
                if member.remove_role(&ctx.http, role_id).await.is_err() {
                    ok = false;
                }
            }
            if ok {
                done += 1;
            }
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Derank")
            .description(format!("{} membre(s) derank.", done))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct DerankCommand;
pub static COMMAND_DESCRIPTOR: DerankCommand = DerankCommand;

impl crate::commands::command_contract::CommandSpec for DerankCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "derank",
            category: "roles",
            params: "<@membre/ID[,..]>",
            description: "Retire tous les roles gerables d un membre.",
            examples: &["+derank @User"],
            default_aliases: &["drk"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
