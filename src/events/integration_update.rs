use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_integration_update(ctx: &Context, integration: &Integration) {
    logs_service::on_integration_update(ctx, integration).await;
}
