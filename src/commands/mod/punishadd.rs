use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_punishadd_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::punish::handle_punishadd(ctx, msg, args).await;
}

pub struct PunishaddCommand;
pub static COMMAND_DESCRIPTOR: PunishaddCommand = PunishaddCommand;

impl crate::commands::command_contract::CommandSpec for PunishaddCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "punishadd",
            category: "mod",
            params: "<nombre> <duree> <sanction> [duree]",
            description: "Ajoute ou met a jour une regle Punish.",
            examples: &["+punishadd 8 1h mute 30m", "+help punishadd"],
            default_aliases: &["pna"],
            allow_in_dm: false,
            default_permission: 7,
        }
    }
}
