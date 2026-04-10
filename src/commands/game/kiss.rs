use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{mention_user, send_embed, theme_color};

pub async fn handle_kiss(ctx: &Context, msg: &Message, args: &[&str]) {
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
        .unwrap_or_else(|| mention_user(msg.author.id));

    let embed = CreateEmbed::new()
        .title("Kiss")
        .description(format!(
            "{} fait un bisou a {}.",
            mention_user(msg.author.id),
            target
        ))
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct KissCommand;
pub static COMMAND_DESCRIPTOR: KissCommand = KissCommand;

impl crate::commands::command_contract::CommandSpec for KissCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "kiss",
            category: "game",
            params: "[@user]",
            description: "Fait un bisou a un utilisateur mentionne ou a un utilisateur.",
            examples: &["+kiss", "+kiss @Pseudo"],
            default_aliases: &["bisou"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
