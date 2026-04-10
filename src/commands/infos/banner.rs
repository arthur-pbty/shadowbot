use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;

pub async fn handle_banner(ctx: &Context, msg: &Message, args: &[&str]) {
    let user = if args.is_empty() {
        msg.author.clone()
    } else {
        let user_id = args[0]
            .trim_start_matches('<')
            .trim_end_matches('>')
            .trim_start_matches('@')
            .trim_start_matches('!')
            .parse::<u64>()
            .ok()
            .map(UserId::new);

        if let Some(uid) = user_id {
            match ctx.http.get_user(uid).await {
                Ok(u) => u,
                Err(_) => {
                    let embed = CreateEmbed::new()
                        .title("Erreur")
                        .description("Utilisateur non trouvé.")
                        .color(0xED4245);
                    send_embed(ctx, msg, embed).await;
                    return;
                }
            }
        } else {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description("Impossible de parser l'utilisateur.")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        }
    };

    let banner_url = user.banner_url().unwrap_or_default();

    if banner_url.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description(format!("{} n'a pas de bannière.", user.name))
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let embed = CreateEmbed::new()
        .title(format!("Bannière de {}", user.name))
        .image(banner_url)
        .color(0x5865F2);

    send_embed(ctx, msg, embed).await;
}

pub struct BannerCommand;
pub static COMMAND_DESCRIPTOR: BannerCommand = BannerCommand;

impl crate::commands::command_contract::CommandSpec for BannerCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "banner",
            category: "infos",
            params: "<@membre/ID>",
            summary: "Affiche la banniere utilisateur",
            description: "Affiche la banniere de profil dun utilisateur cible ou de lauteur.",
            examples: &["+banner", "+br", "+help banner"],
            default_aliases: &["bnr"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
