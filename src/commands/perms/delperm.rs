use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_delperm_command(ctx: &Context, msg: &Message, args: &[&str]) {
    crate::commands::del::handle_del(ctx, msg, args).await;
}

pub struct DelpermCommand;
pub static COMMAND_DESCRIPTOR: DelpermCommand = DelpermCommand;

impl crate::commands::command_contract::CommandSpec for DelpermCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "delperm",
            category: "perms",
            params: "<@&role/@membre/ID>",
            description: "Supprime les permissions ACL associees a un role ou utilisateur.",
            examples: &["+delperm @Role", "+help delperm"],
            default_aliases: &["dlp"],
            allow_in_dm: false,
            default_permission: 7,
        }
    }
}
