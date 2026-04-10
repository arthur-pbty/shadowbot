use serenity::builder::{CreateChannel, CreateEmbed};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::commands::logs_command_helpers::set_log_channel;

const LOG_TYPES: &[(&str, &str)] = &[
    ("moderation", "modlog"),
    ("message", "messagelog"),
    ("voice", "voicelog"),
    ("boost", "boostlog"),
    ("role", "rolelog"),
    ("raid", "raidlog"),
    ("channel", "channellog"),
];

pub async fn handle_autoconfiglog(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let mut created = Vec::new();
    for (log_type, cmd) in LOG_TYPES {
        let name = format!("{}-logs", cmd.replace("log", ""));
        if let Ok(channel) = guild_id
            .create_channel(&ctx.http, CreateChannel::new(name).kind(ChannelType::Text))
            .await
        {
            set_log_channel(ctx, guild_id, log_type, Some(channel.id), true).await;
            created.push(format!("{} -> <#{}>", log_type, channel.id.get()));
        }
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("AutoConfigLog")
            .description(if created.is_empty() {
                "Aucun salon cree.".to_string()
            } else {
                created.join("\n")
            })
            .color(theme_color(ctx).await),
    )
    .await;
}

pub struct AutoconfiglogCommand;
pub static COMMAND_DESCRIPTOR: AutoconfiglogCommand = AutoconfiglogCommand;

impl crate::commands::command_contract::CommandSpec for AutoconfiglogCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "autoconfiglog",
            category: "logs",
            params: "aucun",
            summary: "Cree tous les salons de logs",
            description: "Cree automatiquement les salons de logs et les configure.",
            examples: &["+autoconfiglog"],
            default_aliases: &["acl"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
