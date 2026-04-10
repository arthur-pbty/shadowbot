use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{discord_ts, send_embed};

pub async fn handle_user(ctx: &Context, msg: &Message, args: &[&str]) {
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

    let created_at = discord_ts(user.created_at(), "F");
    let avatar_url = user.avatar_url().unwrap_or_default();

    let embed = CreateEmbed::new()
        .title(&user.name)
        .description(format!("ID: `{}`", user.id.get()))
        .color(0x5865F2)
        .thumbnail(&avatar_url)
        .field("Créé le", created_at, true)
        .field("Bot", if user.bot { "Oui" } else { "Non" }, true)
        .field(
            "Systématique",
            if user.system { "Oui" } else { "Non" },
            true,
        );

    send_embed(ctx, msg, embed).await;
}

pub struct UserCommand;
pub static COMMAND_DESCRIPTOR: UserCommand = UserCommand;

impl crate::commands::command_contract::CommandSpec for UserCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "user",
            command: "user",
            category: "general",
            params: "<@membre/ID>",
            summary: "Affiche le profil utilisateur",
            description: "Affiche les informations principales dun utilisateur cible.",
            examples: &["+user", "+ur", "+help user"],
            alias_source_key: "user",
            default_aliases: &["usr"],
        }
    }
}
