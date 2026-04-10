use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_role, send_embed, theme_color};

pub async fn handle_unmassiverole(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    if args.is_empty() {
        return;
    }

    let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await else {
        return;
    };

    let Some(target_role) = parse_role(&guild, args[0]) else {
        return;
    };

    let filter_role = args.get(1).and_then(|raw| parse_role(&guild, raw));

    let Ok(members) = guild_id.members(&ctx.http, None, None).await else {
        return;
    };

    let mut affected = 0usize;
    for member in members {
        if let Some(filter) = &filter_role {
            if !member.roles.contains(&filter.id) {
                continue;
            }
        }

        if member.remove_role(&ctx.http, target_role.id).await.is_ok() {
            affected += 1;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("UnMassiveRole")
            .description(format!(
                "{} membres traités pour le rôle <@&{}>.",
                affected,
                target_role.id.get()
            ))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct UnMassiveRoleCommand;
pub static COMMAND_DESCRIPTOR: UnMassiveRoleCommand = UnMassiveRoleCommand;

impl crate::commands::command_contract::CommandSpec for UnMassiveRoleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "unmassiverole",
            category: "roles",
            params: "<role_cible> [role_filtre]",
            description: "Retire un role a tous les membres ou a ceux qui ont un role filtre.",
            examples: &["+unmassiverole @VIP", "+unmassiverole @VIP @Membres"],
            default_aliases: &["umrole", "umr"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
