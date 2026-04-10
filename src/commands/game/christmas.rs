use chrono::{Datelike, NaiveDate, Utc};
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

pub async fn handle_christmas(ctx: &Context, msg: &Message, _args: &[&str]) {
    let today = Utc::now().date_naive();
    let mut year = today.year();
    let mut target = NaiveDate::from_ymd_opt(year, 12, 25).unwrap_or(today);

    if today > target {
        year += 1;
        target = NaiveDate::from_ymd_opt(year, 12, 25).unwrap_or(today);
    }

    let days = (target - today).num_days();

    let embed = CreateEmbed::new()
        .title("Christmas")
        .description(format!("Il reste **{}** jour(s) avant Noel.", days))
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct ChristmasCommand;
pub static COMMAND_DESCRIPTOR: ChristmasCommand = ChristmasCommand;

impl crate::commands::command_contract::CommandSpec for ChristmasCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "christmas",
            category: "game",
            params: "aucun",
            description: "Calcule le nombre de jours jusqu'a Noel.",
            examples: &["+christmas"],
            default_aliases: &["xmas"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
