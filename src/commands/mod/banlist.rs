use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_banlist(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let bans = guild_id
        .bans(&ctx.http, None, None)
        .await
        .unwrap_or_default();
    let desc = if bans.is_empty() {
        "Aucun ban en cours.".to_string()
    } else {
        bans.into_iter()
            .map(|ban| format!("- <@{}> ({})", ban.user.id.get(), ban.user.tag()))
            .collect::<Vec<_>>()
            .join("\n")
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("BanList")
            .description(desc)
            .color(theme_color(ctx).await),
    )
    .await;
}
pub struct BanlistCommand;
pub static COMMAND_DESCRIPTOR: BanlistCommand = BanlistCommand;
impl crate::commands::command_contract::CommandSpec for BanlistCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "banlist",
            category: "mod",
            params: "aucun",
            description: "Affiche la liste des bannissements en cours.",
            examples: &["+banlist"],
            default_aliases: &["bls"],
            allow_in_dm: false,
            default_permission: 5,
        }
    }
}
