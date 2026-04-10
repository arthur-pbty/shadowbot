use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{add_list_fields, theme_color};

pub async fn handle_serverlist(ctx: &Context, msg: &Message, _args: &[&str]) {
    let guilds = crate::commands::servertarget::guilds_sorted(ctx).await;
    let lines = guilds
        .iter()
        .enumerate()
        .map(|(index, (guild_id, name))| format!("{} · {} · `{}`", index + 1, name, guild_id.get()))
        .collect::<Vec<_>>();

    let mut embed = CreateEmbed::new()
        .title("Serveurs du bot")
        .color(theme_color(ctx).await);
    embed = add_list_fields(embed, &lines, "Guildes");

    let _ = msg
        .channel_id
        .send_message(&ctx.http, CreateMessage::new().embed(embed))
        .await;
}

pub struct ServerlistCommand;
pub static COMMAND_DESCRIPTOR: ServerlistCommand = ServerlistCommand;

impl crate::commands::command_contract::CommandSpec for ServerlistCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "serverlist",
            category: "info",
            params: "aucun",
            description: "Liste les serveurs ou le bot est present.",
            examples: &["+serverlist", "+sls", "+help serverlist"],
            default_aliases: &["sls"],
            allow_in_dm: false,
            default_permission: 0,
        }
    }
}
