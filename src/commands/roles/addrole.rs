use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::admin_common::parse_user_id;
use crate::commands::common::{parse_role, send_embed, theme_color};

pub async fn handle_addrole(ctx: &Context, msg: &Message, args: &[&str]) {
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
        member.add_role(&ctx.http, role.id).await.is_ok()
    } else {
        false
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("AddRole")
            .description(if done {
                format!("Role <@&{}> ajoute a <@{}>.", role.id.get(), target.get())
            } else {
                "Echec de modification du role.".to_string()
            })
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct AddroleCommand;
pub static COMMAND_DESCRIPTOR: AddroleCommand = AddroleCommand;

impl crate::commands::command_contract::CommandSpec for AddroleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "addrole",
            category: "roles",
            params: "<@membre/ID[,..]> <@role/ID>",
            description: "Ajoute un role a un ou plusieurs membres.",
            examples: &["+addrole @User @Membre"],
            default_aliases: &["ar"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
