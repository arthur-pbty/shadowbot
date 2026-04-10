use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color, truncate_text};

pub async fn handle_catsay(ctx: &Context, msg: &Message, args: &[&str]) {
    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Catsay")
            .description("Utilise `+catsay <message>`.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let text = truncate_text(&args.join(" "), 120);
    let bubble = format!("< {} >", text);
    let cat = format!("{}\n \\\n  \\\n   /\\_/\\\n  ( o.o )\n   > ^ <", bubble);

    let embed = CreateEmbed::new()
        .title("Catsay")
        .description(format!("```\n{}\n```", cat))
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct CatsayCommand;
pub static COMMAND_DESCRIPTOR: CatsayCommand = CatsayCommand;

impl crate::commands::command_contract::CommandSpec for CatsayCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "catsay",
            category: "game",
            params: "<message>",
            description: "Faire parler les chat.",
            examples: &["+catsay Bonjour"],
            default_aliases: &["meow"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
