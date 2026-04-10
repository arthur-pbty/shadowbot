use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::Colour;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;

pub async fn handle_tempvoc_cmd(ctx: &Context, msg: &Message, _args: &[&str]) {
    let embed = CreateEmbed::new()
        .title("Commandes Tempvoc")
        .description("\n+tempvoc\n+tempvoc cmd\n\nLe salon de création est surveillé automatiquement: lorsqu'un membre le rejoint, un vocal temporaire est créé et l'utilisateur y est déplacé.")
        .colour(Colour::from_rgb(100, 180, 255))
        .timestamp(Utc::now());

    send_embed(ctx, msg, embed).await;
}

pub struct TempvocCmdCommand;
pub static COMMAND_DESCRIPTOR: TempvocCmdCommand = TempvocCmdCommand;

impl crate::commands::command_contract::CommandSpec for TempvocCmdCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "tempvoc_cmd",
            category: "salons_vocal",
            params: "aucun",
            summary: "Affiche laide tempvoc",
            description: "Affiche un rappel des commandes et du fonctionnement de tempvoc.",
            examples: &["+tempvoc cmd", "+help tempvoc_cmd"],
            default_aliases: &[],
            allow_in_dm: false,
            default_permission: 0,
        }
    }
}
