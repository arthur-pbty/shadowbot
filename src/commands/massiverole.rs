use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;

pub async fn handle_massiverole(ctx: &Context, msg: &Message, args: &[&str]) {
    advanced_tools::handle_massive_role(ctx, msg, args, true).await;
}

pub struct MassiveRoleCommand;
pub static COMMAND_DESCRIPTOR: MassiveRoleCommand = MassiveRoleCommand;

impl crate::commands::command_contract::CommandSpec for MassiveRoleCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "massiverole",
            command: "massiverole",
            category: "admin",
            params: "<role_cible> [role_filtre]",
            summary: "Ajoute un role en masse",
            description: "Ajoute un role a tous les membres ou a ceux qui ont deja un role filtre.",
            examples: &["+massiverole @VIP", "+massiverole @VIP @Membres"],
            alias_source_key: "massiverole",
            default_aliases: &["mrole", "mr"],
        }
    }
}
