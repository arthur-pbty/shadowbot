use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::automod_service::pool;
use crate::commands::common::send_embed;
use crate::db;

pub async fn handle_clear_badwords(ctx: &Context, msg: &Message, _args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let cleared = db::clear_badwords(&pool, bot_id, guild_id.get() as i64)
        .await
        .unwrap_or(0);

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Clear BadWords")
            .description(format!("{} mot(s) interdit(s) supprime(s).", cleared))
            .color(0x57F287),
    )
    .await;
}

pub struct ClearBadwordsCommand;
pub static COMMAND_DESCRIPTOR: ClearBadwordsCommand = ClearBadwordsCommand;

impl crate::commands::command_contract::CommandSpec for ClearBadwordsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "clearbadwords",
            category: "mod",
            params: "badwords",
            description: "Supprime l ensemble des mots interdits enregistres.",
            examples: &["+clearbadwords", "+help clearbadwords"],
            default_aliases: &["cbw"],
            allow_in_dm: false,
            default_permission: 7,
        }
    }
}
