use serenity::builder::{CreateEmbed, EditChannel};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{parse_channel_id, send_embed, theme_color};

pub async fn handle_sync(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(scope) = args.first() else {
        return;
    };

    let Ok(channels) = guild_id.channels(&ctx.http).await else {
        return;
    };

    let ids_to_sync = if scope.eq_ignore_ascii_case("all") {
        channels.keys().copied().collect::<Vec<_>>()
    } else if let Some(channel_id) = parse_channel_id(scope) {
        if let Some(target) = channels.get(&channel_id) {
            if target.kind == ChannelType::Category {
                channels
                    .values()
                    .filter(|ch| ch.parent_id == Some(channel_id))
                    .map(|ch| ch.id)
                    .collect::<Vec<_>>()
            } else {
                vec![channel_id]
            }
        } else {
            vec![channel_id]
        }
    } else {
        Vec::new()
    };

    let mut synced = 0usize;
    for channel_id in ids_to_sync {
        let Some(channel) = channels.get(&channel_id) else {
            continue;
        };
        let Some(parent_id) = channel.parent_id else {
            continue;
        };
        let Some(parent) = channels.get(&parent_id) else {
            continue;
        };

        if channel_id
            .edit(
                &ctx.http,
                EditChannel::new().permissions(parent.permission_overwrites.clone()),
            )
            .await
            .is_ok()
        {
            synced += 1;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Sync")
            .description(format!("{} salons synchronisés.", synced))
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct SyncCommand;
pub static COMMAND_DESCRIPTOR: SyncCommand = SyncCommand;

impl crate::commands::command_contract::CommandSpec for SyncCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "sync",
            category: "roles",
            params: "<salon/categorie/all>",
            description: "Synchronise les permissions d'un salon avec sa categorie, ou tous les salons avec all.",
            examples: &["+sync all", "+sync #general"],
            default_aliases: &["chsync"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
