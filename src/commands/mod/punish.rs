use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::automod_service::{
    format_duration, parse_duration_to_seconds, parse_sanction, pool,
};
use crate::commands::common::send_embed;
use crate::db;

fn describe_rule(index: usize, rule: &db::PunishRule) -> String {
    let sanction = if let Some(duration) = rule.sanction_seconds {
        format!("{} {}", rule.sanction, format_duration(duration))
    } else {
        rule.sanction.clone()
    };

    format!(
        "{}. {} strikes / {} -> {}",
        index,
        rule.threshold,
        format_duration(rule.window_seconds),
        sanction
    )
}

pub async fn handle_punish(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = pool(ctx).await else {
        return;
    };
    let bot_id = ctx.cache.current_user().id.get() as i64;
    let guild_id_raw = guild_id.get() as i64;

    if args.is_empty() {
        let rules = db::list_punish_rules(&pool, bot_id, guild_id_raw)
            .await
            .unwrap_or_default();

        let description = if rules.is_empty() {
            "Aucune regle.".to_string()
        } else {
            rules
                .iter()
                .enumerate()
                .map(|(idx, rule)| describe_rule(idx + 1, rule))
                .collect::<Vec<_>>()
                .join("\n")
        };

        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Punish")
                .description(description)
                .color(0x5865F2),
        )
        .await;
        return;
    }

    if args[0].eq_ignore_ascii_case("setup") {
        let _ = db::setup_default_punish_rules(&pool, bot_id, guild_id_raw).await;
        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Punish")
                .description("Regles par defaut restaurees.")
                .color(0x57F287),
        )
        .await;
        return;
    }

    if args[0].eq_ignore_ascii_case("add") {
        if args.len() < 4 {
            return;
        }

        let Ok(threshold) = args[1].parse::<i32>() else {
            return;
        };
        let Some(window_seconds) = parse_duration_to_seconds(args[2]) else {
            return;
        };
        let Some(sanction) = parse_sanction(args[3]) else {
            return;
        };
        let sanction_seconds = args.get(4).and_then(|raw| parse_duration_to_seconds(raw));

        let _ = db::upsert_punish_rule(
            &pool,
            bot_id,
            guild_id_raw,
            threshold.clamp(1, 200),
            window_seconds,
            sanction,
            sanction_seconds,
        )
        .await;

        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Punish")
                .description("Regle ajoutee ou mise a jour.")
                .color(0x57F287),
        )
        .await;
        return;
    }

    if args[0].eq_ignore_ascii_case("del") {
        let Some(raw_index) = args.get(1) else {
            return;
        };
        let Ok(index) = raw_index.parse::<usize>() else {
            return;
        };

        let rules = db::list_punish_rules(&pool, bot_id, guild_id_raw)
            .await
            .unwrap_or_default();
        if index == 0 || index > rules.len() {
            return;
        }

        let rule = &rules[index - 1];
        let _ = db::delete_punish_rule_by_id(&pool, bot_id, guild_id_raw, rule.id).await;

        send_embed(
            ctx,
            msg,
            CreateEmbed::new()
                .title("Punish")
                .description(format!("Regle {} supprimee.", index))
                .color(0x57F287),
        )
        .await;
        return;
    }

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("Punish")
            .description("Usage: +punish | +punish add <nombre> <duree> <sanction> [duree] | +punish del <numero> | +punish setup")
            .color(0xED4245),
    )
    .await;
}

pub struct PunishCommand;
pub static COMMAND_DESCRIPTOR: PunishCommand = PunishCommand;

impl crate::commands::command_contract::CommandSpec for PunishCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "punish",
            category: "mod",
            params: "[add <nombre> <duree> <sanction> [duree] | del <numero> | setup]",
            description: "Affiche et gere les sanctions automatiques appliquees selon les strikes.",
            examples: &["+punish", "+punish add 8 1h mute 30m", "+punish setup"],
            default_aliases: &["pn"],
            allow_in_dm: false,
            default_permission: 7,
        }
    }
}
