use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_reaction_remove_emoji(ctx: &Context, removed_reactions: &Reaction) {
    logs_service::on_reaction_remove_emoji(ctx, removed_reactions).await;
}
