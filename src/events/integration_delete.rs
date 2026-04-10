use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_integration_delete(
    ctx: &Context,
    integration_id: IntegrationId,
    guild_id: GuildId,
    application_id: Option<ApplicationId>,
) {
    logs_service::on_integration_delete(ctx, integration_id, guild_id, application_id).await;
}
