use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_channel_id, send_embed, theme_color};

pub async fn handle_bringall(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let target_channel = if let Some(raw) = args.first() {
        parse_channel_id(raw)
    } else {
        let Some(guild) = guild_id.to_guild_cached(&ctx.cache) else {
            return;
        };
        guild
            .voice_states
            .get(&msg.author.id)
            .and_then(|v| v.channel_id)
    };

    let Some(target_channel) = target_channel else {
        return;
    };

    let user_ids = {
        let Some(guild) = guild_id.to_guild_cached(&ctx.cache) else {
            return;
        };

        guild
            .voice_states
            .iter()
            .filter_map(|(uid, state)| state.channel_id.map(|_| *uid))
            .collect::<Vec<_>>()
    };

    let mut moved = 0usize;
    for user_id in user_ids {
        if guild_id
            .move_member(&ctx.http, user_id, target_channel)
            .await
            .is_ok()
        {
            moved += 1;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("BringAll")
            .description(format!("{} membres déplacés.", moved))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct BringAllCommand;
pub static COMMAND_DESCRIPTOR: BringAllCommand = BringAllCommand;

impl crate::commands::command_contract::CommandSpec for BringAllCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "bringall",
            category: "admin",
            params: "[salon_vocal_destination]",
            summary: "Rassemble tous les vocaux",
            description: "Deplace tous les membres actuellement en vocal vers un salon cible.",
            examples: &["+bringall #Event", "+bringall"],
            default_aliases: &["ball", "vbring"],
            default_permission: 8,
        }
    }
}
