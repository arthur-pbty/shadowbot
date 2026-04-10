use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::{
    advanced_tools, ancien, autoconfiglog, boostembed, g2048, help, helpsetting, morpion, mp,
    perms_service, puissance4, rolemenu, suggestion, tempvoc, ticket, viewlogs,
};

pub async fn handle_interaction_create(ctx: &Context, interaction: &Interaction) {
    if let Interaction::Command(_) = interaction {
        if help::handle_slash_interaction(ctx, interaction).await {
            return;
        }
    }

    if let Interaction::Component(component) = interaction {
        if ancien::handle_component_interaction(ctx, component).await {
            return;
        }

        if autoconfiglog::handle_component_interaction(ctx, component).await {
            return;
        }

        if ticket::handle_component_interaction(ctx, component).await {
            return;
        }

        if suggestion::handle_component_interaction(ctx, component).await {
            return;
        }

        if boostembed::handle_component_interaction(ctx, component).await {
            return;
        }

        if tempvoc::handle_component_interaction(ctx, component).await {
            return;
        }

        if rolemenu::handle_component_interaction(ctx, component).await {
            return;
        }

        if morpion::handle_component_interaction(ctx, component).await {
            return;
        }

        if puissance4::handle_component_interaction(ctx, component).await {
            return;
        }

        if g2048::handle_component_interaction(ctx, component).await {
            return;
        }

        if help::handle_help_component(ctx, component).await {
            return;
        }

        if helpsetting::handle_component_interaction(ctx, component).await {
            return;
        }

        if mp::handle_mp_component(ctx, component).await {
            return;
        }

        if perms_service::handle_allperms_component(ctx, component).await {
            return;
        }

        if viewlogs::handle_viewlogs_button(ctx, component).await {
            return;
        }

        let _ = advanced_tools::handle_component_interaction(ctx, component).await;
        return;
    }

    if let Interaction::Modal(modal) = interaction {
        if ancien::handle_modal_interaction(ctx, modal).await {
            return;
        }

        if ticket::handle_modal_interaction(ctx, modal).await {
            return;
        }

        if suggestion::handle_modal_interaction(ctx, modal).await {
            return;
        }

        if boostembed::handle_modal_interaction(ctx, modal).await {
            return;
        }

        if tempvoc::handle_modal_interaction(ctx, modal).await {
            return;
        }

        if rolemenu::handle_modal_interaction(ctx, modal).await {
            return;
        }

        let _ = advanced_tools::handle_modal_interaction(ctx, modal).await;
    }
}
