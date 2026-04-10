use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_unmassiverole(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_massive_role(ctx, msg, args, false).await;
}

pub struct UnMassiveRoleCommand;
pub static COMMAND_DESCRIPTOR: UnMassiveRoleCommand = UnMassiveRoleCommand;

impl crate::commands::command_contract::CommandSpec for UnMassiveRoleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "unmassiverole",
            command: "unmassiverole",
            category: "admin",
            params: "<role_cible> [role_filtre]",
            summary: "Retire un role en masse",
            description: "Retire un role a tous les membres ou a ceux qui ont un role filtre.",
            examples: &["+unmassiverole @VIP", "+unmassiverole @VIP @Membres"],
            alias_source_key: "unmassiverole",
            default_aliases: &["umrole", "umr"],
            default_permission: 8,
        }
    }
}
