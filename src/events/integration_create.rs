use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_integration_create(ctx: &Context, integration: &Integration) {
    logs_service::on_integration_create(ctx, integration).await;
}
