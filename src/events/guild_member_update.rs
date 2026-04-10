use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::{ancien, logs_service};

pub async fn handle_member_update(
    ctx: &Context,
    old_if_available: Option<Member>,
    new: Option<Member>,
    event: &GuildMemberUpdateEvent,
) {
    if let (Some(old), Some(new_member)) = (old_if_available.clone(), new.clone()) {
        logs_service::on_member_roles_updated(
            ctx,
            new_member.guild_id,
            new_member.user.id,
            &old.roles,
            &new_member.roles,
        )
        .await;

        logs_service::on_boost_update(
            ctx,
            new_member.guild_id,
            new_member.user.id,
            old.premium_since,
            new_member.premium_since,
        )
        .await;

        ancien::maybe_assign_ancien_role(ctx, new_member.guild_id, new_member.user.id).await;
        return;
    }

    logs_service::on_boost_update(
        ctx,
        event.guild_id,
        event.user.id,
        None,
        event.premium_since,
    )
    .await;
}
