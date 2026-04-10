use serenity::builder::CreateEmbed;
use serenity::model::guild::Role;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;

use crate::commands::common::{add_list_fields, has_flag, mention_user, parse_limit, send_embed};

pub async fn handle_alladmins(ctx: &Context, msg: &Message, args: &[&str]) {
    let limit = parse_limit(args, 25, 100);
    let detailed = has_flag(args, &["--details", "-d", "full"]);

    let Some(guild_id) = msg.guild_id else {
        let embed = CreateEmbed::new()
            .title("Commande invalide")
            .description("Cette commande doit être utilisée dans un serveur.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let partial_guild = match guild_id.to_partial_guild(&ctx.http).await {
        Ok(guild) => guild,
        Err(why) => {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description(format!("Impossible de récupérer le serveur: {why}"))
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        }
    };

    let members = match guild_id.members(&ctx.http, None, None).await {
        Ok(members) => members,
        Err(why) => {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description(format!("Impossible de récupérer les membres: {why}"))
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        }
    };

    let mut admin_members = members
        .iter()
        .filter(|member| {
            !member.user.bot
                && has_admin_permission(member, partial_guild.owner_id, &partial_guild.roles)
        })
        .collect::<Vec<_>>();

    admin_members.sort_by_key(|member| member.user.name.to_lowercase());

    if admin_members.is_empty() {
        let embed = CreateEmbed::new()
            .title("Admins (hors bots)")
            .description("Aucun administrateur (hors bots) trouvé.")
            .color(0xFEE75C);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let visible = admin_members.iter().take(limit).collect::<Vec<_>>();
    let lines = visible
        .iter()
        .map(|member| {
            if detailed {
                let top_role = member
                    .roles
                    .iter()
                    .filter_map(|role_id| partial_guild.roles.get(role_id))
                    .max_by_key(|role| role.position)
                    .map(|role| role.name.clone())
                    .unwrap_or_else(|| "Aucun".to_string());

                format!(
                    "- {} | ID: {} | Roles: {} | Top: {}",
                    mention_user(member.user.id),
                    member.user.id,
                    member.roles.len(),
                    top_role
                )
            } else {
                format!(
                    "- {} (ID: {})",
                    mention_user(member.user.id),
                    member.user.id
                )
            }
        })
        .collect::<Vec<_>>();

    let ratio = (admin_members.len() as f64 / members.len() as f64) * 100.0;
    let mut embed = CreateEmbed::new()
        .title("Admins (hors bots)")
        .description(format!(
            "Serveur: **{}**\nAdmins humains: **{}** / Membres: **{}** ({:.1}%)",
            partial_guild.name,
            admin_members.len(),
            members.len(),
            ratio
        ))
        .color(0x5865F2)
        .field("Owner", partial_guild.owner_id.mention().to_string(), true);

    embed = add_list_fields(
        embed,
        &lines,
        &format!(
            "Liste ({} affichés / {} total)",
            visible.len(),
            admin_members.len()
        ),
    );

    send_embed(ctx, msg, embed).await;
}

fn has_admin_permission(member: &Member, owner_id: UserId, roles: &HashMap<RoleId, Role>) -> bool {
    if member.user.id == owner_id {
        return true;
    }

    member.roles.iter().any(|role_id| {
        roles
            .get(role_id)
            .is_some_and(|role| role.permissions.contains(Permissions::ADMINISTRATOR))
    })
}

pub struct AlladminsCommand;
pub static COMMAND_DESCRIPTOR: AlladminsCommand = AlladminsCommand;

impl crate::commands::command_contract::CommandSpec for AlladminsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "alladmins",
            category: "general",
            params: "aucun",
            summary: "Liste les administrateurs du serveur",
            description: "Affiche les membres qui possedent des droits administrateur sur le serveur.",
            examples: &["+alladmins", "+as", "+help alladmins"],
            default_aliases: &["aad"],
            default_permission: 0,
        }
    }
}
