use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::automod_service::{parse_profile, parse_trigger, pool};
use crate::commands::common::send_embed;
use crate::db;

pub async fn handle_strikes(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id.get() as i64;
    let guild_id_raw = guild_id.get() as i64;

    if args.is_empty() {
        let rules = db::list_strike_rules(&pool, bot_id, guild_id_raw)
            .await
            .unwrap_or_default();

        let mut lines = Vec::new();
        for trigger in ["spam", "link", "massmention", "badword"] {
            let new_count = rules
                .iter()
                .find(|r| r.trigger == trigger && r.profile == "new")
                .map(|r| r.strike_count)
                .unwrap_or(0);
            let old_count = rules
                .iter()
                .find(|r| r.trigger == trigger && r.profile == "old")
                .map(|r| r.strike_count)
                .unwrap_or(0);

            lines.push(format!(
                "`{}` -> nouveau: `{}` | ancien: `{}`",
                trigger, new_count, old_count
            ));
        }

        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Strikes")
                .description(lines.join("\n"))
                .color(0x5865F2),
        )
        .await;
        return;
    }

    if args.len() < 2 {
        return;
    }

    let Some(trigger) = parse_trigger(args[0]) else {
        return;
    };
    let Ok(count) = args[1].parse::<i32>() else {
        return;
    };
    let count = count.clamp(0, 20);

    if let Some(profile) = parse_profile(args.get(2).copied()) {
        let _ = db::upsert_strike_rule(&pool, bot_id, guild_id_raw, trigger, profile, count).await;
    } else {
        let _ = db::upsert_strike_rule(&pool, bot_id, guild_id_raw, trigger, "new", count).await;
        let _ = db::upsert_strike_rule(&pool, bot_id, guild_id_raw, trigger, "old", count).await;
    }

    let rules = db::list_strike_rules(&pool, bot_id, guild_id_raw)
        .await
        .unwrap_or_default();
    let new_count = rules
        .iter()
        .find(|r| r.trigger == trigger && r.profile == "new")
        .map(|r| r.strike_count)
        .unwrap_or(0);
    let old_count = rules
        .iter()
        .find(|r| r.trigger == trigger && r.profile == "old")
        .map(|r| r.strike_count)
        .unwrap_or(0);

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Strikes")
            .description(format!(
                "Regle mise a jour pour `{}`\nNouveau: `{}`\nAncien: `{}`",
                trigger, new_count, old_count
            ))
            .color(0x57F287),
    )
    .await;
}

pub struct StrikesCommand;
pub static COMMAND_DESCRIPTOR: StrikesCommand = StrikesCommand;

impl crate::commands::command_contract::CommandSpec for StrikesCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "strikes",
            category: "security",
            params: "[<trigger> <nombre> [ancien/nouveau]]",
            description: "Affiche ou modifie les strikes attribues pour chaque trigger automod.",
            examples: &["+strikes", "+strikes spam 2", "+strikes link 1 ancien"],
            default_aliases: &["stk"],
            allow_in_dm: false,
            default_permission: 7,
        }
    }
}
