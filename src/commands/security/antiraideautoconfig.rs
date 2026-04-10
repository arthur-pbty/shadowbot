use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};
use crate::db;

const DEFAULT_STRIKE_RULES: &[(&str, &str, i32)] = &[
    ("spam", "new", 2),
    ("spam", "old", 1),
    ("link", "new", 2),
    ("link", "old", 1),
    ("massmention", "new", 3),
    ("massmention", "old", 2),
    ("badword", "new", 2),
    ("badword", "old", 1),
];

pub async fn handle_antiraideautoconfig(ctx: &Context, msg: &Message, _args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let Some(pool) = ({
        let data = ctx.data.read().await;
        data.get::<db::DbPoolKey>().cloned()
    }) else {
        return;
    };

    let bot_id = ctx.cache.current_user().id.get() as i64;
    let guild_id_raw = guild_id.get() as i64;

    let mut failed = Vec::new();

    if db::set_antispam_settings(&pool, bot_id, guild_id_raw, true, 6, 5)
        .await
        .is_err()
    {
        failed.push("antispam");
    }

    if db::set_antilink_settings(&pool, bot_id, guild_id_raw, true, "invite")
        .await
        .is_err()
    {
        failed.push("antilink");
    }

    if db::set_antimassmention_settings(&pool, bot_id, guild_id_raw, true, 5)
        .await
        .is_err()
    {
        failed.push("antimassmention");
    }

    if db::set_badwords_enabled(&pool, bot_id, guild_id_raw, true)
        .await
        .is_err()
    {
        failed.push("badwords");
    }

    if db::clear_moderation_channel_overrides_by_kind(&pool, bot_id, guild_id_raw, "spam")
        .await
        .is_err()
    {
        failed.push("spam overrides");
    }

    if db::clear_moderation_channel_overrides_by_kind(&pool, bot_id, guild_id_raw, "link")
        .await
        .is_err()
    {
        failed.push("link overrides");
    }

    for (trigger, profile, strike_count) in DEFAULT_STRIKE_RULES {
        if db::upsert_strike_rule(&pool, bot_id, guild_id_raw, trigger, profile, *strike_count)
            .await
            .is_err()
        {
            failed.push("strikes");
            break;
        }
    }

    if db::setup_default_punish_rules(&pool, bot_id, guild_id_raw)
        .await
        .is_err()
    {
        failed.push("punish");
    }

    let mut description = String::from(
        "Configuration anti raid appliquee.\n\n- Antispam: ON (6/5s)\n- AntiLink: ON (invite)\n- AntiMassMention: ON (5)\n- BadWords: ON\n- Strikes: profils par defaut\n- Punish: regles par defaut",
    );

    if !failed.is_empty() {
        description.push_str("\n\nErreurs detectees: ");
        description.push_str(&failed.join(", "));
    }

    let color = if failed.is_empty() {
        theme_color(ctx).await
    } else {
        0xFEE75C
    };

    send_embed(
        ctx,
        msg,
        CreateEmbed::new()
            .title("AntiRaid AutoConfig")
            .description(description)
            .color(color),
    )
    .await;
}

pub struct AntiraideautoconfigCommand;
pub static COMMAND_DESCRIPTOR: AntiraideautoconfigCommand = AntiraideautoconfigCommand;

impl crate::commands::command_contract::CommandSpec for AntiraideautoconfigCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "antiraideautoconfig",
            category: "security",
            params: "aucun",
            description: "Configure automatiquement les protections anti raid du serveur.",
            examples: &["+antiraideautoconfig", "+help antiraideautoconfig"],
            default_aliases: &["arcfg"],
            allow_in_dm: false,
            default_permission: 7,
        }
    }
}
