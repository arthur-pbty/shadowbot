use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::events::{
    auto_moderation_action_execution, auto_moderation_rule_create, auto_moderation_rule_delete,
    auto_moderation_rule_update, channel_create, channel_delete, channel_pins_update,
    channel_update, guild_ban_addition, guild_ban_removal, guild_create, guild_integrations_update,
    guild_member_addition, guild_member_removal, guild_member_update, guild_scheduled_event_create,
    guild_scheduled_event_delete, guild_scheduled_event_update, guild_scheduled_event_user_add,
    guild_scheduled_event_user_remove, integration_create, integration_delete, integration_update,
    interaction_create, invite_create, invite_delete, message, message_delete, message_delete_bulk,
    message_update, reaction_add, reaction_remove, reaction_remove_all, reaction_remove_emoji,
    ready, role_create, role_delete, role_update, stage_instance_create, stage_instance_delete,
    stage_instance_update, thread_create, thread_delete, thread_list_sync, thread_member_update,
    thread_members_update, thread_update, voice_channel_status_update, voice_state_update,
    webhook_update,
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

    async fn message_delete_bulk(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        multiple_deleted_messages_ids: Vec<MessageId>,
        guild_id: Option<GuildId>,
    ) {
        message_delete_bulk::handle_message_delete_bulk(
            &ctx,
            channel_id,
            &multiple_deleted_messages_ids,
            guild_id,
        )
        .await;
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        reaction_add::handle_reaction_add(&ctx, &add_reaction).await;
    }

    async fn reaction_remove(&self, ctx: Context, removed_reaction: Reaction) {
        reaction_remove::handle_reaction_remove(&ctx, &removed_reaction).await;
    }

    async fn reaction_remove_all(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        removed_from_message_id: MessageId,
    ) {
        reaction_remove_all::handle_reaction_remove_all(&ctx, channel_id, removed_from_message_id)
            .await;
    }

    async fn reaction_remove_emoji(&self, ctx: Context, removed_reactions: Reaction) {
        reaction_remove_emoji::handle_reaction_remove_emoji(&ctx, &removed_reactions).await;
    }

    async fn channel_pins_update(&self, ctx: Context, pin: ChannelPinsUpdateEvent) {
        channel_pins_update::handle_channel_pins_update(&ctx, &pin).await;
    }

    async fn invite_create(&self, ctx: Context, data: InviteCreateEvent) {
        invite_create::handle_invite_create(&ctx, &data).await;
    }

    async fn invite_delete(&self, ctx: Context, data: InviteDeleteEvent) {
        invite_delete::handle_invite_delete(&ctx, &data).await;
    }

    async fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {
        guild_ban_addition::handle_guild_ban_addition(&ctx, guild_id, &banned_user).await;
    }

    async fn guild_ban_removal(&self, ctx: Context, guild_id: GuildId, unbanned_user: User) {
        guild_ban_removal::handle_guild_ban_removal(&ctx, guild_id, &unbanned_user).await;
    }

    async fn webhook_update(
        &self,
        ctx: Context,
        guild_id: GuildId,
        belongs_to_channel_id: ChannelId,
    ) {
        webhook_update::handle_webhook_update(&ctx, guild_id, belongs_to_channel_id).await;
    }

    async fn thread_create(&self, ctx: Context, thread: GuildChannel) {
        thread_create::handle_thread_create(&ctx, &thread).await;
    }

    async fn thread_update(&self, ctx: Context, old: Option<GuildChannel>, new: GuildChannel) {
        thread_update::handle_thread_update(&ctx, old, &new).await;
    }

    async fn thread_delete(
        &self,
        ctx: Context,
        thread: PartialGuildChannel,
        full_thread_data: Option<GuildChannel>,
    ) {
        thread_delete::handle_thread_delete(&ctx, &thread, full_thread_data.as_ref()).await;
    }

    async fn thread_list_sync(&self, ctx: Context, thread_list_sync_event: ThreadListSyncEvent) {
        thread_list_sync::handle_thread_list_sync(&ctx, &thread_list_sync_event).await;
    }

    async fn thread_member_update(&self, ctx: Context, thread_member: ThreadMember) {
        thread_member_update::handle_thread_member_update(&ctx, &thread_member).await;
    }

    async fn thread_members_update(&self, ctx: Context, event: ThreadMembersUpdateEvent) {
        thread_members_update::handle_thread_members_update(&ctx, &event).await;
    }

    async fn auto_moderation_rule_create(&self, ctx: Context, rule: Rule) {
        auto_moderation_rule_create::handle_auto_moderation_rule_create(&ctx, &rule).await;
    }

    async fn auto_moderation_rule_update(&self, ctx: Context, rule: Rule) {
        auto_moderation_rule_update::handle_auto_moderation_rule_update(&ctx, &rule).await;
    }

    async fn auto_moderation_rule_delete(&self, ctx: Context, rule: Rule) {
        auto_moderation_rule_delete::handle_auto_moderation_rule_delete(&ctx, &rule).await;
    }

    async fn auto_moderation_action_execution(&self, ctx: Context, execution: ActionExecution) {
        auto_moderation_action_execution::handle_auto_moderation_action_execution(&ctx, &execution)
            .await;
    }

    async fn stage_instance_create(&self, ctx: Context, stage_instance: StageInstance) {
        stage_instance_create::handle_stage_instance_create(&ctx, &stage_instance).await;
    }

    async fn stage_instance_update(&self, ctx: Context, stage_instance: StageInstance) {
        stage_instance_update::handle_stage_instance_update(&ctx, &stage_instance).await;
    }

    async fn stage_instance_delete(&self, ctx: Context, stage_instance: StageInstance) {
        stage_instance_delete::handle_stage_instance_delete(&ctx, &stage_instance).await;
    }

    async fn voice_channel_status_update(
        &self,
        ctx: Context,
        old: Option<String>,
        status: Option<String>,
        id: ChannelId,
        guild_id: GuildId,
    ) {
        voice_channel_status_update::handle_voice_channel_status_update(
            &ctx, old, status, id, guild_id,
        )
        .await;
    }

    async fn guild_scheduled_event_create(&self, ctx: Context, event: ScheduledEvent) {
        guild_scheduled_event_create::handle_guild_scheduled_event_create(&ctx, &event).await;
    }

    async fn guild_scheduled_event_update(&self, ctx: Context, event: ScheduledEvent) {
        guild_scheduled_event_update::handle_guild_scheduled_event_update(&ctx, &event).await;
    }

    async fn guild_scheduled_event_delete(&self, ctx: Context, event: ScheduledEvent) {
        guild_scheduled_event_delete::handle_guild_scheduled_event_delete(&ctx, &event).await;
    }

    async fn guild_scheduled_event_user_add(
        &self,
        ctx: Context,
        subscribed: GuildScheduledEventUserAddEvent,
    ) {
        guild_scheduled_event_user_add::handle_guild_scheduled_event_user_add(&ctx, &subscribed)
            .await;
    }

    async fn guild_scheduled_event_user_remove(
        &self,
        ctx: Context,
        unsubscribed: GuildScheduledEventUserRemoveEvent,
    ) {
        guild_scheduled_event_user_remove::handle_guild_scheduled_event_user_remove(
            &ctx,
            &unsubscribed,
        )
        .await;
    }

    async fn integration_create(&self, ctx: Context, integration: Integration) {
        integration_create::handle_integration_create(&ctx, &integration).await;
    }

    async fn integration_update(&self, ctx: Context, integration: Integration) {
        integration_update::handle_integration_update(&ctx, &integration).await;
    }

    async fn integration_delete(
        &self,
        ctx: Context,
        integration_id: IntegrationId,
        guild_id: GuildId,
        application_id: Option<ApplicationId>,
    ) {
        integration_delete::handle_integration_delete(
            &ctx,
            integration_id,
            guild_id,
            application_id,
        )
        .await;
    }

    async fn guild_integrations_update(&self, ctx: Context, guild_id: GuildId) {
        guild_integrations_update::handle_guild_integrations_update(&ctx, guild_id).await;
    }
}
