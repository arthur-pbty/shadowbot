use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed, CreateMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

fn owned_component_id(action: &str, owner_id: UserId) -> String {
    format!("{}:{}", action, owner_id.get())
}

pub async fn handle_embed(ctx: &Context, msg: &Message, args: &[&str]) {
    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Embed")
            .description("Utilise le bouton pour ouvrir le generateur d'embed.")
            .color(theme_color(ctx).await);
        let components = vec![CreateActionRow::Buttons(vec![
            CreateButton::new(owned_component_id("adv:embed:modal", msg.author.id))
                .label("Ouvrir le generateur")
                .style(ButtonStyle::Primary),
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

    let joined = args.join(" ");
    let mut split = joined.splitn(2, '|').map(str::trim);
    let title = split.next().unwrap_or_default();
    let description = split.next().unwrap_or_default();

    if title.is_empty() || description.is_empty() {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Embed")
                .description("Format attendu: +embed titre | description")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new().embed(
                CreateEmbed::new()
                    .title(title)
                    .description(description)
                    .color(theme_color(ctx).await),
            ),
        )
        .await;
}

pub struct EmbedCommand;
pub static COMMAND_DESCRIPTOR: EmbedCommand = EmbedCommand;

impl crate::commands::command_contract::CommandSpec for EmbedCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "embed",
            category: "fun",
            params: "title | description (v1)",
            description: "Affiche un generateur d'embed interactif version rapide.",
            examples: &["+embed"],
            default_aliases: &["emb"],
            allow_in_dm: false,
            default_permission: 2,
        }
    }
}
