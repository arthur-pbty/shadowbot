use crate::commands::command_contract::{CommandMetadata, CommandSpec};

#[path = "admin/addrole.rs"]
pub mod addrole;
pub mod admin_common;
pub mod admin_service;
pub mod advanced_tools;
#[path = "permissions/alias.rs"]
pub mod alias;
#[path = "general/alladmins.rs"]
pub mod alladmins;
#[path = "general/allbots.rs"]
pub mod allbots;
#[path = "permissions/allperms.rs"]
pub mod allperms;
#[path = "admin/autobackup.rs"]
pub mod autobackup;
#[path = "admin/autoconfiglog.rs"]
pub mod autoconfiglog;
#[path = "admin/autopublish.rs"]
pub mod autopublish;
#[path = "admin/autoreact.rs"]
pub mod autoreact;
#[path = "admin/backup.rs"]
pub mod backup;
#[path = "admin/ban.rs"]
pub mod ban;
#[path = "admin/banlist.rs"]
pub mod banlist;
#[path = "general/banner.rs"]
pub mod banner;
#[path = "admin/bl.rs"]
pub mod bl;
#[path = "admin/blinfo.rs"]
pub mod blinfo;
#[path = "admin/boostembed.rs"]
pub mod boostembed;
#[path = "general/boosters.rs"]
pub mod boosters;
#[path = "admin/boostlog.rs"]
pub mod boostlog;
#[path = "general/botadmins.rs"]
pub mod botadmins;
pub mod botconfig_common;
pub mod botconfig_service;
#[path = "admin/bringall.rs"]
pub mod bringall;
#[path = "admin/button.rs"]
pub mod button;
#[path = "general/calc.rs"]
pub mod calc;
#[path = "permissions/change.rs"]
pub mod change;
#[path = "permissions/changeall.rs"]
pub mod changeall;
#[path = "general/channel.rs"]
pub mod channel;
#[path = "general/choose.rs"]
pub mod choose;
#[path = "admin/claim.rs"]
pub mod claim;
#[path = "admin/cleanup.rs"]
pub mod cleanup;
#[path = "admin/clear_all_sanctions.rs"]
pub mod clear_all_sanctions;
#[path = "admin/clear_bl.rs"]
pub mod clear_bl;
#[path = "admin/clear_messages.rs"]
pub mod clear_messages;
#[path = "admin/clear_owners.rs"]
pub mod clear_owners;
#[path = "permissions/clear_perms.rs"]
pub mod clear_perms;
#[path = "admin/clear_sanctions.rs"]
pub mod clear_sanctions;
#[path = "admin/close.rs"]
pub mod close;
#[path = "admin/cmute.rs"]
pub mod cmute;
pub mod command_contract;
pub mod common;
#[path = "profile/compet.rs"]
pub mod compet;
#[path = "admin/create.rs"]
pub mod create;
#[path = "permissions/del.rs"]
pub mod del;
#[path = "admin/del_sanction.rs"]
pub mod del_sanction;
#[path = "admin/delrole.rs"]
pub mod delrole;
#[path = "admin/derank.rs"]
pub mod derank;
#[path = "profile/discussion.rs"]
pub mod discussion;
#[path = "profile/dnd.rs"]
pub mod dnd;
#[path = "admin/embed.rs"]
pub mod embed;
#[path = "general/emoji.rs"]
pub mod emoji;
#[path = "admin/end.rs"]
pub mod end;
#[path = "admin/giveaway.rs"]
pub mod giveaway;
#[path = "general/help.rs"]
pub mod help;
#[path = "permissions/helpsetting.rs"]
pub mod helpsetting;
#[path = "admin/hide.rs"]
pub mod hide;
#[path = "admin/hideall.rs"]
pub mod hideall;
#[path = "profile/idle.rs"]
pub mod idle;
#[path = "profile/invisible.rs"]
pub mod invisible;
#[path = "admin/invite.rs"]
pub mod invite;
#[path = "admin/join.rs"]
pub mod join;
#[path = "admin/kick.rs"]
pub mod kick;
#[path = "admin/leave.rs"]
pub mod leave;
#[path = "admin/leave_settings.rs"]
pub mod leave_settings;
#[path = "profile/listen.rs"]
pub mod listen;
#[path = "general/loading.rs"]
pub mod loading;
#[path = "admin/lock.rs"]
pub mod lock;
#[path = "admin/lockall.rs"]
pub mod lockall;
pub mod logs_command_helpers;
pub mod logs_service;
#[path = "permissions/mainprefix.rs"]
pub mod mainprefix;
#[path = "admin/massiverole.rs"]
pub mod massiverole;
#[path = "general/member.rs"]
pub mod member;
#[path = "admin/messagelog.rs"]
pub mod messagelog;
pub mod moderation_channel_helpers;
pub mod moderation_sanction_helpers;
pub mod moderation_tools;
#[path = "admin/modlog.rs"]
pub mod modlog;
#[path = "profile/mp.rs"]
pub mod mp;
#[path = "admin/mute.rs"]
pub mod mute;
#[path = "admin/mutelist.rs"]
pub mod mutelist;
#[path = "admin/newsticker.rs"]
pub mod newsticker;
#[path = "admin/nolog.rs"]
pub mod nolog;
#[path = "profile/online.rs"]
pub mod online;
#[path = "admin/owner.rs"]
pub mod owner;
#[path = "permissions/perms.rs"]
pub mod perms;
pub mod perms_helpers;
pub mod perms_service;
#[path = "general/pic.rs"]
pub mod pic;
#[path = "general/ping.rs"]
pub mod ping;
#[path = "profile/playto.rs"]
pub mod playto;
#[path = "permissions/prefix.rs"]
pub mod prefix;
#[path = "admin/raidlog.rs"]
pub mod raidlog;
#[path = "profile/remove_activity.rs"]
pub mod remove_activity;
#[path = "admin/rename.rs"]
pub mod rename;
#[path = "admin/renew.rs"]
pub mod renew;
#[path = "admin/reroll.rs"]
pub mod reroll;
#[path = "general/role.rs"]
pub mod role;
#[path = "admin/rolelog.rs"]
pub mod rolelog;
#[path = "general/rolemembers.rs"]
pub mod rolemembers;
#[path = "admin/sanctions.rs"]
pub mod sanctions;
#[path = "admin/say.rs"]
pub mod say;
#[path = "general/server.rs"]
pub mod server;
#[path = "general/serverinfo.rs"]
pub mod serverinfo;
#[path = "profile/set.rs"]
pub mod set;
#[path = "admin/set_boostembed.rs"]
pub mod set_boostembed;
#[path = "admin/set_modlogs.rs"]
pub mod set_modlogs;
#[path = "general/shadowbot.rs"]
pub mod shadowbot;
#[path = "general/showpics.rs"]
pub mod showpics;
#[path = "general/snipe.rs"]
pub mod snipe;
#[path = "profile/stream.rs"]
pub mod stream;
#[path = "admin/suggestion.rs"]
pub mod suggestion;
#[path = "admin/sync.rs"]
pub mod sync;
#[path = "admin/tempban.rs"]
pub mod tempban;
#[path = "admin/tempcmute.rs"]
pub mod tempcmute;
#[path = "admin/tempmute.rs"]
pub mod tempmute;
#[path = "admin/temprole.rs"]
pub mod temprole;
#[path = "admin/tempvoc.rs"]
pub mod tempvoc;
#[path = "admin/tempvoc_cmd.rs"]
pub mod tempvoc_cmd;
#[path = "profile/theme.rs"]
pub mod theme;
#[path = "admin/ticket.rs"]
pub mod ticket;
#[path = "admin/ticket_member.rs"]
pub mod ticket_member;
#[path = "admin/tickets.rs"]
pub mod tickets;
#[path = "admin/unban.rs"]
pub mod unban;
#[path = "admin/unbanall.rs"]
pub mod unbanall;
#[path = "admin/unbl.rs"]
pub mod unbl;
#[path = "admin/uncmute.rs"]
pub mod uncmute;
#[path = "admin/unhide.rs"]
pub mod unhide;
#[path = "admin/unhideall.rs"]
pub mod unhideall;
#[path = "admin/unlock.rs"]
pub mod unlock;
#[path = "admin/unlockall.rs"]
pub mod unlockall;
#[path = "admin/unmassiverole.rs"]
pub mod unmassiverole;
#[path = "admin/unmute.rs"]
pub mod unmute;
#[path = "admin/unmuteall.rs"]
pub mod unmuteall;
#[path = "admin/unowner.rs"]
pub mod unowner;
#[path = "admin/untemprole.rs"]
pub mod untemprole;
#[path = "general/user.rs"]
pub mod user;
#[path = "logs/viewlogs.rs"]
pub mod viewlogs;
#[path = "general/vocinfo.rs"]
pub mod vocinfo;
#[path = "admin/voicekick.rs"]
pub mod voicekick;
#[path = "admin/voicelog.rs"]
pub mod voicelog;
#[path = "admin/voicemove.rs"]
pub mod voicemove;
#[path = "admin/warn.rs"]
pub mod warn;
#[path = "profile/watch.rs"]
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
        ticket::COMMAND_DESCRIPTOR.metadata(),
        claim::COMMAND_DESCRIPTOR.metadata(),
        rename::COMMAND_DESCRIPTOR.metadata(),
        ticket_member::COMMAND_DESCRIPTOR.metadata(),
        close::COMMAND_DESCRIPTOR.metadata(),
        tickets::COMMAND_DESCRIPTOR.metadata(),
        showpics::COMMAND_DESCRIPTOR.metadata(),
        suggestion::COMMAND_DESCRIPTOR.metadata(),
        autopublish::COMMAND_DESCRIPTOR.metadata(),
        tempvoc::COMMAND_DESCRIPTOR.metadata(),
        tempvoc_cmd::COMMAND_DESCRIPTOR.metadata(),
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
        .find(|meta| meta.name == key)
}

pub fn resolve_default_alias(alias: &str) -> Option<&'static str> {
    let normalized = alias.trim().trim_start_matches('+').to_lowercase();
    all_command_metadata().into_iter().find_map(|meta| {
        if meta
            .default_aliases
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(&normalized))
        {
            Some(meta.name)
        } else {
            None
        }
    })
}
