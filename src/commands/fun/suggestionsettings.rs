use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_suggestionsettings_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::suggestion::handle_suggestionsettings(ctx, msg, args).await;
}

pub struct SuggestionsettingsCommand;
pub static COMMAND_DESCRIPTOR: SuggestionsettingsCommand = SuggestionsettingsCommand;

impl crate::commands::command_contract::CommandSpec for SuggestionsettingsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "suggestionsettings",
            category: "fun",
            params: "aucun",
            description: "Ouvre le panneau de configuration des suggestions du serveur.",
            examples: &["+suggestionsettings", "+sgset", "+help suggestionsettings"],
            default_aliases: &["sgset"],
            allow_in_dm: false,
            default_permission: 0,
        }
    }
}
