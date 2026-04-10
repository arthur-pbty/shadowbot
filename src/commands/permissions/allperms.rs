use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed, CreateMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::perms_helpers::ensure_owner;
use crate::commands::{common::theme_color, common::truncate_text};
use crate::permissions::{all_command_keys, command_required_permission, default_permission};

const ALLPERMS_PAGE_SIZE: usize = 12;
const ALLPERMS_CUSTOM_ID_PREFIX: &str = "allperms";

async fn collect_allperms_lines(ctx: &Context) -> Vec<String> {
    let mut commands = all_command_keys();
    if !commands.iter().any(|c| c == "allperms") {
        commands.push("allperms".to_string());
    }
    commands.sort();

    let mut lines = Vec::with_capacity(commands.len());
    for cmd in commands {
        let required = command_required_permission(ctx, &cmd).await;
        let default = default_permission(&cmd);

        if required == default {
            lines.push(format!("`{}` -> `{}`", cmd, required));
        } else {
            lines.push(format!(
                "`{}` -> `{}` (defaut `{}`)",
                cmd, required, default
            ));
        }
    }

    lines
}

fn total_pages_for(total_items: usize) -> usize {
    ((total_items + ALLPERMS_PAGE_SIZE.saturating_sub(1)) / ALLPERMS_PAGE_SIZE).max(1)
}

fn build_allperms_embed(lines: &[String], page: usize, color: u32) -> CreateEmbed {
    let total_pages = total_pages_for(lines.len());
    let safe_page = page.min(total_pages.saturating_sub(1));
    let start = safe_page * ALLPERMS_PAGE_SIZE;
    let end = (start + ALLPERMS_PAGE_SIZE).min(lines.len());
    let chunk = if start < end { &lines[start..end] } else { &[] };

    let value = if chunk.is_empty() {
        "Aucune commande.".to_string()
    } else {
        truncate_text(&chunk.join("\n"), 1024)
    };

    CreateEmbed::new()
        .title("Permissions de toutes les commandes")
        .description(format!(
            "{} commande(s) · Page {}/{}",
            lines.len(),
            safe_page + 1,
            total_pages
        ))
        .field("Niveaux requis", value, false)
        .color(color)
}

fn allperms_components(owner_id: UserId, page: usize, total_pages: usize) -> Vec<CreateActionRow> {
    let safe_total = total_pages.max(1);
    let safe_page = page.min(safe_total.saturating_sub(1));
    let prev_page = safe_page.saturating_sub(1);
    let next_page = (safe_page + 1).min(safe_total.saturating_sub(1));

    vec![CreateActionRow::Buttons(vec![
        CreateButton::new(format!(
            "{}:{}:{}",
            ALLPERMS_CUSTOM_ID_PREFIX,
            owner_id.get(),
            prev_page
        ))
        .label("◀ Precedent")
        .style(ButtonStyle::Primary)
        .disabled(safe_page == 0),
        CreateButton::new(format!(
            "{}:{}:{}",
            ALLPERMS_CUSTOM_ID_PREFIX,
            owner_id.get(),
            next_page
        ))
        .label("Suivant ▶")
        .style(ButtonStyle::Primary)
        .disabled(safe_page + 1 >= safe_total),
    ])]
}

pub async fn handle_allperms(ctx: &Context, msg: &Message, args: &[&str]) {
    if !ensure_owner(ctx, msg).await {
        return;
    }

    let lines = collect_allperms_lines(ctx).await;
    let total_pages = total_pages_for(lines.len());
    let requested_page = args
        .first()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1)
        .saturating_sub(1);
    let page = requested_page.min(total_pages.saturating_sub(1));

    let color = theme_color(ctx).await;
    let embed = build_allperms_embed(&lines, page, color);
    let components = allperms_components(msg.author.id, page, total_pages);

    let _ = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new().embed(embed).components(components),
        )
        .await;
}

pub struct AllpermsCommand;
pub static COMMAND_DESCRIPTOR: AllpermsCommand = AllpermsCommand;

impl crate::commands::command_contract::CommandSpec for AllpermsCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "allperms",
            category: "permissions",
            params: "[page]",
            summary: "Liste les ACL de toutes commandes",
            description: "Affiche le niveau ACL requis pour chaque commande avec pagination.",
            examples: &["+allperms", "+as", "+help allperms"],
            default_aliases: &["apm"],
            allow_in_dm: false,
            default_permission: 8,
        }
    }
}
