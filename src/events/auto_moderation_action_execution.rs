use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_auto_moderation_action_execution(ctx: &Context, execution: &ActionExecution) {
    logs_service::on_auto_moderation_action_execution(ctx, execution).await;
}
