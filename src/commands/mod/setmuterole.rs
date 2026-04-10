use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::automod_service::pool;
use crate::commands::common::{parse_role, send_embed};
use crate::db;

pub async fn handle_set_muterole(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(raw_role) = args.first() else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Set MuteRole")
                .description("Usage: +setmuterole <@role/ID/nom>")
                .color(0xED4245),
        )
        .await;
        return;
    };

    let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await else {
        return;
    };

    let Some(role) = parse_role(&guild, raw_role) else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    if db::set_mute_role(
        &pool,
        bot_id,
        guild_id.get() as i64,
        Some(role.id.get() as i64),
    )
    .await
    .is_err()
    {
        return;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("MuteRole")
            .description(format!("Role muet defini sur <@&{}>.", role.id.get()))
            .color(0x57F287),
    )
    .await;
}

pub struct SetMuteRoleCommand;
pub static COMMAND_DESCRIPTOR: SetMuteRoleCommand = SetMuteRoleCommand;

impl crate::commands::command_contract::CommandSpec for SetMuteRoleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "setmuterole",
            category: "mod",
            params: "<@role/ID/nom>",
            description: "Definit le role utilise pour le mute lorsque le mode timeout est desactive.",
            examples: &["+setmuterole @Muted", "+help setmuterole"],
            default_aliases: &["smr"],
            allow_in_dm: false,
            default_permission: 7,
        }
    }
}
