use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::parse_user_id;
use crate::commands::common::{parse_role, send_embed, theme_color};

pub async fn handle_delrole(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.len() < 2 {
        return;
    }

    let Some(target) = parse_user_id(args[0]) else {
        return;
    };
    let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await else {
        return;
    };
    let Some(role) = parse_role(&guild, args[1]) else {
        return;
    };

    let done = if let Ok(member) = guild_id.member(&ctx.http, target).await {
        member.remove_role(&ctx.http, role.id).await.is_ok()
    } else {
        false
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("DelRole")
            .description(if done {
                format!("Role <@&{}> retire a <@{}>.", role.id.get(), target.get())
            } else {
                "Echec de modification du role.".to_string()
            })
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct DelroleCommand;
pub static COMMAND_DESCRIPTOR: DelroleCommand = DelroleCommand;

impl crate::commands::command_contract::CommandSpec for DelroleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "delrole",
            category: "roles",
            params: "<@membre/ID[,..]> <@role/ID>",
            description: "Retire un role a un ou plusieurs membres.",
            examples: &["+delrole @User @Membre"],
            default_aliases: &["dr"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
