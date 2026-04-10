use crate::commands::command_contract::{CommandMetadata, CommandSpec};

#[path = "roles/addrole.rs"]
pub mod addrole;
pub mod admin_common;
pub mod admin_service;
pub mod advanced_tools;
#[path = "permissions/alias.rs"]
pub mod alias;
#[path = "administration/alladmins.rs"]
pub mod alladmins;
#[path = "administration/allbots.rs"]
pub mod allbots;
#[path = "permissions/allperms.rs"]
pub mod allperms;
#[path = "outils/autobackup.rs"]
pub mod autobackup;
#[path = "logs/autoconfiglog.rs"]
pub mod autoconfiglog;
#[path = "outils/autopublish.rs"]
pub mod autopublish;
#[path = "outils/autoreact.rs"]
pub mod autoreact;
#[path = "outils/backup.rs"]
pub mod backup;
#[path = "moderation/ban.rs"]
pub mod ban;
#[path = "moderation/banlist.rs"]
pub mod banlist;
#[path = "infos/banner.rs"]
pub mod banner;
#[path = "administration/bl.rs"]
pub mod bl;
#[path = "administration/blinfo.rs"]
pub mod blinfo;
#[path = "logs/boostembed.rs"]
pub mod boostembed;
#[path = "infos/boosters.rs"]
pub mod boosters;
#[path = "logs/boostlog.rs"]
pub mod boostlog;
#[path = "administration/botadmins.rs"]
pub mod botadmins;
pub mod botconfig_common;
pub mod botconfig_service;
#[path = "salons_vocal/bringall.rs"]
pub mod bringall;
#[path = "outils/button.rs"]
pub mod button;
#[path = "outils/calc.rs"]
pub mod calc;
#[path = "bot/change.rs"]
pub mod change;
#[path = "bot/changeall.rs"]
pub mod changeall;
#[path = "infos/channel.rs"]
pub mod channel;
#[path = "outils/choose.rs"]
pub mod choose;
#[path = "outils/claim.rs"]
pub mod claim;
#[path = "moderation/cleanup.rs"]
pub mod cleanup;
#[path = "moderation/clear_all_sanctions.rs"]
pub mod clear_all_sanctions;
#[path = "administration/clear_bl.rs"]
pub mod clear_bl;
#[path = "moderation/clear_messages.rs"]
pub mod clear_messages;
#[path = "administration/clear_owners.rs"]
pub mod clear_owners;
#[path = "permissions/clear_perms.rs"]
pub mod clear_perms;
#[path = "moderation/clear_sanctions.rs"]
pub mod clear_sanctions;
#[path = "outils/close.rs"]
pub mod close;
#[path = "moderation/cmute.rs"]
pub mod cmute;
pub mod command_contract;
pub mod common;
#[path = "bot/compet.rs"]
pub mod compet;
#[path = "outils/create.rs"]
pub mod create;
#[path = "permissions/del.rs"]
pub mod del;
#[path = "moderation/del_sanction.rs"]
pub mod del_sanction;
#[path = "roles/delrole.rs"]
pub mod delrole;
#[path = "roles/derank.rs"]
pub mod derank;
#[path = "administration/discussion.rs"]
pub mod discussion;
#[path = "bot/dnd.rs"]
pub mod dnd;
#[path = "outils/embed.rs"]
pub mod embed;
#[path = "outils/emoji.rs"]
pub mod emoji;
#[path = "outils/end.rs"]
pub mod end;
#[path = "outils/giveaway.rs"]
pub mod giveaway;
#[path = "permissions/help.rs"]
pub mod help;
#[path = "permissions/helpsetting.rs"]
pub mod helpsetting;
#[path = "salons_vocal/hide.rs"]
pub mod hide;
#[path = "salons_vocal/hideall.rs"]
pub mod hideall;
#[path = "bot/idle.rs"]
pub mod idle;
#[path = "bot/invisible.rs"]
pub mod invisible;
#[path = "administration/invite.rs"]
pub mod invite;
#[path = "logs/join.rs"]
pub mod join;
#[path = "moderation/kick.rs"]
pub mod kick;
#[path = "administration/leave.rs"]
pub mod leave;
#[path = "logs/leave_settings.rs"]
pub mod leave_settings;
#[path = "bot/listen.rs"]
pub mod listen;
#[path = "outils/loading.rs"]
pub mod loading;
#[path = "salons_vocal/lock.rs"]
pub mod lock;
#[path = "salons_vocal/lockall.rs"]
pub mod lockall;
pub mod logs_command_helpers;
pub mod logs_service;
#[path = "administration/mainprefix.rs"]
pub mod mainprefix;
#[path = "roles/massiverole.rs"]
pub mod massiverole;
#[path = "infos/member.rs"]
pub mod member;
#[path = "logs/messagelog.rs"]
pub mod messagelog;
pub mod moderation_channel_helpers;
pub mod moderation_sanction_helpers;
pub mod moderation_tools;
#[path = "logs/modlog.rs"]
pub mod modlog;
#[path = "administration/mp.rs"]
pub mod mp;
#[path = "moderation/mute.rs"]
pub mod mute;
#[path = "moderation/mutelist.rs"]
pub mod mutelist;
#[path = "outils/newsticker.rs"]
pub mod newsticker;
#[path = "logs/nolog.rs"]
pub mod nolog;
#[path = "bot/online.rs"]
pub mod online;
#[path = "administration/owner.rs"]
pub mod owner;
#[path = "permissions/perms.rs"]
pub mod perms;
pub mod perms_helpers;
pub mod perms_service;
#[path = "infos/pic.rs"]
pub mod pic;
#[path = "infos/ping.rs"]
pub mod ping;
#[path = "bot/playto.rs"]
pub mod playto;
#[path = "administration/prefix.rs"]
pub mod prefix;
#[path = "logs/raidlog.rs"]
pub mod raidlog;
#[path = "bot/remove_activity.rs"]
pub mod remove_activity;
#[path = "outils/rename.rs"]
pub mod rename;
#[path = "moderation/renew.rs"]
pub mod renew;
#[path = "outils/reroll.rs"]
pub mod reroll;
#[path = "infos/role.rs"]
pub mod role;
#[path = "logs/rolelog.rs"]
pub mod rolelog;
#[path = "infos/rolemembers.rs"]
pub mod rolemembers;
#[path = "roles/rolemenu.rs"]
pub mod rolemenu;
#[path = "moderation/sanctions.rs"]
pub mod sanctions;
#[path = "outils/say.rs"]
pub mod say;
#[path = "infos/server.rs"]
pub mod server;
#[path = "infos/serverinfo.rs"]
pub mod serverinfo;
#[path = "bot/set.rs"]
pub mod set;
#[path = "logs/set_boostembed.rs"]
pub mod set_boostembed;
#[path = "logs/set_modlogs.rs"]
pub mod set_modlogs;
#[path = "bot/shadowbot.rs"]
pub mod shadowbot;
#[path = "infos/showpics.rs"]
pub mod showpics;
#[path = "salons_vocal/slowmode.rs"]
pub mod slowmode;
#[path = "outils/snipe.rs"]
pub mod snipe;
#[path = "bot/stream.rs"]
pub mod stream;
#[path = "outils/suggestion.rs"]
pub mod suggestion;
#[path = "roles/sync.rs"]
pub mod sync;
#[path = "moderation/tempban.rs"]
pub mod tempban;
#[path = "moderation/tempcmute.rs"]
pub mod tempcmute;
#[path = "moderation/tempmute.rs"]
pub mod tempmute;
#[path = "roles/temprole.rs"]
pub mod temprole;
#[path = "salons_vocal/tempvoc.rs"]
pub mod tempvoc;
#[path = "salons_vocal/tempvoc_cmd.rs"]
pub mod tempvoc_cmd;
#[path = "bot/theme.rs"]
pub mod theme;
#[path = "outils/ticket.rs"]
pub mod ticket;
#[path = "outils/ticket_member.rs"]
pub mod ticket_member;
#[path = "outils/tickets.rs"]
pub mod tickets;
#[path = "moderation/unban.rs"]
pub mod unban;
#[path = "moderation/unbanall.rs"]
pub mod unbanall;
#[path = "administration/unbl.rs"]
pub mod unbl;
#[path = "moderation/uncmute.rs"]
pub mod uncmute;
#[path = "salons_vocal/unhide.rs"]
pub mod unhide;
#[path = "salons_vocal/unhideall.rs"]
pub mod unhideall;
#[path = "salons_vocal/unlock.rs"]
pub mod unlock;
#[path = "salons_vocal/unlockall.rs"]
pub mod unlockall;
#[path = "roles/unmassiverole.rs"]
pub mod unmassiverole;
#[path = "moderation/unmute.rs"]
pub mod unmute;
#[path = "moderation/unmuteall.rs"]
pub mod unmuteall;
#[path = "administration/unowner.rs"]
pub mod unowner;
#[path = "roles/untemprole.rs"]
pub mod untemprole;
#[path = "infos/user.rs"]
pub mod user;
#[path = "logs/viewlogs.rs"]
pub mod viewlogs;
#[path = "infos/vocinfo.rs"]
pub mod vocinfo;
#[path = "salons_vocal/voicekick.rs"]
pub mod voicekick;
#[path = "logs/voicelog.rs"]
pub mod voicelog;
#[path = "salons_vocal/voicemove.rs"]
pub mod voicemove;
#[path = "moderation/warn.rs"]
pub mod warn;
#[path = "bot/watch.rs"]
pub mod watch;

pub fn all_command_metadata() -> Vec<CommandMetadata> {
    vec![
        ping::COMMAND_DESCRIPTOR.metadata(),
        allbots::COMMAND_DESCRIPTOR.metadata(),
        alladmins::COMMAND_DESCRIPTOR.metadata(),
        botadmins::COMMAND_DESCRIPTOR.metadata(),
        boosters::COMMAND_DESCRIPTOR.metadata(),
        rolemembers::COMMAND_DESCRIPTOR.metadata(),
        rolemenu::COMMAND_DESCRIPTOR.metadata(),
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
        slowmode::COMMAND_DESCRIPTOR.metadata(),
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
