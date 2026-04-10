use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::events::{
    channel_event, guild_create_event, guild_member_event, interaction_create_event,
    message_delete_event, message_event, message_update_event, ready_event, role_event,
    voice_state_update_event,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready_event::handle_ready(&ctx, &ready).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        message_event::handle_message(&ctx, &msg).await;
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: Option<bool>) {
        guild_create_event::handle_guild_create(&ctx, &guild).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create_event::handle_interaction_create(&ctx, &interaction).await;
    }

    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        deleted_message_id: MessageId,
        guild_id: Option<GuildId>,
    ) {
        message_delete_event::handle_message_delete(&ctx, channel_id, deleted_message_id, guild_id)
            .await;
    }

    async fn message_update(
        &self,
        ctx: Context,
        old_if_available: Option<Message>,
        new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        message_update_event::handle_message_update(&ctx, old_if_available, new, &event).await;
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        voice_state_update_event::handle_voice_state_update(&ctx, old, &new).await;
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        guild_member_event::handle_member_addition(&ctx, &new_member).await;
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        _member_data_if_available: Option<Member>,
    ) {
        guild_member_event::handle_member_removal(&ctx, guild_id, &user).await;
    }

    async fn guild_member_update(
        &self,
        ctx: Context,
        old_if_available: Option<Member>,
        new: Option<Member>,
        event: GuildMemberUpdateEvent,
    ) {
        guild_member_event::handle_member_update(&ctx, old_if_available, new, &event).await;
    }

    async fn guild_role_create(&self, ctx: Context, new: Role) {
        role_event::handle_role_create(&ctx, &new).await;
    }

    async fn guild_role_update(
        &self,
        ctx: Context,
        old_data_if_available: Option<Role>,
        new: Role,
    ) {
        role_event::handle_role_update(&ctx, old_data_if_available, &new).await;
    }

    async fn guild_role_delete(
        &self,
        ctx: Context,
        guild_id: GuildId,
        removed_role_id: RoleId,
        removed_role_data_if_available: Option<Role>,
    ) {
        role_event::handle_role_delete(
            &ctx,
            guild_id,
            removed_role_id,
            removed_role_data_if_available,
        )
        .await;
    }

    async fn channel_create(&self, ctx: Context, guild_channel: GuildChannel) {
        channel_event::handle_channel_create(&ctx, &guild_channel).await;
    }

    async fn channel_update(&self, ctx: Context, old: Option<GuildChannel>, new: GuildChannel) {
        channel_event::handle_channel_update(&ctx, old, &new).await;
    }

    async fn channel_delete(
        &self,
        ctx: Context,
        channel: GuildChannel,
        _messages: Option<Vec<Message>>,
    ) {
        channel_event::handle_channel_delete(&ctx, &channel).await;
    }
}
