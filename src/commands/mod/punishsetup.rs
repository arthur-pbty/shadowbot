use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_punishsetup_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::punish::handle_punishsetup(ctx, msg, args).await;
}

pub struct PunishsetupCommand;
pub static COMMAND_DESCRIPTOR: PunishsetupCommand = PunishsetupCommand;

impl crate::commands::command_contract::CommandSpec for PunishsetupCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "punishsetup",
            category: "mod",
            params: "aucun",
            description: "Recharge les regles Punish par defaut.",
            examples: &["+punishsetup", "+help punishsetup"],
            default_aliases: &["pnsetup"],
            allow_in_dm: false,
            default_permission: 7,
        }
    }
}
