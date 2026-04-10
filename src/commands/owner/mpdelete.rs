use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_mpdelete_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::mp::handle_mpdelete(ctx, msg, args).await;
}

pub struct MpdeleteCommand;
pub static COMMAND_DESCRIPTOR: MpdeleteCommand = MpdeleteCommand;

impl crate::commands::command_contract::CommandSpec for MpdeleteCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "mpdelete",
            category: "owner",
            params: "<id>",
            description: "Supprime un MP envoye precedemment a partir de son identifiant interne.",
            examples: &["+mpdelete 12", "+mpdel 12", "+help mpdelete"],
            default_aliases: &["mpdel"],
            allow_in_dm: false,
            default_permission: 9,
        }
    }
}
