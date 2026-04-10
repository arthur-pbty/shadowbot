use crate::commands::moderation_tools;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn handle_banlist(ctx: &Context, msg: &Message) {
    moderation_tools::handle_banlist(ctx, msg).await;
}
pub struct BanlistCommand;
pub static COMMAND_DESCRIPTOR: BanlistCommand = BanlistCommand;
impl crate::commands::command_contract::CommandSpec for BanlistCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "banlist",
            command: "banlist",
            category: "admin",
            params: "aucun",
            summary: "Liste les bans",
            description: "Affiche la liste des bannissements en cours.",
            examples: &["+banlist"],
            alias_source_key: "banlist",
            default_aliases: &["bls"],
            default_permission: 8,
        }
    }
}
