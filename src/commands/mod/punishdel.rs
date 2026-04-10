use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_punishdel_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::punish::handle_punishdel(ctx, msg, args).await;
}

pub struct PunishdelCommand;
pub static COMMAND_DESCRIPTOR: PunishdelCommand = PunishdelCommand;

impl crate::commands::command_contract::CommandSpec for PunishdelCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "punishdel",
            category: "mod",
            params: "<numero>",
            description: "Supprime une regle Punish par son index.",
            examples: &["+punishdel 2", "+help punishdel"],
            default_aliases: &["pnd"],
            allow_in_dm: false,
            default_permission: 7,
        }
    }
}
