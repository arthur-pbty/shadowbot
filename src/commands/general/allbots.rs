use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{
    add_list_fields, discord_ts, has_flag, mention_user, parse_limit, send_embed,
};

pub async fn handle_allbots(ctx: &Context, msg: &Message, args: &[&str]) {
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

    let guild = match guild_id.to_partial_guild(&ctx.http).await {
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

    let mut bots = members
        .iter()
        .filter(|member| member.user.bot)
        .collect::<Vec<_>>();

    bots.sort_by_key(|member| member.user.name.to_lowercase());

    if bots.is_empty() {
        let embed = CreateEmbed::new()
            .title("Bots du serveur")
            .description("Aucun bot trouvé sur ce serveur.")
            .color(0xFEE75C);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let visible = bots.iter().take(limit).collect::<Vec<_>>();
    let lines = visible
        .iter()
        .map(|member| {
            if detailed {
                format!(
                    "- {} | ID: {} | Créé: {}",
                    mention_user(member.user.id),
                    member.user.id,
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

    let bot_ratio = (bots.len() as f64 / members.len() as f64) * 100.0;
    let mut embed = CreateEmbed::new()
        .title("Bots présents sur le serveur")
        .description(format!(
            "Serveur: **{}**\nBots: **{}** / Membres: **{}** ({:.1}%)",
            guild.name,
            bots.len(),
            members.len(),
            bot_ratio
        ))
        .color(0x5865F2);

    if let Some(newest_bot) = bots.iter().max_by_key(|member| member.user.created_at()) {
        embed = embed.field(
            "Bot le plus récent",
            format!(
                "{} ({})",
                mention_user(newest_bot.user.id),
                discord_ts(newest_bot.user.created_at(), "F")
            ),
            false,
        );
    }

    embed = add_list_fields(
        embed,
        &lines,
        &format!("Liste ({} affichés / {} total)", visible.len(), bots.len()),
    );

    send_embed(ctx, msg, embed).await;
}

pub struct AllbotsCommand;
pub static COMMAND_DESCRIPTOR: AllbotsCommand = AllbotsCommand;

impl crate::commands::command_contract::CommandSpec for AllbotsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "allbots",
            command: "allbots",
            category: "general",
            params: "aucun",
            summary: "Liste tous les bots du serveur",
            description: "Affiche la liste des membres bots presents sur le serveur courant.",
            examples: &["+allbots", "+as", "+help allbots"],
            alias_source_key: "allbots",
            default_aliases: &["abt"],
            default_permission: 0,
        }
    }
}
