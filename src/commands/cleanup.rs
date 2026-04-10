use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_cleanup(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_cleanup(ctx, msg, args).await;
}

pub struct CleanupCommand;
pub static COMMAND_DESCRIPTOR: CleanupCommand = CleanupCommand;

impl crate::commands::command_contract::CommandSpec for CleanupCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "cleanup",
            command: "cleanup",
            category: "admin",
            params: "<salon_vocal>",
            summary: "Vide un salon vocal",
            description: "Deconnecte tous les utilisateurs presents dans un salon vocal cible.",
            examples: &["+cleanup #General"],
            alias_source_key: "cleanup",
            default_aliases: &["vclean", "vcleanup"],
        }
    }
}
