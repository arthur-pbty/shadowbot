use chrono::{Datelike, NaiveDate, Utc};
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_halloween(ctx: &Context, msg: &Message, _args: &[&str]) {
    let today = Utc::now().date_naive();
    let mut year = today.year();
    let mut target = NaiveDate::from_ymd_opt(year, 10, 31).unwrap_or(today);

    if today > target {
        year += 1;
        target = NaiveDate::from_ymd_opt(year, 10, 31).unwrap_or(today);
    }

    let days = (target - today).num_days();

    let embed = CreateEmbed::new()
        .title("Halloween")
        .description(format!("Il reste **{}** jour(s) avant Halloween.", days))
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct HalloweenCommand;
pub static COMMAND_DESCRIPTOR: HalloweenCommand = HalloweenCommand;

impl crate::commands::command_contract::CommandSpec for HalloweenCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "halloween",
            category: "game",
            params: "aucun",
            description: "Calcule le nombre de jours jusqu'a Halloween.",
            examples: &["+halloween"],
            default_aliases: &["spooky"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
