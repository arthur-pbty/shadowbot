use rand::Rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serenity::all::{ButtonStyle, ComponentInteraction};
use serenity::builder::{
    CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::theme_color;
use crate::db;

const GAME_KIND: &str = "2048";
const GAME_PREFIX: &str = "game:2048";
const SIZE: usize = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Game2048State {
    board: Vec<u16>,
    owner_id: i64,
    score: u32,
    over: bool,
}

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<db::DbPoolKey>().cloned()
}

fn parse_component_id(custom_id: &str) -> Option<(i64, String)> {
    let mut parts = custom_id.split(':');
    let scope = parts.next()?;
    let game = parts.next()?;
    let session_id = parts.next()?.parse::<i64>().ok()?;
    let action = parts.next()?.to_string();

    if scope != "game" || game != "2048" || parts.next().is_some() {
        return None;
    }

    Some((session_id, action))
}

fn create_fresh_board() -> Vec<u16> {
    let mut board = vec![0u16; SIZE * SIZE];
    let _ = spawn_random_tile(&mut board);
    let _ = spawn_random_tile(&mut board);
    board
}

fn spawn_random_tile(board: &mut [u16]) -> bool {
    let empties = board
        .iter()
        .enumerate()
        .filter_map(|(idx, value)| if *value == 0 { Some(idx) } else { None })
        .collect::<Vec<_>>();

    if empties.is_empty() {
        return false;
    }

    let picked = {
        let mut rng = rand::thread_rng();
        empties.choose(&mut rng).copied()
    };

    let Some(index) = picked else {
        return false;
    };

    let value = {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.9) { 2 } else { 4 }
    };

    board[index] = value;
    true
}

fn slide_merge_line(input: [u16; SIZE]) -> ([u16; SIZE], u32, bool) {
    let non_zero = input
        .iter()
        .copied()
        .filter(|value| *value != 0)
        .collect::<Vec<_>>();

    let mut merged = Vec::new();
    let mut score_gain = 0u32;
    let mut index = 0usize;

    while index < non_zero.len() {
        if index + 1 < non_zero.len() && non_zero[index] == non_zero[index + 1] {
            let value = non_zero[index] * 2;
            merged.push(value);
            score_gain = score_gain.saturating_add(value as u32);
            index += 2;
        } else {
            merged.push(non_zero[index]);
            index += 1;
        }
    }

    while merged.len() < SIZE {
        merged.push(0);
    }

    let output = [merged[0], merged[1], merged[2], merged[3]];
    let changed = output != input;
    (output, score_gain, changed)
}

fn move_left(board: &mut [u16]) -> (bool, u32) {
    let mut changed = false;
    let mut gain = 0u32;

    for row in 0..SIZE {
        let base = row * SIZE;
        let line = [
            board[base],
            board[base + 1],
            board[base + 2],
            board[base + 3],
        ];
        let (new_line, score, line_changed) = slide_merge_line(line);
        board[base] = new_line[0];
        board[base + 1] = new_line[1];
        board[base + 2] = new_line[2];
        board[base + 3] = new_line[3];
        changed |= line_changed;
        gain = gain.saturating_add(score);
    }

    (changed, gain)
}

fn move_right(board: &mut [u16]) -> (bool, u32) {
    let mut changed = false;
    let mut gain = 0u32;

    for row in 0..SIZE {
        let base = row * SIZE;
        let line = [
            board[base + 3],
            board[base + 2],
            board[base + 1],
            board[base],
        ];
        let (new_line, score, line_changed) = slide_merge_line(line);
        board[base + 3] = new_line[0];
        board[base + 2] = new_line[1];
        board[base + 1] = new_line[2];
        board[base] = new_line[3];
        changed |= line_changed;
        gain = gain.saturating_add(score);
    }

    (changed, gain)
}

fn move_up(board: &mut [u16]) -> (bool, u32) {
    let mut changed = false;
    let mut gain = 0u32;

    for col in 0..SIZE {
        let line = [
            board[col],
            board[SIZE + col],
            board[(2 * SIZE) + col],
            board[(3 * SIZE) + col],
        ];
        let (new_line, score, line_changed) = slide_merge_line(line);
        board[col] = new_line[0];
        board[SIZE + col] = new_line[1];
        board[(2 * SIZE) + col] = new_line[2];
        board[(3 * SIZE) + col] = new_line[3];
        changed |= line_changed;
        gain = gain.saturating_add(score);
    }

    (changed, gain)
}

fn move_down(board: &mut [u16]) -> (bool, u32) {
    let mut changed = false;
    let mut gain = 0u32;

    for col in 0..SIZE {
        let line = [
            board[(3 * SIZE) + col],
            board[(2 * SIZE) + col],
            board[SIZE + col],
            board[col],
        ];
        let (new_line, score, line_changed) = slide_merge_line(line);
        board[(3 * SIZE) + col] = new_line[0];
        board[(2 * SIZE) + col] = new_line[1];
        board[SIZE + col] = new_line[2];
        board[col] = new_line[3];
        changed |= line_changed;
        gain = gain.saturating_add(score);
    }

    (changed, gain)
}

fn has_possible_moves(board: &[u16]) -> bool {
    if board.iter().any(|value| *value == 0) {
        return true;
    }

    for row in 0..SIZE {
        for col in 0..SIZE {
            let current = board[(row * SIZE) + col];

            if col + 1 < SIZE && board[(row * SIZE) + (col + 1)] == current {
                return true;
            }

            if row + 1 < SIZE && board[((row + 1) * SIZE) + col] == current {
                return true;
            }
        }
    }

    false
}

fn apply_action(state: &mut Game2048State, action: &str) -> Result<(), &'static str> {
    if action == "reset" {
        state.board = create_fresh_board();
        state.score = 0;
        state.over = false;
        return Ok(());
    }

    if state.over {
        return Err("La partie est terminee. Utilise le bouton Reset.");
    }

    let (changed, gain) = match action {
        "left" => move_left(&mut state.board),
        "right" => move_right(&mut state.board),
        "up" => move_up(&mut state.board),
        "down" => move_down(&mut state.board),
        _ => return Err("Action inconnue."),
    };

    if !changed {
        return Err("Coup impossible dans cette direction.");
    }

    state.score = state.score.saturating_add(gain);
    let _ = spawn_random_tile(&mut state.board);
    state.over = !has_possible_moves(&state.board);
    Ok(())
}

fn board_text(board: &[u16]) -> String {
    let mut lines = Vec::new();

    for row in 0..SIZE {
        let mut cells = Vec::new();
        for col in 0..SIZE {
            let value = board[(row * SIZE) + col];
            if value == 0 {
                cells.push(format!("{:>5}", "."));
            } else {
                cells.push(format!("{:>5}", value));
            }
        }
        lines.push(cells.join(" "));
    }

    lines.join("\n")
}

fn game_components(session_id: i64, over: bool) -> Vec<CreateActionRow> {
    vec![
        CreateActionRow::Buttons(vec![
            CreateButton::new(format!("{}:{}:{}", GAME_PREFIX, session_id, "up"))
                .label("Haut")
                .style(ButtonStyle::Primary)
                .disabled(over),
            CreateButton::new(format!("{}:{}:{}", GAME_PREFIX, session_id, "reset"))
                .label("Reset")
                .style(ButtonStyle::Danger),
        ]),
        CreateActionRow::Buttons(vec![
            CreateButton::new(format!("{}:{}:{}", GAME_PREFIX, session_id, "left"))
                .label("Gauche")
                .style(ButtonStyle::Primary)
                .disabled(over),
            CreateButton::new(format!("{}:{}:{}", GAME_PREFIX, session_id, "down"))
                .label("Bas")
                .style(ButtonStyle::Primary)
                .disabled(over),
            CreateButton::new(format!("{}:{}:{}", GAME_PREFIX, session_id, "right"))
                .label("Droite")
                .style(ButtonStyle::Primary)
                .disabled(over),
        ]),
    ]
}

fn game_embed(session_id: i64, state: &Game2048State, color: u32) -> CreateEmbed {
    CreateEmbed::new()
        .title("2048 interactif")
        .description(format!(
            "Session `#{}`\n\n```\n{}\n```",
            session_id,
            board_text(&state.board)
        ))
        .field("Score", state.score.to_string(), true)
        .field(
            "Etat",
            if state.over { "Terminee" } else { "En cours" },
            true,
        )
        .color(color)
}

async fn send_ephemeral(ctx: &Context, component: &ComponentInteraction, content: &str) {
    let _ = component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(content)
                    .ephemeral(true),
            ),
        )
        .await;
}

pub async fn handle_2048(ctx: &Context, msg: &Message, _args: &[&str]) {
    let Some(pool) = pool(ctx).await else {
        let _ = msg
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().embed(
                    CreateEmbed::new()
                        .title("2048")
                        .description("Base de donnees indisponible, impossible de demarrer une session interactive.")
                        .color(0xED4245),
                ),
            )
            .await;
        return;
    };

    let owner_id = msg.author.id.get() as i64;
    let state = Game2048State {
        board: create_fresh_board(),
        owner_id,
        score: 0,
        over: false,
    };

    let participants_json =
        serde_json::to_string(&vec![owner_id]).unwrap_or_else(|_| "[]".to_string());
    let state_json = serde_json::to_string(&state).unwrap_or_else(|_| "{}".to_string());
    let bot_id = ctx.cache.current_user().id.get() as i64;

    let Ok(session) = db::create_game_session(
        &pool,
        bot_id,
        msg.guild_id.map(|id| id.get() as i64),
        msg.channel_id.get() as i64,
        owner_id,
        GAME_KIND,
        &participants_json,
        &state_json,
    )
    .await
    else {
        return;
    };

    let color = theme_color(ctx).await;
    let sent = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .embed(game_embed(session.id, &state, color))
                .components(game_components(session.id, state.over)),
        )
        .await;

    if let Ok(message) = sent {
        let _ = db::set_game_session_message(&pool, session.id, message.id.get() as i64).await;
    }
}

pub async fn handle_component_interaction(ctx: &Context, component: &ComponentInteraction) -> bool {
    if !component.data.custom_id.starts_with(GAME_PREFIX) {
        return false;
    }

    let Some((session_id, action)) = parse_component_id(&component.data.custom_id) else {
        return false;
    };

    let Some(pool) = pool(ctx).await else {
        send_ephemeral(ctx, component, "Base de donnees indisponible.").await;
        return true;
    };

    let Ok(Some(session)) = db::get_game_session(&pool, session_id).await else {
        send_ephemeral(ctx, component, "Session introuvable.").await;
        return true;
    };

    if session.game_type != GAME_KIND {
        return false;
    }

    let Ok(mut state) = serde_json::from_str::<Game2048State>(&session.state_json) else {
        send_ephemeral(ctx, component, "Etat de session invalide.").await;
        return true;
    };

    let actor_id = component.user.id.get() as i64;
    if actor_id != state.owner_id {
        send_ephemeral(ctx, component, "Seul le createur de la partie peut jouer.").await;
        return true;
    }

    if let Err(message) = apply_action(&mut state, &action) {
        send_ephemeral(ctx, component, message).await;
        return true;
    }

    let status = if state.over { "finished" } else { "active" };
    let state_json = serde_json::to_string(&state).unwrap_or_else(|_| session.state_json.clone());
    let _ = db::update_game_session_state(&pool, session.id, &state_json, status).await;

    let color = theme_color(ctx).await;
    let _ = component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embed(game_embed(session.id, &state, color))
                    .components(game_components(session.id, state.over)),
            ),
        )
        .await;

    true
}

pub struct Game2048Command;
pub static COMMAND_DESCRIPTOR: Game2048Command = Game2048Command;

impl crate::commands::command_contract::CommandSpec for Game2048Command {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "2048",
            category: "game",
            params: "aucun",
            description: "Jouer au jeu 2048.",
            examples: &["+2048"],
            default_aliases: &["g2048"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
