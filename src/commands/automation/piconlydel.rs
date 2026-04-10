use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_piconlydel_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::piconly::handle_piconlydel(ctx, msg, args).await;
}

pub struct PiconlydelCommand;
pub static COMMAND_DESCRIPTOR: PiconlydelCommand = PiconlydelCommand;

impl crate::commands::command_contract::CommandSpec for PiconlydelCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "piconlydel",
            category: "automation",
            params: "[#salon]",
            description: "Retire un salon selfie (photos uniquement).",
            examples: &["+piconlydel", "+piconlydel #selfie", "+help piconlydel"],
            default_aliases: &["selfiedel"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
