use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::advanced_tools;
use crate::commands::common::{send_embed, theme_color};

pub async fn handle_backup(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    if args.is_empty() {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Backup")
                .description("Utilisation: +backup <serveur/emoji> <nom> | list/delete/load")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let mut action = "create";
    let mut index = 0usize;
    if let Some(first) = args.first() {
        match first.to_lowercase().as_str() {
            "list" | "ls" => {
                action = "list";
                index = 1;
            }
            "delete" | "del" | "rm" => {
                action = "delete";
                index = 1;
            }
            "load" => {
                action = "load";
                index = 1;
            }
            _ => {}
        }
    }

    let Some(kind_raw) = args.get(index) else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Backup")
                .description("Type invalide: utilise serveur ou emoji.")
                .color(0xED4245),
        )
        .await;
        return;
    };

    let Some(kind) = advanced_tools::backup_kind_from_input(kind_raw) else {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Backup")
                .description("Type invalide: utilise serveur ou emoji.")
                .color(0xED4245),
        )
        .await;
        return;
    };

    match action {
        "list" => {
            advanced_tools::backup_list(ctx, msg, guild_id, kind).await;
        }
        "delete" => {
            let Some(name) = args
                .get(index + 1)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            else {
                send_embed(
                    ctx,
                    msg,
                    CreateEmbed::new()
                        .title("Backup")
                        .description("Tu dois preciser un nom de backup.")
                        .color(0xED4245),
                )
                .await;
                return;
            };
            advanced_tools::backup_delete(ctx, msg, guild_id, kind, name).await;
        }
        "load" => {
            let Some(name) = args
                .get(index + 1)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            else {
                send_embed(
                    ctx,
                    msg,
                    CreateEmbed::new()
                        .title("Backup")
                        .description("Tu dois preciser un nom de backup.")
                        .color(0xED4245),
                )
                .await;
                return;
            };
            advanced_tools::backup_load(ctx, msg, guild_id, kind, name).await;
        }
        _ => {
            let Some(name) = args
                .get(index + 1)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            else {
                send_embed(
                    ctx,
                    msg,
                    CreateEmbed::new()
                        .title("Backup")
                        .description("Tu dois preciser un nom de backup.")
                        .color(0xED4245),
                )
                .await;
                return;
            };

            match advanced_tools::backup_create(ctx, guild_id, kind, name).await {
                Ok(()) => {
                    send_embed(
                        ctx,
                        msg,
                        CreateEmbed::new()
                            .title("Backup")
                            .description(format!("Backup `{}` de type `{}` creee.", name, kind))
                            .color(theme_color(ctx).await),
                    )
                    .await;
                }
                Err(err) => {
                    send_embed(
                        ctx,
                        msg,
                        CreateEmbed::new()
                            .title("Backup")
                            .description(format!("Erreur: {}", err))
                            .color(0xED4245),
                    )
                    .await;
                }
            }
        }
    }
}

pub struct BackupCommand;
pub static COMMAND_DESCRIPTOR: BackupCommand = BackupCommand;

impl crate::commands::command_contract::CommandSpec for BackupCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "backup",
            category: "outils",
            params: "<serveur/emoji> <nom> | list/delete/load",
            summary: "Gere les backups serveur et emojis",
            description: "Cree, liste, supprime et recharge des backups serveur ou emojis.",
            examples: &[
                "+backup serveur prod_1",
                "+backup list serveur",
                "+backup load emoji nightly",
            ],
            default_aliases: &["bkp"],
            default_permission: 8,
        }
    }
}
