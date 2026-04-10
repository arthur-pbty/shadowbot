use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::activity::stop_rotation;
use crate::commands::common::send_embed;
use crate::db::DbPoolKey;

pub async fn handle_remove_activity(ctx: &Context, msg: &Message) {
    stop_rotation(ctx).await;
    ctx.set_activity(None);

    let bot_id = ctx.cache.current_user().id;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    if let Some(pool) = pool {
        let _ = crate::db::clear_bot_activity(&pool, bot_id).await;
    }

    let embed = CreateEmbed::new()
        .title("Activité supprimée")
        .description("L'activité du bot a été retirée.")
        .color(0x57F287);

    send_embed(ctx, msg, embed).await;
}

pub struct RemoveActivityCommand;
pub static COMMAND_DESCRIPTOR: RemoveActivityCommand = RemoveActivityCommand;

impl crate::commands::command_contract::CommandSpec for RemoveActivityCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "remove_activity",
            category: "bot",
            params: "aucun",
            description: "Arrete la rotation d activite et retire lactivite courante du bot.",
            examples: &["+remove activity", "+ry", "+help remove activity"],
            default_aliases: &["rma"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
