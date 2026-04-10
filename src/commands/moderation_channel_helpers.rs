use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn edit_channel_visibility(
    ctx: &Context,
    guild_id: GuildId,
    channel_id: ChannelId,
    lock: Option<bool>,
    hide: Option<bool>,
) -> bool {
    let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await else {
        return false;
    };

    let everyone_role = guild
        .roles
        .values()
        .find(|r| r.name == "@everyone")
        .map(|r| r.id);
    let Some(everyone_role) = everyone_role else {
        return false;
    };

    let Ok(channels) = guild_id.channels(&ctx.http).await else {
        return false;
    };
    let Some(channel) = channels.get(&channel_id) else {
        return false;
    };

    let mut allow = Permissions::empty();
    let mut deny = Permissions::empty();

    if let Some(locked) = lock {
        if channel.kind == ChannelType::Text || channel.kind == ChannelType::News {
            if locked {
                deny |= Permissions::SEND_MESSAGES;
            } else {
                allow |= Permissions::SEND_MESSAGES;
            }
        } else if locked {
            deny |= Permissions::CONNECT | Permissions::SPEAK;
        } else {
            allow |= Permissions::CONNECT | Permissions::SPEAK;
        }
    }

    if let Some(hidden) = hide {
        if hidden {
            deny |= Permissions::VIEW_CHANNEL;
        } else {
            allow |= Permissions::VIEW_CHANNEL;
        }
    }

    channel_id
        .create_permission(
            &ctx.http,
            PermissionOverwrite {
                allow,
                deny,
                kind: PermissionOverwriteType::Role(everyone_role),
            },
        )
        .await
        .is_ok()
}
