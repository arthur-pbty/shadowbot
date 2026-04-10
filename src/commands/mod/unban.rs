use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::moderation_sanction_helpers::parse_targets;

pub async fn handle_unban(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };
    if args.is_empty() {
        return;
    }

    let targets = parse_targets(args[0]).await;
    if targets.is_empty() {
        return;
    }

    let mut done = 0usize;
    for uid in &targets {
        if guild_id.unban(&ctx.http, *uid).await.is_ok() {
            done += 1;
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("UnBan")
            .description(format!("{} membre(s) unban.", done))
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct UnbanCommand;
pub static COMMAND_DESCRIPTOR: UnbanCommand = UnbanCommand;
impl crate::commands::command_contract::CommandSpec for UnbanCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "unban",
            category: "mod",
            params: "<@membre/ID[,..]>",
            description: "Unban un ou plusieurs membres.",
            examples: &["+unban @User"],
            default_aliases: &["ub"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
