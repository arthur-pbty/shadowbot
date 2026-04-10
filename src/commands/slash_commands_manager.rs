// Gestion centralisée des slash commands
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::commands::{autopublish, showpics, suggestion, tempvoc, ticket, tickets};

pub async fn handle_slash_commands(ctx: &Context, interaction: &Interaction) -> bool {
    if ticket::handle_slash_interaction(ctx, interaction).await {
        return true;
    }

    if tickets::handle_slash_interaction(ctx, interaction).await {
        return true;
    }

    if showpics::handle_slash_interaction(ctx, interaction).await {
        return true;
    }

    if suggestion::handle_slash_interaction(ctx, interaction).await {
        return true;
    }

    if autopublish::handle_slash_interaction(ctx, interaction).await {
        return true;
    }

    if tempvoc::handle_slash_interaction(ctx, interaction).await {
        return true;
    }

    false
}
