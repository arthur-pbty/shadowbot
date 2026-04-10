use serenity::http::GuildPagination;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub(crate) async fn resolve_guild_target(ctx: &Context, input: &str) -> Option<GuildId> {
    let guilds = guilds_sorted(ctx).await;
    if let Ok(index) = input.parse::<usize>() {
        if index >= 1 && index <= guilds.len() {
            return Some(guilds[index - 1].0);
        }
    }

    input
        .parse::<u64>()
        .ok()
        .map(GuildId::new)
        .filter(|id| guilds.iter().any(|(guild_id, _)| guild_id == id))
}

pub(crate) async fn guilds_sorted(ctx: &Context) -> Vec<(GuildId, String)> {
    let mut all_guilds = Vec::new();
    let mut after: Option<GuildId> = None;

    loop {
        let page = if let Some(after_id) = after {
            ctx.http
                .get_guilds(Some(GuildPagination::After(after_id)), Some(100))
                .await
                .unwrap_or_default()
        } else {
            ctx.http
                .get_guilds(None, Some(100))
                .await
                .unwrap_or_default()
        };

        if page.is_empty() {
            break;
        }

        after = page.last().map(|guild| guild.id);
        all_guilds.extend(page.into_iter().map(|guild| (guild.id, guild.name)));

        if all_guilds.len() % 100 != 0 {
            break;
        }
    }

    all_guilds.sort_by(|a, b| {
        a.1.to_lowercase()
            .cmp(&b.1.to_lowercase())
            .then_with(|| a.0.get().cmp(&b.0.get()))
    });
    all_guilds
}
