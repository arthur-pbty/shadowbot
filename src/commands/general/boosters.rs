use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{
    add_list_fields, discord_ts, has_flag, mention_user, parse_limit, send_embed,
};

pub async fn handle_boosters(ctx: &Context, msg: &Message, args: &[&str]) {
    let limit = parse_limit(args, 25, 100);
    let detailed = has_flag(args, &["--details", "-d", "full"]);
    let recent_first = has_flag(args, &["--recent", "recent", "-r"]);

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

    let mut boosters = members
        .iter()
        .filter(|member| member.premium_since.is_some())
        .collect::<Vec<_>>();

    if recent_first {
        boosters.sort_by(|left, right| right.premium_since.cmp(&left.premium_since));
    } else {
        boosters.sort_by_key(|member| member.user.name.to_lowercase());
    }

    if boosters.is_empty() {
        let embed = CreateEmbed::new()
            .title("Boosters du serveur")
            .description("Aucun booster trouvé sur ce serveur.")
            .color(0xFEE75C);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let visible = boosters.iter().take(limit).collect::<Vec<_>>();
    let lines = visible
        .iter()
        .map(|member| {
            if detailed {
                format!(
                    "- {} | ID: {} | Boost depuis: {}",
                    mention_user(member.user.id),
                    member.user.id,
                    member
                        .premium_since
                        .map(|value| discord_ts(value, "F"))
                        .unwrap_or_else(|| "Inconnu".to_string())
                )
            } else {
                format!("- {}", mention_user(member.user.id))
            }
        })
        .collect::<Vec<_>>();

    let ratio = (boosters.len() as f64 / members.len() as f64) * 100.0;
    let mut embed = CreateEmbed::new()
        .title("Membres boostant le serveur")
        .description(format!(
            "Serveur: **{}**\nBoosters: **{}** / Membres: **{}** ({:.1}%)",
            guild.name,
            boosters.len(),
            members.len(),
            ratio
        ))
        .color(0x5865F2);

    if let Some(last_boost) = boosters
        .iter()
        .filter_map(|member| {
            member
                .premium_since
                .map(|since| (mention_user(member.user.id), since))
        })
        .max_by_key(|(_, since)| *since)
    {
        embed = embed.field(
            "Dernier boost",
            format!("{} ({})", last_boost.0, discord_ts(last_boost.1, "F")),
            false,
        );
    }

    embed = add_list_fields(
        embed,
        &lines,
        &format!(
            "Liste ({} affichés / {} total)",
            visible.len(),
            boosters.len()
        ),
    );

    send_embed(ctx, msg, embed).await;
}

pub struct BoostersCommand;
pub static COMMAND_DESCRIPTOR: BoostersCommand = BoostersCommand;

impl crate::commands::command_contract::CommandSpec for BoostersCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "boosters",
            category: "general",
            params: "aucun",
            summary: "Liste les boosters du serveur",
            description: "Affiche les membres qui boostent actuellement le serveur.",
            examples: &["+boosters", "+bs", "+help boosters"],
            default_aliases: &["bst"],
            default_permission: 0,
        }
    }
}
