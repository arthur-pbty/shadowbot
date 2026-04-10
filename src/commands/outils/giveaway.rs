use serenity::builder::{
    CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter, CreateMessage,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::theme_color;

fn owned_component_id(action: &str, owner_id: UserId) -> String {
    format!("{}:{}", action, owner_id.get())
}

pub async fn handle_giveaway(ctx: &Context, msg: &Message, args: &[&str]) {
    let _ = args;

    let embed = CreateEmbed::new()
        .title("Giveaway")
        .description("Utilise les boutons pour creer ou terminer un giveaway via modal.")
        .color(theme_color(ctx).await)
        .footer(CreateEmbedFooter::new("UI avancee: Components + Modal"));

    let components = vec![CreateActionRow::Buttons(vec![
        CreateButton::new(owned_component_id("adv:giveaway:open_modal", msg.author.id))
            .label("Creer")
            .emoji('🎉')
            .style(ButtonStyle::Success),
        CreateButton::new(owned_component_id("adv:giveaway:end_modal", msg.author.id))
            .label("Terminer")
            .emoji('🛑')
            .style(ButtonStyle::Danger),
    ])];

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new().embed(embed).components(components),
        )
        .await;
}

pub struct GiveawayCommand;
pub static COMMAND_DESCRIPTOR: GiveawayCommand = GiveawayCommand;

impl crate::commands::command_contract::CommandSpec for GiveawayCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "giveaway",
            category: "outils",
            params: "aucun",
            summary: "Ouvre un menu de creation de giveaway",
            description: "Affiche une interface rapide pour initier un giveaway depuis le salon courant.",
            examples: &["+giveaway"],
            default_aliases: &["gstart", "gw"],
            default_permission: 8,
        }
    }
}
