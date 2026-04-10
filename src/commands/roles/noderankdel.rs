use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_noderankdel_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::noderank::handle_noderankdel(ctx, msg, args).await;
}

pub struct NoderankdelCommand;
pub static COMMAND_DESCRIPTOR: NoderankdelCommand = NoderankdelCommand;

impl crate::commands::command_contract::CommandSpec for NoderankdelCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "noderankdel",
            category: "roles",
            params: "<@role/ID/nom>",
            description: "Retire un role de la liste NoDeRank.",
            examples: &["+noderankdel @VIP", "+help noderankdel"],
            default_aliases: &["ndrd"],
            allow_in_dm: false,
            default_permission: 7,
        }
    }
}
