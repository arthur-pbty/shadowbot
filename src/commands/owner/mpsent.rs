use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_mpsent_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::mp::handle_mpsent(ctx, msg, args).await;
}

pub struct MpsentCommand;
pub static COMMAND_DESCRIPTOR: MpsentCommand = MpsentCommand;

impl crate::commands::command_contract::CommandSpec for MpsentCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "mpsent",
            category: "owner",
            params: "[page]",
            description: "Affiche l'historique des MP envoyes par le bot.",
            examples: &["+mpsent", "+mpsent 2", "+help mpsent"],
            default_aliases: &["mps"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
