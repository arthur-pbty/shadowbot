use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_endgiveaway_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::end::handle_endgiveaway(ctx, msg, args).await;
}

pub struct EndgiveawayCommand;
pub static COMMAND_DESCRIPTOR: EndgiveawayCommand = EndgiveawayCommand;

impl crate::commands::command_contract::CommandSpec for EndgiveawayCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "endgiveaway",
            category: "event",
            params: "<id_message>",
            description: "Termine instantanement un giveaway a partir de l'identifiant du message.",
            examples: &["+endgiveaway 123456789012345678", "+help endgiveaway"],
            default_aliases: &["gend"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
