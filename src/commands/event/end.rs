use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed, CreateMessage, EditMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

fn owned_component_id(action: &str, owner_id: UserId) -> String {
    format!("{}:{}", action, owner_id.get())
}

pub async fn handle_endgiveaway(ctx: &Context, msg: &Message, args: &[&str]) {
    let message_id_raw = args.first().copied().unwrap_or_default();

    let Ok(message_id) = message_id_raw.trim().parse::<u64>() else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("End")
                .description("ID du message invalide.")
                .color(0xED4245),
        )
        .await;
        return;
    };

    let result = msg
        .channel_id
        .edit_message(
            &ctx.http,
            MessageId::new(message_id),
            EditMessage::new().content("🎉 Giveaway termine manuellement."),
        )
        .await;

    let (description, color) = if result.is_ok() {
        ("Giveaway termine.", theme_color(ctx).await)
    } else {
        ("Impossible de terminer ce giveaway.", 0xED4245)
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("End")
            .description(description)
            .color(color),
    )
    .await;
}

pub async fn handle_end(ctx: &Context, msg: &Message, args: &[&str]) {
    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("End")
            .description("Utilise le bouton pour terminer un giveaway via modal.")
            .color(theme_color(ctx).await);
        let components = vec![CreateActionRow::Buttons(vec![
            CreateButton::new(owned_component_id("adv:giveaway:end_modal", msg.author.id))
                .label("Terminer un giveaway")
                .style(ButtonStyle::Danger),
        ])];

        let _ = msg
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().embed(embed).components(components),
            )
            .await;
        return;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("End")
            .description("Usage: +end")
            .color(0xED4245),
    )
    .await;
}

pub struct EndCommand;
pub static COMMAND_DESCRIPTOR: EndCommand = EndCommand;

impl crate::commands::command_contract::CommandSpec for EndCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "end",
            category: "event",
            params: "aucun",
            description: "Affiche le panneau interactif pour terminer un giveaway.",
            examples: &["+end", "+help end"],
            default_aliases: &["endmenu"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
