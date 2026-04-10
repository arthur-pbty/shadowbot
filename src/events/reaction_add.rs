use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_reaction_add(ctx: &Context, add_reaction: &Reaction) {
    logs_service::on_reaction_add(ctx, add_reaction).await;
}
