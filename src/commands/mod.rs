use crate::commands::command_contract::{CommandMetadata, CommandSpec};

#[path = "roles/addrole.rs"]
pub mod addrole;
pub mod admin_common;
pub mod admin_service;
pub mod advanced_tools;
#[path = "perms/alias.rs"]
pub mod alias;
#[path = "info/alladmins.rs"]
pub mod alladmins;
#[path = "info/allbots.rs"]
pub mod allbots;
#[path = "perms/allperms.rs"]
pub mod allperms;
#[path = "roles/ancien.rs"]
pub mod ancien;
#[path = "security/antilink.rs"]
pub mod antilink;
#[path = "security/antimassmention.rs"]
pub mod antimassmention;
#[path = "security/antiraideautoconfig.rs"]
pub mod antiraideautoconfig;
#[path = "security/antispam.rs"]
pub mod antispam;
#[path = "automation/autobackup.rs"]
pub mod autobackup;
#[path = "config/autoconfiglog.rs"]
pub mod autoconfiglog;
pub mod automod_service;
#[path = "automation/autopublish.rs"]
pub mod autopublish;
#[path = "automation/autoreact.rs"]
pub mod autoreact;
#[path = "automation/backup.rs"]
pub mod backup;
#[path = "security/badwords.rs"]
pub mod badwords;
#[path = "mod/ban.rs"]
pub mod ban;
#[path = "mod/banlist.rs"]
pub mod banlist;
#[path = "info/banner.rs"]
pub mod banner;
#[path = "owner/bl.rs"]
pub mod bl;
#[path = "owner/blinfo.rs"]
pub mod blinfo;
#[path = "config/boostembed.rs"]
pub mod boostembed;
#[path = "info/boosters.rs"]
pub mod boosters;
#[path = "config/boostlog.rs"]
pub mod boostlog;
#[path = "owner/botadmins.rs"]
pub mod botadmins;
pub mod botconfig_common;
pub mod botconfig_service;
#[path = "channel/bringall.rs"]
pub mod bringall;
#[path = "automation/button.rs"]
pub mod button;
#[path = "fun/calc.rs"]
pub mod calc;
#[path = "botconfig/change.rs"]
pub mod change;
#[path = "botconfig/changeall.rs"]
pub mod changeall;
#[path = "info/channel.rs"]
pub mod channel;
#[path = "fun/choose.rs"]
pub mod choose;
#[path = "ticket/claim.rs"]
pub mod claim;
#[path = "mod/cleanup.rs"]
pub mod cleanup;
#[path = "mod/clearallsanctions.rs"]
pub mod clear_all_sanctions;
#[path = "mod/clearbadwords.rs"]
pub mod clear_badwords;
#[path = "owner/clearbl.rs"]
pub mod clear_bl;
#[path = "mod/clearlimit.rs"]
pub mod clear_limit;
#[path = "mod/clearmessages.rs"]
pub mod clear_messages;
#[path = "owner/clearowners.rs"]
pub mod clear_owners;
#[path = "perms/clearperms.rs"]
pub mod clear_perms;
#[path = "mod/clearsanctions.rs"]
pub mod clear_sanctions;
#[path = "ticket/close.rs"]
pub mod close;
#[path = "mod/cmute.rs"]
pub mod cmute;
pub mod command_contract;
pub mod common;
#[path = "botconfig/compet.rs"]
pub mod compet;
#[path = "automation/create.rs"]
pub mod create;
#[path = "perms/del.rs"]
pub mod del;
#[path = "mod/delsanction.rs"]
pub mod del_sanction;
#[path = "roles/delrole.rs"]
pub mod delrole;
#[path = "roles/derank.rs"]
pub mod derank;
#[path = "owner/discussion.rs"]
pub mod discussion;
#[path = "botconfig/dnd.rs"]
pub mod dnd;
#[path = "fun/embed.rs"]
pub mod embed;
#[path = "fun/emoji.rs"]
pub mod emoji;
#[path = "event/end.rs"]
pub mod end;
#[path = "event/giveaway.rs"]
pub mod giveaway;
#[path = "perms/help.rs"]
pub mod help;
#[path = "perms/helpsetting.rs"]
pub mod helpsetting;
#[path = "channel/hide.rs"]
pub mod hide;
#[path = "channel/hideall.rs"]
pub mod hideall;
#[path = "botconfig/idle.rs"]
pub mod idle;
#[path = "botconfig/invisible.rs"]
pub mod invisible;
#[path = "owner/invite.rs"]
pub mod invite;
#[path = "config/join.rs"]
pub mod join;
#[path = "mod/kick.rs"]
pub mod kick;
#[path = "owner/leave.rs"]
pub mod leave;
#[path = "config/leavesettings.rs"]
pub mod leave_settings;
#[path = "security/link.rs"]
pub mod link;
#[path = "botconfig/listen.rs"]
pub mod listen;
#[path = "fun/loading.rs"]
pub mod loading;
#[path = "channel/lock.rs"]
pub mod lock;
#[path = "channel/lockall.rs"]
pub mod lockall;
pub mod logs_command_helpers;
pub mod logs_service;
#[path = "botconfig/mainprefix.rs"]
pub mod mainprefix;
#[path = "roles/massiverole.rs"]
pub mod massiverole;
#[path = "info/member.rs"]
pub mod member;
#[path = "config/messagelog.rs"]
pub mod messagelog;
pub mod moderation_channel_helpers;
pub mod moderation_sanction_helpers;
pub mod moderation_tools;
#[path = "config/modlog.rs"]
pub mod modlog;
#[path = "owner/mp.rs"]
pub mod mp;
#[path = "mod/mute.rs"]
pub mod mute;
#[path = "mod/mutelist.rs"]
pub mod mutelist;
#[path = "mod/muterole.rs"]
pub mod muterole;
#[path = "automation/newsticker.rs"]
pub mod newsticker;
#[path = "roles/noderank.rs"]
pub mod noderank;
#[path = "config/nolog.rs"]
pub mod nolog;
#[path = "botconfig/online.rs"]
pub mod online;
#[path = "owner/owner.rs"]
pub mod owner;
#[path = "perms/perms.rs"]
pub mod perms;
pub mod perms_helpers;
pub mod perms_service;
#[path = "info/pic.rs"]
pub mod pic;
#[path = "automation/piconly.rs"]
pub mod piconly;
#[path = "info/ping.rs"]
pub mod ping;
#[path = "botconfig/playto.rs"]
pub mod playto;
#[path = "botconfig/prefix.rs"]
pub mod prefix;
#[path = "channel/public.rs"]
pub mod public;
#[path = "mod/punish.rs"]
pub mod punish;
#[path = "config/raidlog.rs"]
pub mod raidlog;
#[path = "botconfig/removeactivity.rs"]
pub mod remove_activity;
#[path = "ticket/rename.rs"]
pub mod rename;
#[path = "channel/renew.rs"]
pub mod renew;
#[path = "event/reroll.rs"]
pub mod reroll;
#[path = "security/resetantiraide.rs"]
pub mod resetantiraide;
#[path = "info/role.rs"]
pub mod role;
#[path = "config/rolelog.rs"]
pub mod rolelog;
#[path = "info/rolemembers.rs"]
pub mod rolemembers;
#[path = "roles/rolemenu.rs"]
pub mod rolemenu;
#[path = "mod/sanctions.rs"]
pub mod sanctions;
#[path = "fun/say.rs"]
pub mod say;
#[path = "info/server.rs"]
pub mod server;
#[path = "info/serverinfo.rs"]
pub mod serverinfo;
#[path = "botconfig/set.rs"]
pub mod set;
#[path = "config/setboostembed.rs"]
pub mod set_boostembed;
#[path = "config/setmodlogs.rs"]
pub mod set_modlogs;
#[path = "mod/setmuterole.rs"]
pub mod set_muterole;
#[path = "botconfig/shadowbot.rs"]
pub mod shadowbot;
#[path = "info/showpics.rs"]
pub mod showpics;
#[path = "channel/slowmode.rs"]
pub mod slowmode;
#[path = "fun/snipe.rs"]
pub mod snipe;
#[path = "security/spam.rs"]
pub mod spam;
#[path = "botconfig/stream.rs"]
pub mod stream;
#[path = "security/strikes.rs"]
pub mod strikes;
#[path = "fun/suggestion.rs"]
pub mod suggestion;
#[path = "roles/sync.rs"]
pub mod sync;
#[path = "mod/tempban.rs"]
pub mod tempban;
#[path = "mod/tempcmute.rs"]
pub mod tempcmute;
#[path = "mod/tempmute.rs"]
pub mod tempmute;
#[path = "roles/temprole.rs"]
pub mod temprole;
#[path = "channel/tempvoc.rs"]
pub mod tempvoc;
#[path = "channel/tempvoccmd.rs"]
pub mod tempvoc_cmd;
#[path = "botconfig/theme.rs"]
pub mod theme;
#[path = "ticket/ticket.rs"]
pub mod ticket;
#[path = "ticket/ticketmember.rs"]
pub mod ticket_member;
#[path = "ticket/tickets.rs"]
pub mod tickets;
#[path = "mod/timeout.rs"]
pub mod timeout;
#[path = "mod/unban.rs"]
pub mod unban;
#[path = "mod/unbanall.rs"]
pub mod unbanall;
#[path = "owner/unbl.rs"]
pub mod unbl;
#[path = "mod/uncmute.rs"]
pub mod uncmute;
#[path = "channel/unhide.rs"]
pub mod unhide;
#[path = "channel/unhideall.rs"]
pub mod unhideall;
#[path = "channel/unlock.rs"]
pub mod unlock;
#[path = "channel/unlockall.rs"]
pub mod unlockall;
#[path = "roles/unmassiverole.rs"]
pub mod unmassiverole;
#[path = "mod/unmute.rs"]
pub mod unmute;
#[path = "mod/unmuteall.rs"]
pub mod unmuteall;
#[path = "owner/unowner.rs"]
pub mod unowner;
#[path = "roles/untemprole.rs"]
pub mod untemprole;
#[path = "info/user.rs"]
pub mod user;
#[path = "config/viewlogs.rs"]
pub mod viewlogs;
#[path = "info/vocinfo.rs"]
pub mod vocinfo;
#[path = "channel/voicekick.rs"]
pub mod voicekick;
#[path = "config/voicelog.rs"]
pub mod voicelog;
#[path = "channel/voicemove.rs"]
pub mod voicemove;
#[path = "mod/warn.rs"]
pub mod warn;
#[path = "botconfig/watch.rs"]
pub mod watch;

pub fn all_command_metadata() -> Vec<CommandMetadata> {
    vec![
        ping::COMMAND_DESCRIPTOR.metadata(),
        timeout::COMMAND_DESCRIPTOR.metadata(),
        allbots::COMMAND_DESCRIPTOR.metadata(),
        alladmins::COMMAND_DESCRIPTOR.metadata(),
        botadmins::COMMAND_DESCRIPTOR.metadata(),
        ancien::COMMAND_DESCRIPTOR.metadata(),
        antiraideautoconfig::COMMAND_DESCRIPTOR.metadata(),
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
        clear_limit::COMMAND_DESCRIPTOR.metadata(),
        clear_badwords::COMMAND_DESCRIPTOR.metadata(),
        muterole::COMMAND_DESCRIPTOR.metadata(),
        set_muterole::COMMAND_DESCRIPTOR.metadata(),
        antispam::COMMAND_DESCRIPTOR.metadata(),
        antilink::COMMAND_DESCRIPTOR.metadata(),
        antimassmention::COMMAND_DESCRIPTOR.metadata(),
        badwords::COMMAND_DESCRIPTOR.metadata(),
        spam::COMMAND_DESCRIPTOR.metadata(),
        link::COMMAND_DESCRIPTOR.metadata(),
        strikes::COMMAND_DESCRIPTOR.metadata(),
        punish::COMMAND_DESCRIPTOR.metadata(),
        public::COMMAND_DESCRIPTOR.metadata(),
        resetantiraide::COMMAND_DESCRIPTOR.metadata(),
        backup::COMMAND_DESCRIPTOR.metadata(),
        ticket::COMMAND_DESCRIPTOR.metadata(),
        claim::COMMAND_DESCRIPTOR.metadata(),
        rename::COMMAND_DESCRIPTOR.metadata(),
        ticket_member::COMMAND_DESCRIPTOR.metadata(),
        close::COMMAND_DESCRIPTOR.metadata(),
        tickets::COMMAND_DESCRIPTOR.metadata(),
        showpics::COMMAND_DESCRIPTOR.metadata(),
        piconly::COMMAND_DESCRIPTOR.metadata(),
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
        noderank::COMMAND_DESCRIPTOR.metadata(),
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
    let normalized = key.to_lowercase();
    let compact = normalized.replace('_', "");

    all_command_metadata()
        .into_iter()
        .find(|meta| {
            meta.name.eq_ignore_ascii_case(&normalized)
                || meta.name.replace('_', "").eq_ignore_ascii_case(&compact)
        })
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
