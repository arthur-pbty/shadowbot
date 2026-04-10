use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{mention_user, send_embed, theme_color};

pub async fn handle_claque(ctx: &Context, msg: &Message, args: &[&str]) {
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
        })
        .unwrap_or_else(|| "le vide".to_string());

    let embed = CreateEmbed::new()
        .title("Claque")
        .description(format!(
            "{} met une claque a {}.",
            mention_user(msg.author.id),
            target
        ))
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct ClaqueCommand;
pub static COMMAND_DESCRIPTOR: ClaqueCommand = ClaqueCommand;

impl crate::commands::command_contract::CommandSpec for ClaqueCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "claque",
            category: "game",
            params: "[@user]",
            description: "Fait une claque a un utilisateur mentionne ou a un utilisateur.",
            examples: &["+claque", "+claque @Pseudo"],
            default_aliases: &["slap"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
