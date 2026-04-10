use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::{advanced_tools, help, mp, perms_service, suggestion, tempvoc, ticket};

pub async fn handle_interaction_create(ctx: &Context, interaction: &Interaction) {
    if let Interaction::Command(_) = interaction {
        if help::handle_slash_interaction(ctx, interaction).await {
            return;
        }
    }

    if let Interaction::Component(component) = interaction {
        if ticket::handle_component_interaction(ctx, component).await {
            return;
        }

        if suggestion::handle_component_interaction(ctx, component).await {
            return;
        }

        if tempvoc::handle_component_interaction(ctx, component).await {
            return;
        }

        if help::handle_help_component(ctx, component).await {
            return;
        }

        if mp::handle_mp_component(ctx, component).await {
            return;
        }

        if perms_service::handle_allperms_component(ctx, component).await {
            return;
        }

        let _ = advanced_tools::handle_component_interaction(ctx, component).await;
        return;
    }

    if let Interaction::Modal(modal) = interaction {
        if ticket::handle_modal_interaction(ctx, modal).await {
            return;
        }

        if suggestion::handle_modal_interaction(ctx, modal).await {
            return;
        }

        if tempvoc::handle_modal_interaction(ctx, modal).await {
            return;
        }

        let _ = advanced_tools::handle_modal_interaction(ctx, modal).await;
    }
}


