use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{mention_user, send_embed, theme_color};

pub async fn handle_marry(ctx: &Context, msg: &Message, args: &[&str]) {
    let target = msg
        .mentions
        .first()
        .map(|user| mention_user(user.id))
        .or_else(|| {
            args.first().map(|raw| {
                if raw.starts_with("<@") {
                    raw.to_string()
                } else {
                    format!("**{}**", raw)
                }
            })
        });

    let Some(target) = target else {
        let embed = CreateEmbed::new()
            .title("Marry")
            .description("Utilise `+marry <@user>` pour faire une demande.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let embed = CreateEmbed::new()
        .title("Marry")
        .description(format!(
            "{} propose en mariage a {}. Reponse attendue dans le chat.",
            mention_user(msg.author.id),
            target
        ))
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct MarryCommand;
pub static COMMAND_DESCRIPTOR: MarryCommand = MarryCommand;

impl crate::commands::command_contract::CommandSpec for MarryCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "marry",
            category: "game",
            params: "<@user>",
            description: "Proposez en mariage a un utilisateur.",
            examples: &["+marry @Pseudo"],
            default_aliases: &["proposal"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
