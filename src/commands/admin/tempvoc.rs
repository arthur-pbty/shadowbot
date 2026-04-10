use chrono::Utc;
use serenity::builder::{
    CreateActionRow, CreateButton, CreateChannel, CreateEmbed, CreateInputText,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateModal,
};
use serenity::model::Colour;
use serenity::model::application::{
    ActionRowComponent, ButtonStyle, ComponentInteraction, InputTextStyle, ModalInteraction,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::db;

const TEMPVOC_MENU: &str = "tempvoc:settings";

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

fn tempvoc_embed(settings: &db::TempvocSettings) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title("Tempvoc")
        .description("Gère les vocaux temporaires du serveur.")
        .colour(Colour::from_rgb(100, 180, 255))
        .timestamp(Utc::now())
        .field(
            "Statut",
            if settings.enabled { "Actif" } else { "Inactif" },
            true,
        );

    if let Some(trigger) = settings.trigger_channel_id {
        embed = embed.field("Canal déclencheur", format!("<#{}>", trigger), true);
    }

    if let Some(category) = settings.category_id {
        embed = embed.field("Catégorie", format!("<#{}>", category), true);
    }

    embed
}

fn tempvoc_components(owner_id: UserId, settings: &db::TempvocSettings) -> Vec<CreateActionRow> {
    let toggle_label = if settings.enabled {
        "Désactiver"
    } else {
        "Activer"
    };

    vec![CreateActionRow::Buttons(vec![
        CreateButton::new(format!("{}:toggle:{}", TEMPVOC_MENU, owner_id.get()))
            .label(toggle_label)
            .style(ButtonStyle::Primary),
        CreateButton::new(format!("{}:configure:{}", TEMPVOC_MENU, owner_id.get()))
            .label("Configurer")
            .style(ButtonStyle::Secondary),
        CreateButton::new(format!("{}:refresh:{}", TEMPVOC_MENU, owner_id.get()))
            .label("Rafraîchir")
            .style(ButtonStyle::Success),
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
    let settings = db::get_or_create_tempvoc_settings(&pool, bot_id, guild_id.get() as i64)
        .await
        .unwrap_or(db::TempvocSettings {
            bot_id,
            guild_id: guild_id.get() as i64,
            trigger_channel_id: None,
            category_id: None,
            enabled: false,
            updated_at: chrono::Utc::now(),
        });

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .embed(tempvoc_embed(&settings))
                .components(tempvoc_components(msg.author.id, &settings)),
        )
        .await;
}

pub async fn handle_tempvoc(ctx: &Context, msg: &Message, _args: &[&str]) {
    show_menu(ctx, msg).await;
}

pub async fn handle_component_interaction(ctx: &Context, component: &ComponentInteraction) -> bool {
    if !component.data.custom_id.starts_with(TEMPVOC_MENU) {
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
    let settings = db::get_or_create_tempvoc_settings(&pool, bot_id, guild_id.get() as i64)
        .await
        .ok();

    let Some(settings) = settings else {
        return true;
    };

    if action.ends_with(":configure") {
        let modal = CreateModal::new(component.data.custom_id.clone(), "Configurer Tempvoc")
            .components(vec![
                CreateActionRow::InputText(
                    CreateInputText::new(
                        InputTextStyle::Short,
                        "Canal déclencheur",
                        "trigger_channel_id",
                    )
                    .required(false),
                ),
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "Catégorie", "category_id")
                        .required(false),
                ),
            ]);

        let _ = component
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await;
        return true;
    }

    if action.ends_with(":toggle") {
        let new_settings = db::update_tempvoc_settings(
            &pool,
            bot_id,
            guild_id.get() as i64,
            settings.trigger_channel_id,
            settings.category_id,
            !settings.enabled,
        )
        .await
        .ok();

        if let Some(updated) = new_settings {
            let _ = component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .embed(tempvoc_embed(&updated))
                            .components(tempvoc_components(component.user.id, &updated)),
                    ),
                )
                .await;
        }

        return true;
    }

    if action.ends_with(":refresh") {
        let _ = component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(tempvoc_embed(&settings))
                        .components(tempvoc_components(component.user.id, &settings)),
                ),
            )
            .await;
        return true;
    }

    false
}

pub async fn handle_modal_interaction(ctx: &Context, modal: &ModalInteraction) -> bool {
    if !modal.data.custom_id.starts_with(TEMPVOC_MENU) {
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

    if !action.contains(":configure") {
        return false;
    }

    let Some(guild_id) = modal.guild_id else {
        return true;
    };

    let Some(pool) = pool(ctx).await else {
        return true;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let trigger_channel_id =
        modal_value(modal, "trigger_channel_id").and_then(|value| value.trim().parse::<i64>().ok());
    let category_id =
        modal_value(modal, "category_id").and_then(|value| value.trim().parse::<i64>().ok());

    let updated = db::update_tempvoc_settings(
        &pool,
        bot_id,
        guild_id.get() as i64,
        trigger_channel_id,
        category_id,
        true,
    )
    .await
    .ok();

    if let Some(updated) = updated {
        let _ = modal
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .embed(tempvoc_embed(&updated))
                        .components(tempvoc_components(modal.user.id, &updated))
                        .ephemeral(true),
                ),
            )
            .await;
    }

    true
}

fn sanitize_voice_name(input: &str) -> String {
    let mut out = String::new();
    let mut previous_dash = false;

    for ch in input.to_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            previous_dash = false;
        } else if (ch.is_whitespace() || ch == '-' || ch == '_') && !previous_dash {
            out.push('-');
            previous_dash = true;
        }
    }

    out.trim_matches('-').to_string()
}

async fn cached_room_members(ctx: &Context, guild_id: GuildId, channel_id: ChannelId) -> usize {
    ctx.cache
        .guild(guild_id)
        .map(|guild| {
            guild
                .voice_states
                .values()
                .filter(|state| state.channel_id == Some(channel_id))
                .count()
        })
        .unwrap_or(0)
}

async fn create_temp_channel(
    ctx: &Context,
    guild_id: GuildId,
    user: &User,
    settings: &db::TempvocSettings,
) {
    let Some(trigger_channel_id) = settings.trigger_channel_id else {
        return;
    };

    let Ok(trigger_channel) = ChannelId::new(trigger_channel_id as u64)
        .to_channel(&ctx.http)
        .await
    else {
        return;
    };

    let Channel::Guild(trigger) = trigger_channel else {
        return;
    };

    let category_id = settings
        .category_id
        .map(|value| ChannelId::new(value as u64))
        .or(trigger.parent_id);
    let name = sanitize_voice_name(&user.name);
    if name.is_empty() {
        return;
    }

    let mut builder = CreateChannel::new(format!("🎤 {}", name))
        .kind(ChannelType::Voice)
        .permissions(trigger.permission_overwrites.clone());

    if let Some(category_id) = category_id {
        builder = builder.category(category_id);
    }

    let Ok(channel) = guild_id.create_channel(&ctx.http, builder).await else {
        return;
    };

    if guild_id
        .move_member(&ctx.http, user.id, channel.id)
        .await
        .is_err()
    {
        let _ = channel.delete(&ctx.http).await;
        return;
    }

    if let Some(pool) = pool(ctx).await {
        let _ = db::create_tempvoc_room(
            &pool,
            settings.bot_id,
            settings.guild_id,
            channel.id.get() as i64,
            user.id.get() as i64,
        )
        .await;
    }
}

async fn delete_temp_channel(ctx: &Context, channel_id: ChannelId) {
    if let Ok(channel) = channel_id.to_channel(&ctx.http).await {
        if let Channel::Guild(guild_channel) = channel {
            let _ = guild_channel.delete(&ctx.http).await;
        }
    }
}

pub async fn handle_voice_state_update(ctx: &Context, old: Option<&VoiceState>, new: &VoiceState) {
    let Some(guild_id) = new.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let guild_id_i64 = guild_id.get() as i64;
    let settings = db::get_or_create_tempvoc_settings(&pool, bot_id, guild_id_i64)
        .await
        .ok();

    let Some(settings) = settings else {
        return;
    };

    let old_channel = old.and_then(|state| state.channel_id);
    let new_channel = new.channel_id;

    if settings.enabled
        && settings.trigger_channel_id.is_some()
        && new_channel.map(|channel| channel.get() as i64) == settings.trigger_channel_id
    {
        if let Ok(member) = guild_id.member(&ctx.http, new.user_id).await {
            create_temp_channel(ctx, guild_id, &member.user, &settings).await;
        }
    }

    if let Some(old_channel) = old_channel {
        if db::get_tempvoc_room_by_channel(&pool, old_channel.get() as i64)
            .await
            .ok()
            .flatten()
            .is_some()
            && cached_room_members(ctx, guild_id, old_channel).await == 0
        {
            delete_temp_channel(ctx, old_channel).await;
            let _ = db::delete_tempvoc_room(&pool, old_channel.get() as i64).await;
        }
    }
}

pub struct TempvocCommand;
pub static COMMAND_DESCRIPTOR: TempvocCommand = TempvocCommand;

impl crate::commands::command_contract::CommandSpec for TempvocCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "tempvoc",
            category: "admin",
            params: "[cmd]",
            summary: "Configure les vocaux temporaires",
            description: "Affiche le menu de configuration du systeme de vocaux temporaires.",
            examples: &["+tempvoc", "+tempvoc cmd", "+help tempvoc"],
            default_aliases: &[],
            default_permission: 8,
        }
    }
}
