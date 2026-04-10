use serenity::builder::{CreateEmbed, EditRole};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::automod_service::pool;
use crate::commands::common::send_embed;
use crate::db;

fn mute_permissions() -> Permissions {
    Permissions::SEND_MESSAGES | Permissions::ADD_REACTIONS | Permissions::SPEAK
}

pub async fn handle_muterole(ctx: &Context, msg: &Message, _args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let Ok(settings) =
        db::get_or_create_moderation_settings(&pool, bot_id, guild_id.get() as i64).await
    else {
        return;
    };

    let mut role_id = settings
        .mute_role_id
        .and_then(|raw| u64::try_from(raw).ok())
        .map(RoleId::new);

    let Ok(partial_guild) = guild_id.to_partial_guild(&ctx.http).await else {
        return;
    };

    if role_id.is_none() {
        role_id = partial_guild
            .roles
            .values()
            .find(|role| role.name.eq_ignore_ascii_case("Muted"))
            .map(|role| role.id);
    }

    if role_id.is_none() {
        let created = guild_id
            .create_role(
                &ctx.http,
                EditRole::new()
                    .name("Muted")
                    .permissions(Permissions::empty()),
            )
            .await
            .ok();
        role_id = created.map(|role| role.id);
    }

    let Some(role_id) = role_id else {
        return;
    };

    let mut failed_channels = Vec::new();
    if let Ok(channels) = guild_id.channels(&ctx.http).await {
        for channel in channels.values() {
            let result = channel
                .create_permission(
                    &ctx.http,
                    PermissionOverwrite {
                        allow: Permissions::empty(),
                        deny: mute_permissions(),
                        kind: PermissionOverwriteType::Role(role_id),
                    },
                )
                .await;

            if result.is_err() {
                failed_channels.push(channel.id.get());
            }
        }
    }

    let _ = db::set_mute_role(
        &pool,
        bot_id,
        guild_id.get() as i64,
        Some(role_id.get() as i64),
    )
    .await;

    let mut description = format!(
        "Role muet configure: <@&{}>.\nPermissions appliquees sur les salons du serveur.",
        role_id.get()
    );

    if !failed_channels.is_empty() {
        let list = failed_channels
            .iter()
            .take(10)
            .map(|id| format!("<#{}>", id))
            .collect::<Vec<_>>()
            .join(", ");
        description.push_str(&format!(
            "\nErreurs permissions: {} salon(s). {}",
            failed_channels.len(),
            list
        ));
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("MuteRole")
            .description(description)
            .color(if failed_channels.is_empty() {
                0x57F287
            } else {
                0xFEE75C
            }),
    )
    .await;
}

pub struct MuteRoleCommand;
pub static COMMAND_DESCRIPTOR: MuteRoleCommand = MuteRoleCommand;

impl crate::commands::command_contract::CommandSpec for MuteRoleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "muterole",
            category: "moderation",
            params: "aucun",
            description: "Cree ou met a jour le role muet et tente de corriger les permissions des salons.",
            examples: &["+muterole", "+help muterole"],
            default_aliases: &["mr"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
