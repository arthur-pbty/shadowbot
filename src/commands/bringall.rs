use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_bringall(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_bringall(ctx, msg, args).await;
}

pub struct BringAllCommand;
pub static COMMAND_DESCRIPTOR: BringAllCommand = BringAllCommand;

impl crate::commands::command_contract::CommandSpec for BringAllCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "bringall",
            command: "bringall",
            category: "admin",
            params: "[salon_vocal_destination]",
            summary: "Rassemble tous les vocaux",
            description: "Deplace tous les membres actuellement en vocal vers un salon cible.",
            examples: &["+bringall #Event", "+bringall"],
            alias_source_key: "bringall",
            default_aliases: &["ball", "vbring"],
        }
    }
}
