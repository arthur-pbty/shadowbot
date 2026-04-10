use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::events::{
    channel_create, channel_delete, channel_update, guild_create, guild_member_addition,
    guild_member_removal, guild_member_update, interaction_create, message, message_delete,
    message_update, ready, role_create, role_delete, role_update, voice_state_update,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::handle_ready(&ctx, &ready).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        message::handle_message(&ctx, &msg).await;
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: Option<bool>) {
        guild_create::handle_guild_create(&ctx, &guild).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create::handle_interaction_create(&ctx, &interaction).await;
    }

    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        deleted_message_id: MessageId,
        guild_id: Option<GuildId>,
    ) {
        message_delete::handle_message_delete(&ctx, channel_id, deleted_message_id, guild_id).await;
    }

    async fn message_update(
        &self,
        ctx: Context,
        old_if_available: Option<Message>,
        new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        message_update::handle_message_update(&ctx, old_if_available, new, &event).await;
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        voice_state_update::handle_voice_state_update(&ctx, old, &new).await;
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        guild_member_addition::handle_member_addition(&ctx, &new_member).await;
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        _member_data_if_available: Option<Member>,
    ) {
        guild_member_removal::handle_member_removal(&ctx, guild_id, &user).await;
    }

    async fn guild_member_update(
        &self,
        ctx: Context,
        old_if_available: Option<Member>,
        new: Option<Member>,
        event: GuildMemberUpdateEvent,
    ) {
        guild_member_update::handle_member_update(&ctx, old_if_available, new, &event).await;
    }

    async fn guild_role_create(&self, ctx: Context, new: Role) {
        role_create::handle_role_create(&ctx, &new).await;
    }

    async fn guild_role_update(
        &self,
        ctx: Context,
        old_data_if_available: Option<Role>,
        new: Role,
    ) {
        role_update::handle_role_update(&ctx, old_data_if_available, &new).await;
    }

    async fn guild_role_delete(
        &self,
        ctx: Context,
        guild_id: GuildId,
        removed_role_id: RoleId,
        removed_role_data_if_available: Option<Role>,
    ) {
        role_delete::handle_role_delete(
            &ctx,
            guild_id,
            removed_role_id,
            removed_role_data_if_available,
        )
        .await;
    }

    async fn channel_create(&self, ctx: Context, guild_channel: GuildChannel) {
        channel_create::handle_channel_create(&ctx, &guild_channel).await;
    }

    async fn channel_update(&self, ctx: Context, old: Option<GuildChannel>, new: GuildChannel) {
        channel_update::handle_channel_update(&ctx, old, &new).await;
    }

    async fn channel_delete(
        &self,
        ctx: Context,
        channel: GuildChannel,
        _messages: Option<Vec<Message>>,
    ) {
        channel_delete::handle_channel_delete(&ctx, &channel).await;
    }
}
