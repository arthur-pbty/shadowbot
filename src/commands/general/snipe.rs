use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{discord_ts, send_embed, truncate_text};
use crate::db::{DbPoolKey, get_last_deleted_in_channel};

pub async fn handle_snipe(ctx: &Context, msg: &Message, _args: &[&str]) {
    let bot_id = ctx.cache.current_user().id;

    let pool = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    };

    let Some(pool) = pool else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Base de données indisponible.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let result = get_last_deleted_in_channel(&pool, bot_id, msg.channel_id).await;

    let Ok(sniped) = result else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Impossible de lire le snipe depuis la base de données.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let Some(sniped) = sniped else {
        let embed = CreateEmbed::new()
            .title("Snipe")
            .description("Aucun message supprimé enregistré dans ce salon.")
            .color(0x5865F2);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let author = sniped
        .author_id
        .map(|id| format!("<@{}>", id))
        .unwrap_or_else(|| "Inconnu".to_string());

    let deleted_at = discord_ts(
        Timestamp::from_unix_timestamp(sniped.deleted_at.timestamp())
            .unwrap_or_else(|_| Timestamp::now()),
        "F",
    );

    let embed = CreateEmbed::new()
        .title("Dernier message supprimé")
        .color(0xFEE75C)
        .field("Auteur", author, true)
        .field("Supprimé", deleted_at, true)
        .field("Contenu", truncate_text(&sniped.content, 1024), false);

    send_embed(ctx, msg, embed).await;
}

pub struct SnipeCommand;
pub static COMMAND_DESCRIPTOR: SnipeCommand = SnipeCommand;

impl crate::commands::command_contract::CommandSpec for SnipeCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "snipe",
            category: "general",
            params: "[index]",
            summary: "Recupere un message supprime",
            description: "Affiche le dernier message supprime dans le salon ou un index de messages supprimes.",
            examples: &["+snipe", "+se", "+help snipe"],
            default_aliases: &["snp"],
            default_permission: 0,
        }
    }
}
