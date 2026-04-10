use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_noderankadd_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::noderank::handle_noderankadd(ctx, msg, args).await;
}

pub struct NoderankaddCommand;
pub static COMMAND_DESCRIPTOR: NoderankaddCommand = NoderankaddCommand;

impl crate::commands::command_contract::CommandSpec for NoderankaddCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "noderankadd",
            category: "roles",
            params: "<@role/ID/nom>",
            description: "Ajoute un role a la liste NoDeRank.",
            examples: &["+noderankadd @VIP", "+help noderankadd"],
            default_aliases: &["ndra"],
            allow_in_dm: false,
            default_permission: 7,
        }
    }
}
