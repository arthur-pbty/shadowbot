use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::Colour;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::send_embed;

pub async fn handle_show_pics(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Ok(guild) = guild_id.to_partial_guild(&ctx).await else {
        return;
    };

    let members = guild.members(ctx, Some(200), None).await.unwrap_or_default();
    let members: Vec<_> = members.into_iter().filter(|member| !member.user.bot).collect();

    if members.is_empty() {
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Photos de profils")
                .description("Aucun membre disponible.")
                .color(0xED4245),
        )
        .await;
        return;
    }

    let count = args
        .first()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(1)
        .clamp(1, 5);

    for index in 0..count {
        let selected = &members[(msg.id.get() as usize + index) % members.len()];
        let embed = CreateEmbed::new()
            .title(format!("📸 {}", selected.user.name))
            .image(selected.user.face())
            .colour(Colour::from_rgb(100, 150, 255))
            .timestamp(Utc::now());

        send_embed(ctx, msg, embed).await;
    }
}
