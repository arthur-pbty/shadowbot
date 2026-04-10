use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::parse_role;
use crate::commands::common::{add_list_fields, mention_user, send_embed};

pub async fn handle_rolemembers(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Ok(guild) = guild_id.to_partial_guild(&ctx).await else {
        return;
    };

    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+rolemembers <rôle>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let (role_query, limit) = if args.len() > 1 {
        if let Ok(parsed_limit) = args[args.len() - 1].parse::<usize>() {
            (args[..args.len() - 1].join(" "), parsed_limit.clamp(1, 100))
        } else {
            (args.join(" "), 25)
        }
    } else {
        (args[0].to_string(), 25)
    };

    let Some(role) = parse_role(&guild, &role_query) else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description(format!("Rôle '{}' non trouvé.", role_query))
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let Ok(members) = guild_id.members(ctx, None, None).await else {
        return;
    };

    let members_count = members.len();

    let role_members: Vec<Member> = members
        .into_iter()
        .filter(|member| member.roles.contains(&role.id))
        .collect();

    let displayed = role_members.iter().take(limit).collect::<Vec<_>>();

    let member_list: Vec<String> = displayed
        .iter()
        .map(|member| mention_user(member.user.id))
        .collect();

    let ratio = if !role_members.is_empty() && members_count > 0 {
        (role_members.len() as f64 / members_count as f64) * 100.0
    } else {
        0.0
    };

    let mut embed = CreateEmbed::new()
        .title(format!("Membres avec le rôle: {}", role.name))
        .description(format!(
            "**Ratio:** {:.1}% · **Total:** {}",
            ratio,
            role_members.len()
        ))
        .color(role.colour.0);

    embed = add_list_fields(embed, &member_list, "Membres");

    send_embed(ctx, msg, embed).await;
}

pub struct RolemembersCommand;
pub static COMMAND_DESCRIPTOR: RolemembersCommand = RolemembersCommand;

impl crate::commands::command_contract::CommandSpec for RolemembersCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "rolemembers",
            category: "infos",
            params: "<@&rôle/ID>",
            description: "Affiche les membres associes a un role donne.",
            examples: &["+rolemembers", "+rs", "+help rolemembers"],
            default_aliases: &["rmb"],
            allow_in_dm: false,
            default_permission: 0,
        }
    }
}
