use chrono::Utc;
use serenity::builder::{
    CreateActionRow, CreateButton, CreateEmbed, CreateInputText, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, CreateModal,
};
use serenity::model::Colour;
use serenity::model::application::{
    ActionRowComponent, ButtonStyle, ComponentInteraction, InputTextStyle, ModalInteraction,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::db;

const SUGGESTION_MENU: &str = "suggestion:settings";

fn parse_owner_id(custom_id: &str) -> Option<(String, u64)> {
    let mut parts = custom_id.rsplitn(2, ':');
    let owner = parts.next()?.parse::<u64>().ok()?;
    let action = parts.next()?.to_string();
    Some((action, owner))
}

fn modal_value(modal: &ModalInteraction, wanted_id: &str) -> Option<String> {
    for row in &modal.data.components {
        for component in &row.components {
            if let ActionRowComponent::InputText(input) = component {
                if input.custom_id == wanted_id {
                    return input.value.clone();
                }
            }
        }
    }
    None
}

fn suggestion_embed(author: &User, content: &str) -> CreateEmbed {
    CreateEmbed::new()
        .title("💡 Suggestion")
        .description(content)
        .colour(Colour::from_rgb(255, 200, 0))
        .author(serenity::builder::CreateEmbedAuthor::new(&author.name).icon_url(author.face()))
        .timestamp(Utc::now())
}

fn suggestion_settings_embed(settings: &db::SuggestionSettings) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title("Gestion des suggestions")
        .description("Configure le système de suggestions du serveur.")
        .colour(Colour::from_rgb(255, 200, 0))
        .timestamp(Utc::now())
        .field(
            "Statut",
            if settings.enabled { "Actif" } else { "Inactif" },
            true,
        );

    if let Some(channel_id) = settings.channel_id {
        embed = embed.field("Canal", format!("<#{}>", channel_id), true);
    }

    if let Some(approve_channel_id) = settings.approve_channel_id {
        embed = embed.field("Validation", format!("<#{}>", approve_channel_id), true);
    }

    embed
}

fn suggestion_components(
    owner_id: UserId,
    settings: &db::SuggestionSettings,
) -> Vec<CreateActionRow> {
    let toggle_label = if settings.enabled {
        "Désactiver"
    } else {
        "Activer"
    };

    vec![CreateActionRow::Buttons(vec![
        CreateButton::new(format!("{}:submit:{}", SUGGESTION_MENU, owner_id.get()))
            .label("Soumettre")
            .style(ButtonStyle::Success),
        CreateButton::new(format!("{}:configure:{}", SUGGESTION_MENU, owner_id.get()))
            .label("Configurer")
            .style(ButtonStyle::Secondary),
        CreateButton::new(format!("{}:toggle:{}", SUGGESTION_MENU, owner_id.get()))
            .label(toggle_label)
            .style(ButtonStyle::Primary),
        CreateButton::new(format!("{}:refresh:{}", SUGGESTION_MENU, owner_id.get()))
            .label("Rafraîchir")
            .style(ButtonStyle::Secondary),
    ])]
}

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<db::DbPoolKey>().cloned()
}

async fn show_menu(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let settings = db::get_or_create_suggestion_settings(&pool, bot_id, guild_id.get() as i64)
        .await
        .ok();

    let Some(settings) = settings else {
        return;
    };

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .embed(suggestion_settings_embed(&settings))
                .components(suggestion_components(msg.author.id, &settings)),
        )
        .await;
}

async fn submit_suggestion(
    ctx: &Context,
    guild_id: GuildId,
    author: &User,
    content: String,
) -> Result<(), String> {
    let pool = pool(ctx)
        .await
        .ok_or_else(|| "Base de données indisponible".to_string())?;
    let bot_id = ctx.cache.current_user().id.get() as i64;
    let settings = db::get_or_create_suggestion_settings(&pool, bot_id, guild_id.get() as i64)
        .await
        .map_err(|e| format!("Erreur: {e}"))?;

    if !settings.enabled {
        return Err("Le système de suggestions est désactivé.".to_string());
    }

    let channel_id = settings
        .channel_id
        .ok_or_else(|| "Canal de suggestions non configuré".to_string())?;
    let channel = ChannelId::new(channel_id as u64)
        .to_channel(&ctx.http)
        .await
        .map_err(|e| format!("Erreur: {e}"))?;
    let guild_channel = channel
        .guild()
        .ok_or_else(|| "Canal de suggestions introuvable".to_string())?;

    let message = guild_channel
        .send_message(
            &ctx.http,
            serenity::builder::CreateMessage::new().embed(suggestion_embed(author, &content)),
        )
        .await
        .map_err(|e| format!("Erreur: {e}"))?;

    db::create_suggestion(
        &pool,
        bot_id,
        guild_id.get() as i64,
        channel_id,
        message.id.get() as i64,
        author.id.get() as i64,
        content.clone(),
    )
    .await
    .map_err(|e| format!("Erreur: {e}"))?;

    let _ = message.react(&ctx.http, '👍').await;
    let _ = message.react(&ctx.http, '👎').await;

    if let Ok(channels) = db::get_autopublish_channels(&pool, bot_id, guild_id.get() as i64).await {
        for autopublish_channel in channels {
            if autopublish_channel.channel_id == channel_id {
                continue;
            }

            let _ = ChannelId::new(autopublish_channel.channel_id as u64)
                .send_message(
                    &ctx.http,
                    serenity::builder::CreateMessage::new()
                        .embed(suggestion_embed(author, &content)),
                )
                .await;
        }
    }

    Ok(())
}

pub async fn handle_suggestion(ctx: &Context, msg: &Message, args: &[&str]) {
    if args
        .first()
        .map(|value| value.eq_ignore_ascii_case("settings"))
        .unwrap_or(false)
    {
        show_menu(ctx, msg).await;
        return;
    }

    if args.is_empty() {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Suggestion")
                .description("Utilisation: +suggestion <contenu> ou +suggestion settings")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let content = args.join(" ");
    if content.trim().is_empty() {
        return;
    }

    if let Err(error) = submit_suggestion(ctx, guild_id, &msg.author, content).await {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Suggestion")
                .description(error)
                .color(0xED4245),
        )
        .await;
        return;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Suggestion envoyée")
            .description("La suggestion a été publiée.")
            .colour(Colour::from_rgb(0, 200, 120))
            .timestamp(Utc::now()),
    )
    .await;
}

pub async fn handle_component_interaction(ctx: &Context, component: &ComponentInteraction) -> bool {
    if !component.data.custom_id.starts_with(SUGGESTION_MENU) {
        return false;
    }

    let Some((action, owner_id)) = parse_owner_id(&component.data.custom_id) else {
        return false;
    };

    if component.user.id.get() != owner_id {
        let _ = component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Seul l'auteur du menu peut l'utiliser.")
                        .ephemeral(true),
                ),
            )
            .await;
        return true;
    }

    let Some(guild_id) = component.guild_id else {
        return true;
    };

    let Some(pool) = pool(ctx).await else {
        return true;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let settings = db::get_or_create_suggestion_settings(&pool, bot_id, guild_id.get() as i64)
        .await
        .ok();

    let Some(settings) = settings else {
        return true;
    };

    if action.ends_with(":refresh") {
        let _ = component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(suggestion_settings_embed(&settings))
                        .components(suggestion_components(component.user.id, &settings)),
                ),
            )
            .await;
        return true;
    }

    if action.ends_with(":toggle") {
        if let Ok(updated) = db::update_suggestion_settings(
            &pool,
            bot_id,
            guild_id.get() as i64,
            !settings.enabled,
            settings.channel_id,
            settings.approve_channel_id,
        )
        .await
        {
            let _ = component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .embed(suggestion_settings_embed(&updated))
                            .components(suggestion_components(component.user.id, &updated)),
                    ),
                )
                .await;
        }
        return true;
    }

    if action.ends_with(":configure") {
        let modal = CreateModal::new(
            component.data.custom_id.clone(),
            "Configurer les suggestions",
        )
        .components(vec![
            CreateActionRow::InputText(
                CreateInputText::new(InputTextStyle::Short, "Canal des suggestions", "channel_id")
                    .required(false),
            ),
            CreateActionRow::InputText(
                CreateInputText::new(
                    InputTextStyle::Short,
                    "Canal d'approbation",
                    "approve_channel_id",
                )
                .required(false),
            ),
        ]);

        let _ = component
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await;
        return true;
    }

    if action.ends_with(":submit") {
        let modal = CreateModal::new(component.data.custom_id.clone(), "Soumettre une suggestion")
            .components(vec![CreateActionRow::InputText(
                CreateInputText::new(InputTextStyle::Paragraph, "Contenu", "content")
                    .required(true)
                    .max_length(2000),
            )]);

        let _ = component
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await;
        return true;
    }

    false
}

pub async fn handle_modal_interaction(ctx: &Context, modal: &ModalInteraction) -> bool {
    if !modal.data.custom_id.starts_with(SUGGESTION_MENU) {
        return false;
    }

    let Some((action, owner_id)) = parse_owner_id(&modal.data.custom_id) else {
        return false;
    };

    if modal.user.id.get() != owner_id {
        let _ = modal
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Seul l'auteur du menu peut soumettre ce formulaire.")
                        .ephemeral(true),
                ),
            )
            .await;
        return true;
    }

    let Some(guild_id) = modal.guild_id else {
        return true;
    };

    let Some(pool) = pool(ctx).await else {
        return true;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let current = db::get_or_create_suggestion_settings(&pool, bot_id, guild_id.get() as i64)
        .await
        .ok();

    let Some(settings) = current else {
        return true;
    };

    if action.ends_with(":configure") {
        let channel_id =
            modal_value(modal, "channel_id").and_then(|value| value.trim().parse::<i64>().ok());
        let approve_channel_id = modal_value(modal, "approve_channel_id")
            .and_then(|value| value.trim().parse::<i64>().ok());

        if let Ok(updated) = db::update_suggestion_settings(
            &pool,
            bot_id,
            guild_id.get() as i64,
            settings.enabled,
            channel_id,
            approve_channel_id,
        )
        .await
        {
            let _ = modal
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .embed(suggestion_settings_embed(&updated))
                            .components(suggestion_components(modal.user.id, &updated))
                            .ephemeral(true),
                    ),
                )
                .await;
        }
        return true;
    }

    if action.ends_with(":submit") {
        let content = modal_value(modal, "content").unwrap_or_default();
        if content.trim().is_empty() {
            let _ = modal
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("Contenu invalide.")
                            .ephemeral(true),
                    ),
                )
                .await;
            return true;
        }

        match submit_suggestion(ctx, guild_id, &modal.user, content).await {
            Ok(_) => {
                let _ = modal
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("Suggestion envoyée.")
                                .ephemeral(true),
                        ),
                    )
                    .await;
            }
            Err(error) => {
                let _ = modal
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content(error)
                                .ephemeral(true),
                        ),
                    )
                    .await;
            }
        }

        return true;
    }

    false
}

pub struct SuggestionCommand;
pub static COMMAND_DESCRIPTOR: SuggestionCommand = SuggestionCommand;

impl crate::commands::command_contract::CommandSpec for SuggestionCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "suggestion",
            category: "admin",
            params: "<contenu...> | settings",
            summary: "Publie ou configure les suggestions",
            description: "Publie une suggestion utilisateur ou ouvre le panneau de configuration.",
            examples: &[
                "+suggestion Ameliorer le salon",
                "+suggestion settings",
                "+help suggestion",
            ],
            default_aliases: &[],
            default_permission: 0,
        }
    }
}
