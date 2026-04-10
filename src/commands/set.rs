use serenity::builder::{CreateAttachment, CreateEmbed, EditProfile};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::commands::perms_service;

pub async fn handle_set(ctx: &Context, msg: &Message, args: &[&str]) {
    if args
        .first()
        .map(|a| a.eq_ignore_ascii_case("perm"))
        .unwrap_or(false)
    {
        perms_service::handle_set_perm(ctx, msg, args).await;
        return;
    }

    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+set name|pic|banner|profil ...`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let sub = args[0].to_lowercase();

    match sub.as_str() {
        "name" => {
            if args.len() < 2 {
                let embed = CreateEmbed::new()
                    .title("Erreur")
                    .description("Usage: `+set name <nom>`")
                    .color(0xED4245);
                send_embed(ctx, msg, embed).await;
                return;
            }

            let name = args[1..].join(" ");
            let mut me = match ctx.http.get_current_user().await {
                Ok(user) => user,
                Err(err) => {
                    let embed = CreateEmbed::new()
                        .title("Erreur")
                        .description(format!("Impossible de charger le profil bot: {}", err))
                        .color(0xED4245);
                    send_embed(ctx, msg, embed).await;
                    return;
                }
            };

            let result = me
                .edit(&ctx.http, EditProfile::new().username(name.clone()))
                .await;
            let embed = match result {
                Ok(_) => CreateEmbed::new()
                    .title("Profil mis à jour")
                    .description(format!("Nom défini sur: {}", name))
                    .color(0x57F287),
                Err(err) => CreateEmbed::new()
                    .title("Erreur")
                    .description(format!("Impossible de modifier le nom: {}", err))
                    .color(0xED4245),
            };
            send_embed(ctx, msg, embed).await;
        }
        "pic" => {
            if args.len() < 2 {
                let embed = CreateEmbed::new()
                    .title("Erreur")
                    .description("Usage: `+set pic <lien_image>`")
                    .color(0xED4245);
                send_embed(ctx, msg, embed).await;
                return;
            }

            let Ok(attachment) = CreateAttachment::url(&ctx.http, args[1]).await else {
                let embed = CreateEmbed::new()
                    .title("Erreur")
                    .description("Impossible de télécharger l'image.")
                    .color(0xED4245);
                send_embed(ctx, msg, embed).await;
                return;
            };

            let mut me = match ctx.http.get_current_user().await {
                Ok(user) => user,
                Err(err) => {
                    let embed = CreateEmbed::new()
                        .title("Erreur")
                        .description(format!("Impossible de charger le profil bot: {}", err))
                        .color(0xED4245);
                    send_embed(ctx, msg, embed).await;
                    return;
                }
            };

            let result = me
                .edit(&ctx.http, EditProfile::new().avatar(&attachment))
                .await;
            let embed = match result {
                Ok(_) => CreateEmbed::new()
                    .title("Profil mis à jour")
                    .description("Photo de profil modifiée.")
                    .color(0x57F287),
                Err(err) => CreateEmbed::new()
                    .title("Erreur")
                    .description(format!("Impossible de modifier la photo: {}", err))
                    .color(0xED4245),
            };
            send_embed(ctx, msg, embed).await;
        }
        "banner" => {
            if args.len() < 2 {
                let embed = CreateEmbed::new()
                    .title("Erreur")
                    .description("Usage: `+set banner <lien_image>`")
                    .color(0xED4245);
                send_embed(ctx, msg, embed).await;
                return;
            }

            let Ok(attachment) = CreateAttachment::url(&ctx.http, args[1]).await else {
                let embed = CreateEmbed::new()
                    .title("Erreur")
                    .description("Impossible de télécharger l'image.")
                    .color(0xED4245);
                send_embed(ctx, msg, embed).await;
                return;
            };

            let mut me = match ctx.http.get_current_user().await {
                Ok(user) => user,
                Err(err) => {
                    let embed = CreateEmbed::new()
                        .title("Erreur")
                        .description(format!("Impossible de charger le profil bot: {}", err))
                        .color(0xED4245);
                    send_embed(ctx, msg, embed).await;
                    return;
                }
            };

            let result = me
                .edit(&ctx.http, EditProfile::new().banner(&attachment))
                .await;
            let embed = match result {
                Ok(_) => CreateEmbed::new()
                    .title("Profil mis à jour")
                    .description("Bannière modifiée.")
                    .color(0x57F287),
                Err(err) => CreateEmbed::new()
                    .title("Erreur")
                    .description(format!("Impossible de modifier la bannière: {}", err))
                    .color(0xED4245),
            };
            send_embed(ctx, msg, embed).await;
        }
        "profil" => {
            let raw = args[1..].join(" ");
            let parts: Vec<&str> = raw.split(";;").map(|s| s.trim()).collect();

            if parts.len() != 3 {
                let embed = CreateEmbed::new()
                    .title("Erreur")
                    .description("Usage: `+set profil <nom> ;; <lien_pic> ;; <lien_banner>`")
                    .color(0xED4245);
                send_embed(ctx, msg, embed).await;
                return;
            }

            let mut builder = EditProfile::new();

            if !parts[0].is_empty() {
                builder = builder.username(parts[0].to_string());
            }

            if !parts[1].is_empty() {
                match CreateAttachment::url(&ctx.http, parts[1]).await {
                    Ok(avatar) => builder = builder.avatar(&avatar),
                    Err(_) => {
                        let embed = CreateEmbed::new()
                            .title("Erreur")
                            .description("Image avatar invalide dans `+set profil`. ")
                            .color(0xED4245);
                        send_embed(ctx, msg, embed).await;
                        return;
                    }
                }
            }

            if !parts[2].is_empty() {
                match CreateAttachment::url(&ctx.http, parts[2]).await {
                    Ok(banner) => builder = builder.banner(&banner),
                    Err(_) => {
                        let embed = CreateEmbed::new()
                            .title("Erreur")
                            .description("Image bannière invalide dans `+set profil`. ")
                            .color(0xED4245);
                        send_embed(ctx, msg, embed).await;
                        return;
                    }
                }
            }

            let mut me = match ctx.http.get_current_user().await {
                Ok(user) => user,
                Err(err) => {
                    let embed = CreateEmbed::new()
                        .title("Erreur")
                        .description(format!("Impossible de charger le profil bot: {}", err))
                        .color(0xED4245);
                    send_embed(ctx, msg, embed).await;
                    return;
                }
            };

            let result = me.edit(&ctx.http, builder).await;
            let embed = match result {
                Ok(_) => CreateEmbed::new()
                    .title("Profil mis à jour")
                    .description("Nom, avatar et bannière traités.")
                    .color(0x57F287),
                Err(err) => CreateEmbed::new()
                    .title("Erreur")
                    .description(format!("Impossible de modifier le profil: {}", err))
                    .color(0xED4245),
            };
            send_embed(ctx, msg, embed).await;
        }
        _ => {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description("Sous-commande inconnue. Utilise `name`, `pic`, `banner` ou `profil`.")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
        }
    }
}

pub struct SetCommand;
pub static COMMAND_DESCRIPTOR: SetCommand = SetCommand;

impl crate::commands::command_contract::CommandSpec for SetCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            key: "set",
            command: "set",
            category: "profile",
            params: "name <nom> | pic <url> | banner <url> | profil <nom> ;; <url_pic> ;; <url_banner> | perm ...",
            summary: "Configure le profil du bot",
            description: "Modifie le nom, lavatar, la banniere ou des options avancees via les sous commandes.",
            examples: &["+set", "+st", "+help set"],
            alias_source_key: "set",
            default_aliases: &["cfg"],
        }
    }
}
