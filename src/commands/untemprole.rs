use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_untemprole(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_untemprole(ctx, msg, args).await;
}

pub struct UnTempRoleCommand;
pub static COMMAND_DESCRIPTOR: UnTempRoleCommand = UnTempRoleCommand;

impl crate::commands::command_contract::CommandSpec for UnTempRoleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "untemprole",
            command: "untemprole",
            category: "admin",
            params: "<membre> <role>",
            summary: "Retire un role temporaire",
            description: "Retire immediatement un role temporaire et desactive son expiration.",
            examples: &["+untemprole @User @VIP"],
            alias_source_key: "untemprole",
            default_aliases: &["untrole", "deltrole"],
        }
    }
}
