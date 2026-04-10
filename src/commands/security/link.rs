use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::automod_service::pool;
use crate::commands::common::{parse_channel_id, send_embed};
use crate::db;

pub async fn handle_link_override(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(action) = args.first() else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Link")
                .description("Usage: +link <allow/deny/reset> [#salon]")
                .color(0xED4245),
        )
        .await;
        return;
    };

    let channel_id = args
        .get(1)
        .and_then(|raw| parse_channel_id(raw))
        .unwrap_or(msg.channel_id);

    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let guild_id_raw = guild_id.get() as i64;
    let channel_id_raw = channel_id.get() as i64;

    let description = if action.eq_ignore_ascii_case("allow") {
        let _ = db::set_moderation_channel_override(
            &pool,
            bot_id,
            guild_id_raw,
            channel_id_raw,
            "link",
            "allow",
        )
        .await;
        format!("AntiLink desactive dans <#{}>.", channel_id.get())
    } else if action.eq_ignore_ascii_case("deny") {
        let _ = db::set_moderation_channel_override(
            &pool,
            bot_id,
            guild_id_raw,
            channel_id_raw,
            "link",
            "deny",
        )
        .await;
        format!("AntiLink force dans <#{}>.", channel_id.get())
    } else if action.eq_ignore_ascii_case("reset") {
        let _ = db::remove_moderation_channel_override(
            &pool,
            bot_id,
            guild_id_raw,
            channel_id_raw,
            "link",
        )
        .await;
        format!("Override antilink supprime dans <#{}>.", channel_id.get())
    } else {
        return;
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Link Override")
            .description(description)
            .color(0x57F287),
    )
    .await;
}

pub struct LinkCommand;
pub static COMMAND_DESCRIPTOR: LinkCommand = LinkCommand;

impl crate::commands::command_contract::CommandSpec for LinkCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "link",
            category: "security",
            params: "<allow/deny/reset> [#salon]",
            description: "Definit l override antilink pour un salon (allow, deny, reset).",
            examples: &["+link allow #general", "+link deny #regles", "+link reset"],
            default_aliases: &["linkch"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
