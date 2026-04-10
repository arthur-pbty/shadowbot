use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_auto_moderation_rule_delete(ctx: &Context, rule: &Rule) {
    logs_service::on_auto_moderation_rule_delete(ctx, rule).await;
}
