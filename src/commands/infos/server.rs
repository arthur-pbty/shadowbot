use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::http::GuildPagination;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{add_list_fields, send_embed, theme_color};

pub async fn handle_server(ctx: &Context, msg: &Message, args: &[&str]) {
    if args
        .first()
        .map(|value| value.eq_ignore_ascii_case("list"))
        .unwrap_or(false)
    {
        handle_server_list(ctx, msg).await;
        return;
    }

    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Ok(guild) = guild_id.to_partial_guild(&ctx).await else {
        return;
    };

    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+server pic`, `+server banner` ou `+server list`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    match args[0].to_lowercase().as_str() {
        "pic" | "icon" | "avatar" => {
            let icon_url = guild.icon_url().unwrap_or_default();

            if icon_url.is_empty() {
                let embed = CreateEmbed::new()
                    .title("Erreur")
                    .description("Ce serveur n'a pas d'icône.")
                    .color(0xED4245);
                send_embed(ctx, msg, embed).await;
                return;
            }

            let embed = CreateEmbed::new()
                .title(format!("Icône du serveur {}", guild.name))
                .image(icon_url)
                .color(0x5865F2);

            send_embed(ctx, msg, embed).await;
        }
        "banner" => {
            let banner_url = guild.banner_url().unwrap_or_default();

            if banner_url.is_empty() {
                let embed = CreateEmbed::new()
                    .title("Erreur")
                    .description("Ce serveur n'a pas de bannière.")
                    .color(0xED4245);
                send_embed(ctx, msg, embed).await;
                return;
            }

            let embed = CreateEmbed::new()
                .title(format!("Bannière du serveur {}", guild.name))
                .image(banner_url)
                .color(0x5865F2);

            send_embed(ctx, msg, embed).await;
        }
        _ => {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description("Usage: `+server pic`, `+server banner` ou `+server list`")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
        }
    }
}

pub async fn handle_server_list(ctx: &Context, msg: &Message) {
    let guilds = guilds_sorted(ctx).await;
    let lines = guilds
        .iter()
        .enumerate()
        .map(|(index, (guild_id, name))| format!("{} · {} · `{}`", index + 1, name, guild_id.get()))
        .collect::<Vec<_>>();

    let mut embed = CreateEmbed::new()
        .title("Serveurs du bot")
        .color(theme_color(ctx).await);
    embed = add_list_fields(embed, &lines, "Guildes");
    let _ = msg
        .channel_id
        .send_message(&ctx.http, CreateMessage::new().embed(embed))
        .await;
}

pub(crate) async fn resolve_guild_target(ctx: &Context, input: &str) -> Option<GuildId> {
    let guilds = guilds_sorted(ctx).await;
    if let Ok(index) = input.parse::<usize>() {
        if index >= 1 && index <= guilds.len() {
            return Some(guilds[index - 1].0);
        }
    }

    input
        .parse::<u64>()
        .ok()
        .map(GuildId::new)
        .filter(|id| guilds.iter().any(|(guild_id, _)| guild_id == id))
}

async fn guilds_sorted(ctx: &Context) -> Vec<(GuildId, String)> {
    let mut all_guilds = Vec::new();
    let mut after: Option<GuildId> = None;

    loop {
        let page = if let Some(after_id) = after {
            ctx.http
                .get_guilds(Some(GuildPagination::After(after_id)), Some(100))
                .await
                .unwrap_or_default()
        } else {
            ctx.http
                .get_guilds(None, Some(100))
                .await
                .unwrap_or_default()
        };

        if page.is_empty() {
            break;
        }

        after = page.last().map(|guild| guild.id);
        all_guilds.extend(page.into_iter().map(|guild| (guild.id, guild.name)));

        if all_guilds.len() % 100 != 0 {
            break;
        }
    }

    all_guilds.sort_by(|a, b| {
        a.1.to_lowercase()
            .cmp(&b.1.to_lowercase())
            .then_with(|| a.0.get().cmp(&b.0.get()))
    });
    all_guilds
}

pub struct ServerCommand;
pub static COMMAND_DESCRIPTOR: ServerCommand = ServerCommand;

impl crate::commands::command_contract::CommandSpec for ServerCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "server",
            category: "infos",
            params: "pic | banner | list",
            summary: "Affiche et gere le serveur",
            description: "Affiche licone ou la banniere du serveur, ou liste les serveurs du bot selon la sous commande.",
            examples: &["+server", "+sr", "+help server"],
            default_aliases: &["srv"],
            default_permission: 0,
        }
    }
}
