use serenity::builder::CreateChannel;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::parse_channel_id;

pub async fn handle_renew(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let channel_id = args
        .first()
        .and_then(|raw| parse_channel_id(raw))
        .unwrap_or(msg.channel_id);

    let Ok(channel) = channel_id.to_channel(&ctx.http).await else {
        return;
    };
    let Channel::Guild(text_channel) = channel else {
        return;
    };

    if text_channel.kind != ChannelType::Text && text_channel.kind != ChannelType::News {
        return;
    }

    let parent_id = text_channel.parent_id;
    let topic = text_channel.topic.clone();
    let nsfw = text_channel.nsfw;
    let slowmode = text_channel.rate_limit_per_user;
    let name = text_channel.name.clone();

    let _ = text_channel.delete(&ctx.http).await;

    let mut builder = CreateChannel::new(name)
        .kind(ChannelType::Text)
        .nsfw(nsfw)
        .rate_limit_per_user(slowmode.unwrap_or(0));

    if let Some(parent) = parent_id {
        builder = builder.category(parent);
    }
    if let Some(topic) = topic {
        builder = builder.topic(topic);
    }

    let _ = guild_id.create_channel(&ctx.http, builder).await;
}

pub struct RenewCommand;
pub static COMMAND_DESCRIPTOR: RenewCommand = RenewCommand;

impl crate::commands::command_contract::CommandSpec for RenewCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "renew",
            category: "moderation",
            params: "[salon]",
            summary: "Recree un salon textuel",
            description: "Supprime puis recree un salon textuel en conservant les options principales.",
            examples: &["+renew", "+renew #general"],
            default_aliases: &["nuke", "rebuildch"],
            default_permission: 8,
        }
    }
}
