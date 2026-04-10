use rand::seq::SliceRandom;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_eightball(ctx: &Context, msg: &Message, args: &[&str]) {
    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("8ball")
            .description("Pose ta question avec `+8ball <question>`.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let answers = [
        "Oui, clairement.",
        "Probablement.",
        "Je ne pense pas.",
        "Reessaye plus tard.",
        "C est certain.",
        "Impossible a dire pour le moment.",
    ];

    let answer = {
        let mut rng = rand::thread_rng();
        answers
            .choose(&mut rng)
            .copied()
            .unwrap_or("Reessaye plus tard.")
    };

    let question = args.join(" ");
    let embed = CreateEmbed::new()
        .title("8ball")
        .description(format!("Question: {}\nReponse: **{}**", question, answer))
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct EightballCommand;
pub static COMMAND_DESCRIPTOR: EightballCommand = EightballCommand;

impl crate::commands::command_contract::CommandSpec for EightballCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "8ball",
            category: "game",
            params: "<question>",
            description: "Posez une question a la boule magique 8.",
            examples: &["+8ball Vais-je gagner ?"],
            default_aliases: &["magic8"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
