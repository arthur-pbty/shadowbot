use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_unalias_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::alias::handle_unalias(ctx, msg, args).await;
}

pub struct UnaliasCommand;
pub static COMMAND_DESCRIPTOR: UnaliasCommand = UnaliasCommand;

impl crate::commands::command_contract::CommandSpec for UnaliasCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "unalias",
            category: "perms",
            params: "<alias>",
            description: "Supprime un alias de commande en base.",
            examples: &["+unalias m", "+help unalias"],
            default_aliases: &["uals"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
