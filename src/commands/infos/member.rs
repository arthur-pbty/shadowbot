use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{discord_ts, send_embed, truncate_text};

pub async fn handle_member(ctx: &Context, msg: &Message, args: &[&str]) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let member = if args.is_empty() {
        guild_id.member(&ctx.http, msg.author.id).await.ok()
    } else {
        let user_id = args[0]
            .trim_start_matches('<')
            .trim_end_matches('>')
            .trim_start_matches('@')
            .trim_start_matches('!')
            .parse::<u64>()
            .ok()
            .map(UserId::new);

        if let Some(uid) = user_id {
            guild_id.member(&ctx.http, uid).await.ok()
        } else {
            None
        }
    };

    let Some(member) = member else {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Membre non trouvé.")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    };

    let joined_at = discord_ts(
        member.joined_at.unwrap_or_else(|| member.user.created_at()),
        "F",
    );
    let created_at = discord_ts(member.user.created_at(), "F");
    let avatar_url = member.user.avatar_url().unwrap_or_default();

    let roles_str = if member.roles.is_empty() {
        "@everyone".to_string()
    } else {
        let roles_list: Vec<String> = member
            .roles
            .iter()
            .map(|r| format!("<@&{}>", r.get()))
            .collect();
        truncate_text(&roles_list.join(", "), 1024)
    };

    let mut embed = CreateEmbed::new()
        .title(&member.user.name)
        .description(format!("ID: `{}`", member.user.id.get()))
        .color(0x5865F2)
        .thumbnail(&avatar_url)
        .field("Compte créé", created_at, true)
        .field("A rejoint", joined_at, true)
        .field("Rôles", roles_str, false);

    if let Some(nick) = &member.nick {
        embed = embed.field("Surnom", nick, true);
    }

    send_embed(ctx, msg, embed).await;
}

pub struct MemberCommand;
pub static COMMAND_DESCRIPTOR: MemberCommand = MemberCommand;

impl crate::commands::command_contract::CommandSpec for MemberCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "member",
            category: "infos",
            params: "<@membre/ID>",
            description: "Affiche les informations dun membre dans le serveur courant.",
            examples: &["+member", "+mr", "+help member"],
            default_aliases: &["mbr"],
            allow_in_dm: false,
            default_permission: 0,
        }
    }
}
