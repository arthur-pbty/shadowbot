use serenity::builder::{CreateAttachment, CreateEmbed, EditProfile};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::commands::perms_helpers::{
    ensure_owner, get_pool, normalize_command_name, parse_user_or_role,
};
use crate::db::{grant_command_access, grant_perm_level};

pub async fn handle_setperm(ctx: &Context, msg: &Message, args: &[&str]) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    if args.len() < 2 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+setperm <permission/commande> <role/membre>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let Some((scope_type, scope_id)) = parse_user_or_role(args[1]) else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Role/membre invalide.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let bot_id = ctx.cache.current_user().id;
    let Some(pool) = get_pool(ctx).await else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("DB indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    if let Ok(level) = args[0].parse::<u8>() {
        if level > 9 {
            let embed = CreateEmbed::new()
                .title("Erreur")
                .description("Permission invalide (0..9).")
                .color(0xED4245);
            send_embed(ctx, msg, embed).await;
            return;
        }

        let _ = grant_perm_level(&pool, bot_id, scope_type, scope_id, level).await;
        let who = if scope_type == "role" {
            format!("<@&{}>", scope_id)
        } else {
            format!("<@{}>", scope_id)
        };
        let embed = CreateEmbed::new()
            .title("Permission attribuee")
            .description(format!("{} recoit la permission `{}`", who, level))
            .color(0x57F287);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let command = normalize_command_name(args[0]);
    let _ = grant_command_access(&pool, bot_id, scope_type, scope_id, &command).await;

    let who = if scope_type == "role" {
        format!("<@&{}>", scope_id)
    } else {
        format!("<@{}>", scope_id)
    };

    let embed = CreateEmbed::new()
        .title("Acces commande attribue")
        .description(format!("{} recoit l'acces direct a `{}`", who, command))
        .color(0x57F287);
    send_embed(ctx, msg, embed).await;
}

pub async fn handle_setname(ctx: &Context, msg: &Message, args: &[&str]) {
    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+setname <nom>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let name = args.join(" ");
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

pub async fn handle_setpic(ctx: &Context, msg: &Message, args: &[&str]) {
    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+setpic <lien_image>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let Ok(attachment) = CreateAttachment::url(&ctx.http, args[0]).await else {
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

pub async fn handle_setbanner(ctx: &Context, msg: &Message, args: &[&str]) {
    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+setbanner <lien_image>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let Ok(attachment) = CreateAttachment::url(&ctx.http, args[0]).await else {
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

pub async fn handle_setprofil(ctx: &Context, msg: &Message, args: &[&str]) {
    let raw = args.join(" ");
    let parts: Vec<&str> = raw.split(";;").map(|s| s.trim()).collect();

    if parts.len() != 3 {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+setprofil <nom> ;; <lien_pic> ;; <lien_banner>`")
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
                    .description("Image avatar invalide dans `+setprofil`.")
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
                    .description("Image bannière invalide dans `+setprofil`.")
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

