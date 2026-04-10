use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::parse_role;
use crate::commands::common::{discord_ts, send_embed};

pub async fn handle_role(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Ok(guild) = guild_id.to_partial_guild(&ctx).await else {
        return;
    };

    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+role <rôle>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let role_query = args.join(" ");

    let Some(role) = parse_role(&guild, &role_query) else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description(format!("Rôle '{}' non trouvé.", role_query))
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let members_with_role = guild_id.to_partial_guild(ctx).await.ok().and_then(|g| {
        Some(
            g.roles
                .values()
                .find(|r| r.id == role.id)
                .map(|_| g.clone())?,
        )
    });

    let member_count = if let Some(_g) = members_with_role {
        guild_id
            .members(ctx, None, None)
            .await
            .ok()
            .map(|m| {
                m.iter()
                    .filter(|member| member.roles.contains(&role.id))
                    .count()
            })
            .unwrap_or(0)
    } else {
        0
    };

    let created_at = discord_ts(role.id.created_at(), "F");

    let embed = CreateEmbed::new()
        .title(&role.name)
        .description(format!("ID: `{}`", role.id.get()))
        .color(role.colour.0)
        .field("Créé", created_at, true)
        .field("Membres", format!("{}", member_count), true)
        .field("Position", format!("{}", role.position), true)
        .field(
            "Géré par bot",
            if role.managed { "Oui" } else { "Non" },
            true,
        )
        .field(
            "Mentionnable",
            if role.mentionable { "Oui" } else { "Non" },
            true,
        )
        .field(
            "Affichable séparément",
            if role.hoist { "Oui" } else { "Non" },
            true,
        );

    send_embed(ctx, msg, embed).await;
}

pub struct RoleCommand;
pub static COMMAND_DESCRIPTOR: RoleCommand = RoleCommand;

impl crate::commands::command_contract::CommandSpec for RoleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "role",
            command: "role",
            category: "general",
            params: "<@&rôle/ID>",
            summary: "Affiche les details dun role",
            description: "Affiche les informations utiles dun role cible.",
            examples: &["+role", "+re", "+help role"],
            alias_source_key: "role",
            default_aliases: &["rol"],
            default_permission: 0,
        }
    }
}
