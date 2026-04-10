use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{mention_user, send_embed, theme_color};

pub async fn handle_unmarry(ctx: &Context, msg: &Message, _args: &[&str]) {
    let author = mention_user(msg.author.id);
    let target = msg
        .mentions
        .first()
        .map(|user| mention_user(user.id))
        .unwrap_or_else(|| "ton partenaire imaginaire".to_string());

    let embed = CreateEmbed::new()
        .title("Unmarry")
        .description(format!("{} a dissous le mariage avec {}.", author, target))
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct UnmarryCommand;
pub static COMMAND_DESCRIPTOR: UnmarryCommand = UnmarryCommand;

impl crate::commands::command_contract::CommandSpec for UnmarryCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "unmarry",
            category: "game",
            params: "[@user]",
            description: "Dissoudre un mariage.",
            examples: &["+unmarry", "+unmarry @Pseudo"],
            default_aliases: &["divorce"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
