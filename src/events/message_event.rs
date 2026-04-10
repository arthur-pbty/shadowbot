use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::sync::{Mutex, OnceLock};

use crate::commands::moderation_tools;
use crate::commands::remove_activity;
use crate::commands::{
    addrole, alias, autobackup, autoconfiglog, autopublish, autoreact, backup, ban, banlist,
    banner, bl, blinfo, boostembed, boosters, boostlog, bringall, button, calc, change, changeall,
    channel, choose, claim, cleanup, clear_all_sanctions, clear_bl, clear_messages, clear_owners,
    clear_perms, clear_sanctions, close, cmute, compet, create, del, del_sanction, delrole, derank,
    discussion, dnd, embed, emoji, end, giveaway, help, helpsetting, hide, hideall, idle,
    invisible, invite, join, kick, leave, leave_settings, listen, loading, lock, lockall,
    mainprefix, massiverole, member, messagelog, modlog, mp, mute, mutelist, newsticker, nolog,
    online, owner, perms, pic, ping, playto, prefix, raidlog, rename, renew, reroll, role, rolelog,
    rolemembers, sanctions, say, server, serverinfo, set, set_boostembed, set_modlogs, shadowbot,
    showpics, snipe, stream, suggestion, sync, tempban, tempcmute, tempmute, temprole, tempvoc,
    tempvoc_cmd, theme, ticket, ticket_member, tickets, unban, unbanall, unbl, uncmute, unhide,
    unhideall, unlock, unlockall, unmassiverole, unmute, unmuteall, unowner, untemprole, user,
    viewlogs, vocinfo, voicekick, voicelog, voicemove, warn, watch,
};
use crate::commands::{alladmins, allbots, allperms, botadmins};
use crate::db::{DbPoolKey, upsert_message_observed};
use crate::permissions;

const PROCESSED_CACHE_MAX: usize = 8192;
static PROCESSED_MESSAGES: OnceLock<Mutex<ProcessedMessages>> = OnceLock::new();

struct ProcessedMessages {
    order: VecDeque<u64>,
    seen: HashSet<u64>,
}

impl ProcessedMessages {
    fn new() -> Self {
        Self {
            order: VecDeque::with_capacity(PROCESSED_CACHE_MAX),
            seen: HashSet::with_capacity(PROCESSED_CACHE_MAX),
        }
    }
}

fn should_process_message(message_id: MessageId) -> bool {
    let lock = PROCESSED_MESSAGES.get_or_init(|| Mutex::new(ProcessedMessages::new()));
    let mut cache = lock.lock().expect("processed message cache poisoned");
    let id = message_id.get();

    if cache.seen.contains(&id) {
        return false;
    }

    cache.seen.insert(id);
    cache.order.push_back(id);

    while cache.order.len() > PROCESSED_CACHE_MAX {
        if let Some(oldest) = cache.order.pop_front() {
            cache.seen.remove(&oldest);
        }
    }

    true
}

pub async fn handle_message(ctx: &Context, msg: &Message) {
    if !should_process_message(msg.id) {
        return;
    }

    if crate::commands::admin_service::enforce_blacklist_on_message(ctx, msg).await {
        return;
    }

    let bot_id = ctx.cache.current_user().id;
    if let Some(pool) = {
        let data = ctx.data.read().await;
        data.get::<DbPoolKey>().cloned()
    } {
        let _ = upsert_message_observed(&pool, bot_id, msg).await;
    }

    if msg.author.bot {
        return;
    }

    crate::commands::advanced_tools::apply_autoreacts(ctx, msg).await;
    crate::commands::advanced_tools::maybe_run_maintenance(ctx, msg.guild_id).await;
    moderation_tools::maybe_run_maintenance(ctx, msg.guild_id).await;

    let content = msg.content.trim();
    let prefix_value = permissions::resolve_prefix(ctx, msg.guild_id).await;
    if !content.starts_with(&prefix_value) {
        return;
    }

    let without_prefix = content.trim_start_matches(&prefix_value).trim();
    if without_prefix.is_empty() {
        return;
    }

    let mut parts = without_prefix.split_whitespace();
    let mut command = parts.next().unwrap_or("").to_lowercase();
    let args = parts.collect::<Vec<_>>();

    // Ne laisse pas un alias écraser une commande native.
    let native_commands = permissions::all_command_keys();
    if !native_commands.iter().any(|cmd| cmd == &command) {
        if let Some(alias_target) = alias::resolve_command_alias_name(ctx, &command).await {
            command = alias_target;
        } else if let Some(default_target) = crate::commands::resolve_default_alias(&command) {
            command = default_target.to_string();
        }
    }

    let command_key = permissions::command_key(&command, &args);

    let can_use = permissions::can_use_command(ctx, msg, &command_key).await;
    if !can_use {
        let required = permissions::command_required_permission(ctx, &command_key).await;
        permissions::deny_permission(ctx, msg, &command_key, required).await;
        return;
    }

    match command.as_str() {
        "ticket" => ticket::handle_ticket_settings(ctx, msg, &args).await,
        "claim" => claim::handle_claim(ctx, msg, &args).await,
        "rename" => rename::handle_rename(ctx, msg, &args).await,
        "add" => ticket_member::handle_ticket_add(ctx, msg, &args).await,
        "del"
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("perm"))
                .unwrap_or(false) =>
        {
            del::handle_del(ctx, msg, &args).await
        }
        "del"
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("sanction"))
                .unwrap_or(false) =>
        {
            del_sanction::handle_del_sanction(ctx, msg, &args).await
        }
        "del" => ticket_member::handle_ticket_remove(ctx, msg, &args).await,
        "close" => close::handle_close(ctx, msg, &args).await,
        "tickets" => tickets::handle_tickets(ctx, msg, &args).await,
        "show"
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("pics"))
                .unwrap_or(false) =>
        {
            showpics::handle_show_pics(ctx, msg, &args[1..]).await
        }
        "suggestion" => suggestion::handle_suggestion(ctx, msg, &args).await,
        "autopublish" => autopublish::handle_autopublish(ctx, msg, &args).await,
        "tempvoc"
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("cmd"))
                .unwrap_or(false) =>
        {
            tempvoc_cmd::handle_tempvoc_cmd(ctx, msg, &args[1..]).await
        }
        "tempvoc" => tempvoc::handle_tempvoc(ctx, msg, &args).await,
        "ping" => ping::handle_ping(ctx, msg, &args).await,
        "allbots" => allbots::handle_allbots(ctx, msg, &args).await,
        "alladmins" => alladmins::handle_alladmins(ctx, msg, &args).await,
        "botadmins" => botadmins::handle_botadmins(ctx, msg, &args).await,
        "boosters" => boosters::handle_boosters(ctx, msg, &args).await,
        "rolemembers" => rolemembers::handle_rolemembers(ctx, msg, &args).await,
        "serverinfo" => serverinfo::handle_serverinfo(ctx, msg, &args).await,
        "vocinfo" => vocinfo::handle_vocinfo(ctx, msg, &args).await,
        "role" => role::handle_role(ctx, msg, &args).await,
        "channel" => channel::handle_channel(ctx, msg, &args).await,
        "user" => user::handle_user(ctx, msg, &args).await,
        "member" => member::handle_member(ctx, msg, &args).await,
        "pic" => pic::handle_pic(ctx, msg, &args).await,
        "banner" => banner::handle_banner(ctx, msg, &args).await,
        "server" => server::handle_server(ctx, msg, &args).await,
        "snipe" => snipe::handle_snipe(ctx, msg, &args).await,
        "emoji" => emoji::handle_emoji(ctx, msg, &args).await,
        "giveaway" => giveaway::handle_giveaway(ctx, msg, &args).await,
        "modlog" => modlog::handle_modlog(ctx, msg, &args).await,
        "messagelog" => messagelog::handle_messagelog(ctx, msg, &args).await,
        "voicelog" => voicelog::handle_voicelog(ctx, msg, &args).await,
        "boostlog" => boostlog::handle_boostlog(ctx, msg, &args).await,
        "rolelog" => rolelog::handle_rolelog(ctx, msg, &args).await,
        "raidlog" => raidlog::handle_raidlog(ctx, msg, &args).await,
        "autoconfiglog" => autoconfiglog::handle_autoconfiglog(ctx, msg).await,
        "join" => join::handle_join(ctx, msg, &args).await,
        "boostembed" => boostembed::handle_boostembed(ctx, msg, &args).await,
        "nolog" => nolog::handle_nolog(ctx, msg, &args).await,
        "sanctions" => sanctions::handle_sanctions(ctx, msg, &args).await,
        "end" => end::handle_end(ctx, msg, &args).await,
        "reroll" => reroll::handle_reroll(ctx, msg, &args).await,
        "choose" => choose::handle_choose(ctx, msg, &args).await,
        "embed" => embed::handle_embed(ctx, msg, &args).await,
        "backup" => backup::handle_backup(ctx, msg, &args).await,
        "autobackup" => autobackup::handle_autobackup(ctx, msg, &args).await,
        "loading" => loading::handle_loading(ctx, msg, &args).await,
        "create" => create::handle_create(ctx, msg, &args).await,
        "newsticker" => newsticker::handle_newsticker(ctx, msg, &args).await,
        "massiverole" => massiverole::handle_massiverole(ctx, msg, &args).await,
        "unmassiverole" => unmassiverole::handle_unmassiverole(ctx, msg, &args).await,
        "voicemove" => voicemove::handle_voicemove(ctx, msg, &args).await,
        "voicekick" => voicekick::handle_voicekick(ctx, msg, &args).await,
        "cleanup" => cleanup::handle_cleanup(ctx, msg, &args).await,
        "bringall" => bringall::handle_bringall(ctx, msg, &args).await,
        "renew" => renew::handle_renew(ctx, msg, &args).await,
        "unbanall" => unbanall::handle_unbanall(ctx, msg, &args).await,
        "warn" => {
            warn::handle_warn(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "warn", &args).await;
        }
        "mute" => {
            mute::handle_mute(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "mute", &args).await;
        }
        "tempmute" => {
            tempmute::handle_tempmute(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "tempmute", &args)
                .await;
        }
        "unmute" => {
            unmute::handle_unmute(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "unmute", &args).await;
        }
        "cmute" => {
            cmute::handle_cmute(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "cmute", &args).await;
        }
        "tempcmute" => {
            tempcmute::handle_tempcmute(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "tempcmute", &args)
                .await;
        }
        "uncmute" => {
            uncmute::handle_uncmute(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "uncmute", &args).await;
        }
        "mutelist" => {
            mutelist::handle_mutelist(ctx, msg).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "mutelist", &args)
                .await;
        }
        "unmuteall" => {
            unmuteall::handle_unmuteall(ctx, msg).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "unmuteall", &args)
                .await;
        }
        "kick" => {
            kick::handle_kick(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "kick", &args).await;
        }
        "ban" => {
            ban::handle_ban(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "ban", &args).await;
        }
        "tempban" => {
            tempban::handle_tempban(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "tempban", &args).await;
        }
        "unban" => {
            unban::handle_unban(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "unban", &args).await;
        }
        "banlist" => {
            banlist::handle_banlist(ctx, msg).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "banlist", &args).await;
        }
        "lock" => {
            lock::handle_lock(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "lock", &args).await;
        }
        "unlock" => {
            unlock::handle_unlock(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "unlock", &args).await;
        }
        "lockall" => {
            lockall::handle_lockall(ctx, msg).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "lockall", &args).await;
        }
        "unlockall" => {
            unlockall::handle_unlockall(ctx, msg).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "unlockall", &args)
                .await;
        }
        "hide" => {
            hide::handle_hide(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "hide", &args).await;
        }
        "unhide" => {
            unhide::handle_unhide(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "unhide", &args).await;
        }
        "hideall" => {
            hideall::handle_hideall(ctx, msg).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "hideall", &args).await;
        }
        "unhideall" => {
            unhideall::handle_unhideall(ctx, msg).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "unhideall", &args)
                .await;
        }
        "addrole" => {
            addrole::handle_addrole(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "addrole", &args).await;
        }
        "delrole" => {
            delrole::handle_delrole(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "delrole", &args).await;
        }
        "derank" => {
            derank::handle_derank(ctx, msg, &args).await;
            crate::commands::logs_service::log_moderation_command(ctx, msg, "derank", &args).await;
        }
        "temprole" => temprole::handle_temprole(ctx, msg, &args).await,
        "untemprole" => untemprole::handle_untemprole(ctx, msg, &args).await,
        "sync" => sync::handle_sync(ctx, msg, &args).await,
        "button" => button::handle_button(ctx, msg, &args).await,
        "autoreact" => autoreact::handle_autoreact(ctx, msg, &args).await,
        "calc" => calc::handle_calc(ctx, msg, &args).await,
        "shadowbot" => shadowbot::handle_shadowbot(ctx, msg, &args).await,
        "set"
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("modlogs"))
                .unwrap_or(false) =>
        {
            set_modlogs::handle_set_modlogs(ctx, msg, &args[1..]).await
        }
        "set"
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("boostembed"))
                .unwrap_or(false) =>
        {
            set_boostembed::handle_set_boostembed(ctx, msg, &args[1..]).await
        }
        "set" => set::handle_set(ctx, msg, &args).await,
        "theme" => theme::handle_theme(ctx, msg, &args).await,
        "playto" => playto::handle_playto(ctx, msg, &args).await,
        "listen" => listen::handle_listen(ctx, msg, &args).await,
        "watch" => watch::handle_watch(ctx, msg, &args).await,
        "compet" => compet::handle_compet(ctx, msg, &args).await,
        "stream" => stream::handle_stream(ctx, msg, &args).await,
        "help" => help::handle_help(ctx, msg, &args).await,
        "helpsetting" => helpsetting::handle_helpsetting(ctx, msg, &args).await,
        "alias" => alias::handle_alias(ctx, msg, &args).await,
        "mp" => mp::handle_mp(ctx, msg, &args).await,
        "invite" => invite::handle_invite(ctx, msg, &args).await,
        "leave"
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("settings"))
                .unwrap_or(false) =>
        {
            leave_settings::handle_leave_settings(ctx, msg, &args).await
        }
        "leave" => leave::handle_leave(ctx, msg, &args).await,
        "viewlogs" => viewlogs::handle_viewlogs(ctx, msg, &args).await,
        "discussion" => discussion::handle_discussion(ctx, msg, &args).await,
        "remove"
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("activity"))
                .unwrap_or(false) =>
        {
            remove_activity::handle_remove_activity(ctx, msg).await
        }
        "online" => online::handle_online(ctx, msg).await,
        "idle" => idle::handle_idle(ctx, msg).await,
        "dnd" => dnd::handle_dnd(ctx, msg).await,
        "invisible" => invisible::handle_invisible(ctx, msg).await,
        "owner" => owner::handle_owner(ctx, msg, &args).await,
        "unowner" => unowner::handle_unowner(ctx, msg, &args).await,
        "bl" => bl::handle_bl(ctx, msg, &args).await,
        "unbl" => unbl::handle_unbl(ctx, msg, &args).await,
        "blinfo" => blinfo::handle_blinfo(ctx, msg, &args).await,
        "say" => say::handle_say(ctx, msg, &args).await,
        "change" => change::handle_change(ctx, msg, &args).await,
        "changeall" => changeall::handle_changeall(ctx, msg, &args).await,
        "mainprefix" => mainprefix::handle_mainprefix(ctx, msg, &args).await,
        "prefix" => prefix::handle_prefix(ctx, msg, &args).await,
        "perms" => perms::handle_perms(ctx, msg, &args).await,
        "allperms" => allperms::handle_allperms(ctx, msg, &args).await,
        "clear"
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("owners"))
                .unwrap_or(false) =>
        {
            clear_owners::handle_clear_owners(ctx, msg).await
        }
        "clear"
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("bl"))
                .unwrap_or(false) =>
        {
            clear_bl::handle_clear_bl(ctx, msg).await
        }
        "clear"
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("perms"))
                .unwrap_or(false) =>
        {
            clear_perms::handle_clear_perms(ctx, msg).await
        }
        "clear"
            if args
                .first()
                .map(|s| s.eq_ignore_ascii_case("sanctions"))
                .unwrap_or(false) =>
        {
            clear_sanctions::handle_clear_sanctions(ctx, msg, &args).await
        }
        "clear"
            if args.len() >= 2
                && args[0].eq_ignore_ascii_case("all")
                && args[1].eq_ignore_ascii_case("sanctions") =>
        {
            clear_all_sanctions::handle_clear_all_sanctions(ctx, msg).await
        }
        "clear" => clear_messages::handle_clear_messages(ctx, msg, &args).await,
        _ => {}
    }
}
