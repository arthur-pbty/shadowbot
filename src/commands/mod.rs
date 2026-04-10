use crate::commands::command_contract::{CommandMetadata, CommandSpec};

pub mod addrole;
pub mod admin_common;
pub mod admin_service;
pub mod advanced_tools;
pub mod alias;
pub mod alladmins;
pub mod allbots;
pub mod allperms;
pub mod autobackup;
pub mod autoconfiglog;
pub mod autoreact;
pub mod backup;
pub mod ban;
pub mod banlist;
pub mod banner;
pub mod bl;
pub mod blinfo;
pub mod boostembed;
pub mod boosters;
pub mod boostlog;
pub mod botadmins;
pub mod botconfig_common;
pub mod botconfig_service;
pub mod bringall;
pub mod button;
pub mod calc;
pub mod change;
pub mod changeall;
pub mod channel;
pub mod choose;
pub mod cleanup;
pub mod clear_all_sanctions;
pub mod clear_bl;
pub mod clear_messages;
pub mod clear_owners;
pub mod clear_perms;
pub mod clear_sanctions;
pub mod cmute;
pub mod command_contract;
pub mod common;
pub mod compet;
pub mod create;
pub mod del;
pub mod del_sanction;
pub mod delrole;
pub mod derank;
pub mod discussion;
pub mod dnd;
pub mod embed;
pub mod emoji;
pub mod end;
pub mod giveaway;
pub mod help;
pub mod helpsetting;
pub mod hide;
pub mod hideall;
pub mod idle;
pub mod invisible;
pub mod invite;
pub mod join;
pub mod kick;
pub mod leave;
pub mod leave_settings;
pub mod listen;
pub mod loading;
pub mod lock;
pub mod lockall;
pub mod logs_service;
pub mod mainprefix;
pub mod massiverole;
pub mod member;
pub mod messagelog;
pub mod moderation_tools;
pub mod modlog;
pub mod mp;
pub mod mute;
pub mod mutelist;
pub mod newsticker;
pub mod nolog;
pub mod online;
pub mod owner;
pub mod perms;
pub mod perms_service;
pub mod pic;
pub mod ping;
pub mod playto;
pub mod prefix;
pub mod raidlog;
pub mod remove_activity;
pub mod renew;
pub mod reroll;
pub mod role;
pub mod rolelog;
pub mod rolemembers;
pub mod sanctions;
pub mod say;
pub mod server;
pub mod serverinfo;
pub mod set;
pub mod set_boostembed;
pub mod set_modlogs;
pub mod shadowbot;
pub mod snipe;
pub mod stream;
pub mod sync;
pub mod tempban;
pub mod tempcmute;
pub mod tempmute;
pub mod temprole;
pub mod theme;
pub mod unban;
pub mod unbanall;
pub mod unbl;
pub mod uncmute;
pub mod unhide;
pub mod unhideall;
pub mod unlock;
pub mod unlockall;
pub mod unmassiverole;
pub mod unmute;
pub mod unmuteall;
pub mod unowner;
pub mod untemprole;
pub mod user;
pub mod viewlogs;
pub mod vocinfo;
pub mod voicekick;
pub mod ticket;
pub mod tickets;
pub mod showpics;
pub mod suggestion;
pub mod autopublish;
pub mod tempvoc;
pub mod tempvoc_cmd;
pub mod voicelog;
pub mod voicemove;
pub mod warn;
pub mod claim;
pub mod close;
pub mod rename;
pub mod ticket_member;
pub mod watch;

pub fn all_command_metadata() -> Vec<CommandMetadata> {
    vec![
        ping::COMMAND_DESCRIPTOR.metadata(),
        allbots::COMMAND_DESCRIPTOR.metadata(),
        alladmins::COMMAND_DESCRIPTOR.metadata(),
        botadmins::COMMAND_DESCRIPTOR.metadata(),
        boosters::COMMAND_DESCRIPTOR.metadata(),
        rolemembers::COMMAND_DESCRIPTOR.metadata(),
        serverinfo::COMMAND_DESCRIPTOR.metadata(),
        vocinfo::COMMAND_DESCRIPTOR.metadata(),
        role::COMMAND_DESCRIPTOR.metadata(),
        join::COMMAND_DESCRIPTOR.metadata(),
        channel::COMMAND_DESCRIPTOR.metadata(),
        user::COMMAND_DESCRIPTOR.metadata(),
        member::COMMAND_DESCRIPTOR.metadata(),
        pic::COMMAND_DESCRIPTOR.metadata(),
        banner::COMMAND_DESCRIPTOR.metadata(),
        server::COMMAND_DESCRIPTOR.metadata(),
        snipe::COMMAND_DESCRIPTOR.metadata(),
        emoji::COMMAND_DESCRIPTOR.metadata(),
        giveaway::COMMAND_DESCRIPTOR.metadata(),
        modlog::COMMAND_DESCRIPTOR.metadata(),
        messagelog::COMMAND_DESCRIPTOR.metadata(),
        voicelog::COMMAND_DESCRIPTOR.metadata(),
        boostlog::COMMAND_DESCRIPTOR.metadata(),
        rolelog::COMMAND_DESCRIPTOR.metadata(),
        raidlog::COMMAND_DESCRIPTOR.metadata(),
        autoconfiglog::COMMAND_DESCRIPTOR.metadata(),
        boostembed::COMMAND_DESCRIPTOR.metadata(),
        nolog::COMMAND_DESCRIPTOR.metadata(),
        set_modlogs::COMMAND_DESCRIPTOR.metadata(),
        set_boostembed::COMMAND_DESCRIPTOR.metadata(),
        sanctions::COMMAND_DESCRIPTOR.metadata(),
        end::COMMAND_DESCRIPTOR.metadata(),
        reroll::COMMAND_DESCRIPTOR.metadata(),
        choose::COMMAND_DESCRIPTOR.metadata(),
        embed::COMMAND_DESCRIPTOR.metadata(),
        clear_messages::COMMAND_DESCRIPTOR.metadata(),
        backup::COMMAND_DESCRIPTOR.metadata(),
        autobackup::COMMAND_DESCRIPTOR.metadata(),
        loading::COMMAND_DESCRIPTOR.metadata(),
        create::COMMAND_DESCRIPTOR.metadata(),
        newsticker::COMMAND_DESCRIPTOR.metadata(),
        massiverole::COMMAND_DESCRIPTOR.metadata(),
        unmassiverole::COMMAND_DESCRIPTOR.metadata(),
        voicemove::COMMAND_DESCRIPTOR.metadata(),
        voicekick::COMMAND_DESCRIPTOR.metadata(),
        cleanup::COMMAND_DESCRIPTOR.metadata(),
        bringall::COMMAND_DESCRIPTOR.metadata(),
        renew::COMMAND_DESCRIPTOR.metadata(),
        unbanall::COMMAND_DESCRIPTOR.metadata(),
        warn::COMMAND_DESCRIPTOR.metadata(),
        mute::COMMAND_DESCRIPTOR.metadata(),
        tempmute::COMMAND_DESCRIPTOR.metadata(),
        unmute::COMMAND_DESCRIPTOR.metadata(),
        cmute::COMMAND_DESCRIPTOR.metadata(),
        tempcmute::COMMAND_DESCRIPTOR.metadata(),
        uncmute::COMMAND_DESCRIPTOR.metadata(),
        mutelist::COMMAND_DESCRIPTOR.metadata(),
        unmuteall::COMMAND_DESCRIPTOR.metadata(),
        kick::COMMAND_DESCRIPTOR.metadata(),
        ban::COMMAND_DESCRIPTOR.metadata(),
        tempban::COMMAND_DESCRIPTOR.metadata(),
        unban::COMMAND_DESCRIPTOR.metadata(),
        banlist::COMMAND_DESCRIPTOR.metadata(),
        lock::COMMAND_DESCRIPTOR.metadata(),
        unlock::COMMAND_DESCRIPTOR.metadata(),
        lockall::COMMAND_DESCRIPTOR.metadata(),
        unlockall::COMMAND_DESCRIPTOR.metadata(),
        hide::COMMAND_DESCRIPTOR.metadata(),
        unhide::COMMAND_DESCRIPTOR.metadata(),
        hideall::COMMAND_DESCRIPTOR.metadata(),
        unhideall::COMMAND_DESCRIPTOR.metadata(),
        addrole::COMMAND_DESCRIPTOR.metadata(),
        delrole::COMMAND_DESCRIPTOR.metadata(),
        derank::COMMAND_DESCRIPTOR.metadata(),
        del_sanction::COMMAND_DESCRIPTOR.metadata(),
        clear_sanctions::COMMAND_DESCRIPTOR.metadata(),
        clear_all_sanctions::COMMAND_DESCRIPTOR.metadata(),
        temprole::COMMAND_DESCRIPTOR.metadata(),
        untemprole::COMMAND_DESCRIPTOR.metadata(),
        sync::COMMAND_DESCRIPTOR.metadata(),
        button::COMMAND_DESCRIPTOR.metadata(),
        autoreact::COMMAND_DESCRIPTOR.metadata(),
        calc::COMMAND_DESCRIPTOR.metadata(),
        shadowbot::COMMAND_DESCRIPTOR.metadata(),
        set::COMMAND_DESCRIPTOR.metadata(),
        theme::COMMAND_DESCRIPTOR.metadata(),
        playto::COMMAND_DESCRIPTOR.metadata(),
        listen::COMMAND_DESCRIPTOR.metadata(),
        watch::COMMAND_DESCRIPTOR.metadata(),
        compet::COMMAND_DESCRIPTOR.metadata(),
        stream::COMMAND_DESCRIPTOR.metadata(),
        remove_activity::COMMAND_DESCRIPTOR.metadata(),
        online::COMMAND_DESCRIPTOR.metadata(),
        idle::COMMAND_DESCRIPTOR.metadata(),
        dnd::COMMAND_DESCRIPTOR.metadata(),
        invisible::COMMAND_DESCRIPTOR.metadata(),
        owner::COMMAND_DESCRIPTOR.metadata(),
        unowner::COMMAND_DESCRIPTOR.metadata(),
        clear_owners::COMMAND_DESCRIPTOR.metadata(),
        bl::COMMAND_DESCRIPTOR.metadata(),
        unbl::COMMAND_DESCRIPTOR.metadata(),
        blinfo::COMMAND_DESCRIPTOR.metadata(),
        clear_bl::COMMAND_DESCRIPTOR.metadata(),
        say::COMMAND_DESCRIPTOR.metadata(),
        change::COMMAND_DESCRIPTOR.metadata(),
        changeall::COMMAND_DESCRIPTOR.metadata(),
        mainprefix::COMMAND_DESCRIPTOR.metadata(),
        prefix::COMMAND_DESCRIPTOR.metadata(),
        perms::COMMAND_DESCRIPTOR.metadata(),
        del::COMMAND_DESCRIPTOR.metadata(),
        clear_perms::COMMAND_DESCRIPTOR.metadata(),
        allperms::COMMAND_DESCRIPTOR.metadata(),
        help::COMMAND_DESCRIPTOR.metadata(),
        helpsetting::COMMAND_DESCRIPTOR.metadata(),
        alias::COMMAND_DESCRIPTOR.metadata(),
        mp::COMMAND_DESCRIPTOR.metadata(),
        invite::COMMAND_DESCRIPTOR.metadata(),
        leave::COMMAND_DESCRIPTOR.metadata(),
        leave_settings::COMMAND_DESCRIPTOR.metadata(),
        viewlogs::COMMAND_DESCRIPTOR.metadata(),
        discussion::COMMAND_DESCRIPTOR.metadata(),
    ]
}

pub fn command_metadata_by_key(key: &str) -> Option<CommandMetadata> {
    all_command_metadata()
        .into_iter()
        .find(|meta| meta.key == key)
}

pub fn resolve_default_alias(alias: &str) -> Option<&'static str> {
    let normalized = alias.trim().trim_start_matches('+').to_lowercase();
    all_command_metadata().into_iter().find_map(|meta| {
        if meta
            .default_aliases
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(&normalized))
        {
            Some(meta.key)
        } else {
            None
        }
    })
}
