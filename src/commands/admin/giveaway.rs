use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_giveaway(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_giveaway(ctx, msg, args).await;
}

pub struct GiveawayCommand;
pub static COMMAND_DESCRIPTOR: GiveawayCommand = GiveawayCommand;

impl crate::commands::command_contract::CommandSpec for GiveawayCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "giveaway",
            command: "giveaway",
            category: "admin",
            params: "aucun",
            summary: "Ouvre un menu de creation de giveaway",
            description: "Affiche une interface rapide pour initier un giveaway depuis le salon courant.",
            examples: &["+giveaway"],
            alias_source_key: "giveaway",
            default_aliases: &["gstart", "gw"],
            default_permission: 8,
        }
    }
}
