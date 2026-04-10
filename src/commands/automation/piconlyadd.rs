use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_piconlyadd_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::piconly::handle_piconlyadd(ctx, msg, args).await;
}

pub struct PiconlyaddCommand;
pub static COMMAND_DESCRIPTOR: PiconlyaddCommand = PiconlyaddCommand;

impl crate::commands::command_contract::CommandSpec for PiconlyaddCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "piconlyadd",
            category: "automation",
            params: "[#salon]",
            description: "Ajoute un salon selfie (photos uniquement).",
            examples: &["+piconlyadd", "+piconlyadd #selfie", "+help piconlyadd"],
            default_aliases: &["selfieadd"],
            allow_in_dm: false,
            default_permission: 6,
        }
    }
}
