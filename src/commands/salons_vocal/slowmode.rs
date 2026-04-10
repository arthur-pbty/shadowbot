use serenity::builder::{CreateEmbed, EditChannel};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_channel_id, send_embed, theme_color};
use crate::commands::moderation_sanction_helpers::duration_from_input;

const SLOWMODE_MAX_SECONDS: u64 = 6 * 60 * 60;

fn parse_slowmode_seconds(raw: &str) -> Option<u64> {
    let normalized = raw.trim().to_lowercase();
    if normalized.is_empty() {
        return None;
    }

    if matches!(normalized.as_str(), "off" | "none" | "disable" | "disabled") {
        return Some(0);
    }

    if let Ok(seconds) = normalized.parse::<u64>() {
        return Some(seconds);
    }

    duration_from_input(&normalized).map(|duration| duration.as_secs())
}

pub async fn handle_slowmode(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(_guild_id) = msg.guild_id else {
        return;
    };

    let Some(raw_duration) = args.first() else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Slowmode")
                .description("Utilisation: +slowmode <duree> [salon]")
                .color(theme_color(ctx).await),
        )
        .await;
        return;
    };

    let Some(seconds) = parse_slowmode_seconds(raw_duration) else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Slowmode")
                .description("Duree invalide. Exemples: 10s, 2m, 1h, off")
                .color(theme_color(ctx).await),
        )
        .await;
        return;
    };

    if seconds > SLOWMODE_MAX_SECONDS {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Slowmode")
                .description("La duree maximale du mode lent est 6h.")
                .color(theme_color(ctx).await),
        )
        .await;
        return;
    }

    let target = args
        .get(1)
        .and_then(|raw| parse_channel_id(raw))
        .unwrap_or(msg.channel_id);

    let applied = target
        .edit(
            &ctx.http,
            EditChannel::new().rate_limit_per_user(seconds as u16),
        )
        .await
        .is_ok();

    let description = if applied {
        if seconds == 0 {
            format!("Mode lent desactive sur <#{}>.", target.get())
        } else {
            format!(
                "Mode lent de {}s applique sur <#{}>.",
                seconds,
                target.get()
            )
        }
    } else {
        "Echec de mise a jour du mode lent (verifie le type de salon et les permissions)."
            .to_string()
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Slowmode")
            .description(description)
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct SlowmodeCommand;
pub static COMMAND_DESCRIPTOR: SlowmodeCommand = SlowmodeCommand;

impl crate::commands::command_contract::CommandSpec for SlowmodeCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "slowmode",
            category: "salons_vocal",
            params: "<duree> [salon]",
            description: "Modifie la duree du mode lent sur un salon texte (maximum 6 heures).",
            examples: &["+slowmode 10s", "+slowmode 2m #general", "+slowmode off"],
            default_aliases: &["sm"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
