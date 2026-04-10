use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::parse_user_id;
use crate::commands::common::{parse_role, send_embed, theme_color};
use crate::db::DbPoolKey;

pub async fn handle_untemprole(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    if args.len() < 2 {
        return;
    }

    let Some(user_id) = parse_user_id(args[0]) else {
        return;
    };

    let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await else {
        return;
    };

    let Some(role) = parse_role(&guild, args[1]) else {
        return;
    };

    if let Ok(member) = guild_id.member(&ctx.http, user_id).await {
        let _ = member.remove_role(&ctx.http, role.id).await;
    }

    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };
    if let Some(pool) = pool {
        let bot_id = ctx.cache.current_user().id;
        let _ = sqlx::query(
            r#"
            UPDATE bot_temproles
            SET active = FALSE
            WHERE bot_id = $1 AND guild_id = $2 AND user_id = $3 AND role_id = $4;
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(guild_id.get() as i64)
        .bind(user_id.get() as i64)
        .bind(role.id.get() as i64)
        .execute(&pool)
        .await;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("UnTempRole")
            .description(format!(
                "Rôle <@&{}> retiré à <@{}>.",
                role.id.get(),
                user_id.get()
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct UnTempRoleCommand;
pub static COMMAND_DESCRIPTOR: UnTempRoleCommand = UnTempRoleCommand;

impl crate::commands::command_contract::CommandSpec for UnTempRoleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "untemprole",
            category: "roles",
            params: "<membre> <role>",
            summary: "Retire un role temporaire",
            description: "Retire immediatement un role temporaire et desactive son expiration.",
            examples: &["+untemprole @User @VIP"],
            default_aliases: &["untrole", "deltrole"],
            default_permission: 8,
        }
    }
}
