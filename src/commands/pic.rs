use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;

pub async fn handle_pic(ctx: &Context, msg: &Message, args: &[&str]) {
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

    let avatar_url = user.avatar_url().unwrap_or_default();

    if avatar_url.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Cet utilisateur n'a pas de photo de profil.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let embed = CreateEmbed::new()
        .title(format!("Photo de profil de {}", user.name))
        .image(avatar_url)
        .color(0x5865F2);

    send_embed(ctx, msg, embed).await;
}

pub struct PicCommand;
pub static COMMAND_DESCRIPTOR: PicCommand = PicCommand;

impl crate::commands::command_contract::CommandSpec for PicCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "pic",
            command: "pic",
            category: "general",
            params: "<@membre/ID>",
            summary: "Affiche la photo de profil",
            description: "Affiche la photo de profil dun utilisateur cible ou de lauteur.",
            examples: &["+pic", "+pc", "+help pic"],
            alias_source_key: "pic",
            default_aliases: &["pfp"],
        }
    }
}
