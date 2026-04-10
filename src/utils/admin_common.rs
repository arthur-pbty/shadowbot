use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;
use crate::permissions::is_owner_user;

pub fn parse_user_id(input: &str) -> Option<UserId> {
    let cleaned = input
        .trim()
        .trim_start_matches('<')
        .trim_end_matches('>')
        .trim_start_matches('@')
        .trim_start_matches('!');

    cleaned.parse::<u64>().ok().map(UserId::new)
}

pub async fn app_owner_id(ctx: &Context) -> Option<UserId> {
    let info = ctx.http.get_current_application_info().await.ok()?;
    info.owner.map(|u| u.id)
}

pub async fn ensure_owner(ctx: &Context, msg: &Message) -> Result<(), ()> {
    if is_owner_user(ctx, msg.author.id).await {
        Ok(())
    } else {
        let embed = CreateEmbed::new()
            .title("Accès refusé")
            .description("Cette commande est réservée aux owners du bot.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        Err(())
    }
}

pub async fn ban_user_everywhere(ctx: &Context, user_id: UserId, reason: &str) -> (usize, usize) {
    let guilds = ctx.cache.guilds();
    let mut ok = 0usize;
    let mut ko = 0usize;

    for guild_id in guilds {
        match guild_id
            .ban_with_reason(&ctx.http, user_id, 0, reason)
            .await
        {
            Ok(_) => ok += 1,
            Err(_) => ko += 1,
        }
    }

    (ok, ko)
}
