use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::{ancien, logs_service};

pub async fn handle_member_addition(ctx: &Context, new_member: &Member) {
    logs_service::on_member_join(ctx, new_member.guild_id, &new_member.user).await;
    ancien::maybe_assign_ancien_role(ctx, new_member.guild_id, new_member.user.id).await;
}
