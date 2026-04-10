use serenity::builder::CreateEmbed;
use serenity::model::guild::Role;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;

use crate::commands::common::{
    add_list_fields, discord_ts, has_flag, mention_user, parse_limit, send_embed,
};

pub async fn handle_botadmins(ctx: &Context, msg: &Message, args: &[&str]) {
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

    let mut admin_bots = members
        .iter()
        .filter(|member| {
            member.user.bot
                && has_admin_permission(member, partial_guild.owner_id, &partial_guild.roles)
        })
        .collect::<Vec<_>>();

    admin_bots.sort_by_key(|member| member.user.name.to_lowercase());

    if admin_bots.is_empty() {
        let embed = CreateEmbed::new()
            .title("Bots administrateurs")
            .description("Aucun bot administrateur trouvé.")
            .color(0xFEE75C);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let visible = admin_bots.iter().take(limit).collect::<Vec<_>>();
    let lines = visible
        .iter()
        .map(|member| {
            if detailed {
                format!(
                    "- {} | ID: {} | Roles: {} | Créé: {}",
                    mention_user(member.user.id),
                    member.user.id,
                    member.roles.len(),
                    discord_ts(member.user.created_at(), "F")
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

    let total_bots = members.iter().filter(|member| member.user.bot).count();
    let ratio = if total_bots == 0 {
        0.0
    } else {
        (admin_bots.len() as f64 / total_bots as f64) * 100.0
    };

    let mut embed = CreateEmbed::new()
        .title("Bots administrateurs")
        .description(format!(
            "Serveur: **{}**\nBots admin: **{}** / Bots totaux: **{}** ({:.1}%)",
            partial_guild.name,
            admin_bots.len(),
            total_bots,
            ratio
        ))
        .color(0x5865F2);

    embed = add_list_fields(
        embed,
        &lines,
        &format!(
            "Liste ({} affichés / {} total)",
            visible.len(),
            admin_bots.len()
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

pub struct BotadminsCommand;
pub static COMMAND_DESCRIPTOR: BotadminsCommand = BotadminsCommand;

impl crate::commands::command_contract::CommandSpec for BotadminsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "botadmins",
            category: "owner",
            params: "aucun",
            description: "Affiche les utilisateurs ayant des droits admin sur le bot.",
            examples: &["+botadmins", "+bs", "+help botadmins"],
            default_aliases: &["bad"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
