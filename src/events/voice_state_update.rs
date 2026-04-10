use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;
use crate::commands::tempvoc;

pub async fn handle_voice_state_update(ctx: &Context, old: Option<VoiceState>, new: &VoiceState) {
    tempvoc::handle_voice_state_update(ctx, old.as_ref(), new).await;

    let Some(guild_id) = new.guild_id else {
        return;
    };

    let old_channel = old.and_then(|v| v.channel_id);
    let new_channel = new.channel_id;
    if old_channel == new_channel {
        return;
    }

    logs_service::on_voice_update(ctx, guild_id, new.user_id, old_channel, new_channel).await;
}
