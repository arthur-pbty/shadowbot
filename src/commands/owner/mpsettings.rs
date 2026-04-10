use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_mpsettings_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::mp::handle_mpsettings(ctx, msg, args).await;
}

pub struct MpsettingsCommand;
pub static COMMAND_DESCRIPTOR: MpsettingsCommand = MpsettingsCommand;

impl crate::commands::command_contract::CommandSpec for MpsettingsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "mpsettings",
            category: "owner",
            params: "[on|off]",
            description: "Affiche ou modifie l'etat global de l'envoi de MP par le bot.",
            examples: &["+mpsettings", "+mpsettings off", "+help mpsettings"],
            default_aliases: &["mpset"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
