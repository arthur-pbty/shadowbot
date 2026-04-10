use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_stage_instance_update(ctx: &Context, stage_instance: &StageInstance) {
    logs_service::on_stage_instance_update(ctx, stage_instance).await;
}
