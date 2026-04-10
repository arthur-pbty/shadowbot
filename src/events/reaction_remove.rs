use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_reaction_remove(ctx: &Context, removed_reaction: &Reaction) {
    logs_service::on_reaction_remove(ctx, removed_reaction).await;
}
