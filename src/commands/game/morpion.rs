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

const GAME_KIND: &str = "morpion";
const GAME_PREFIX: &str = "game:morpion";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MorpionState {
    board: Vec<u8>,
    player_x: i64,
    player_o: i64,
    current_turn: i64,
    winner: i64,
    moves: u8,
    vs_bot: bool,
}

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<db::DbPoolKey>().cloned()
}

fn parse_component_id(custom_id: &str) -> Option<(i64, usize)> {
    let mut parts = custom_id.split(':');
    let scope = parts.next()?;
    let game = parts.next()?;
    let session_id = parts.next()?.parse::<i64>().ok()?;
    let cell = parts.next()?.parse::<usize>().ok()?;

    if scope != "game" || game != "morpion" || cell >= 9 || parts.next().is_some() {
        return None;
    }

    Some((session_id, cell))
}

fn player_name(id: i64) -> String {
    if id == 0 {
        "Bot".to_string()
    } else {
        format!("<@{}>", id)
    }
}

fn render_board(state: &MorpionState) -> String {
    let mut rows = Vec::new();

    for row in 0..3 {
        let mut values = Vec::new();
        for col in 0..3 {
            let index = row * 3 + col;
            let value = state.board[index];
            let label = match value {
                1 => "X".to_string(),
                2 => "O".to_string(),
                _ => (index + 1).to_string(),
            };
            values.push(label);
        }
        rows.push(values.join(" | "));
    }

    rows.join("\n---------\n")
}

fn has_winner(board: &[u8], mark: u8) -> bool {
    const PATTERNS: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    PATTERNS
        .iter()
        .any(|pattern| pattern.iter().all(|index| board[*index] == mark))
}

fn available_cells(board: &[u8]) -> Vec<usize> {
    board
        .iter()
        .enumerate()
        .filter_map(|(index, value)| if *value == 0 { Some(index) } else { None })
        .collect()
}

fn apply_player_move(
    state: &mut MorpionState,
    cell: usize,
    actor_id: i64,
) -> Result<(), &'static str> {
    if state.winner != 0 {
        return Err("La partie est deja terminee.");
    }

    if state.current_turn != actor_id {
        return Err("Ce n'est pas ton tour.");
    }

    if state.board.get(cell).copied().unwrap_or(9) != 0 {
        return Err("Cette case est deja occupee.");
    }

    let mark = if actor_id == state.player_x {
        1
    } else if actor_id == state.player_o {
        2
    } else {
        return Err("Tu n'es pas un joueur de cette partie.");
    };

    state.board[cell] = mark;
    state.moves = state.moves.saturating_add(1);

    if has_winner(&state.board, mark) {
        state.winner = if state.vs_bot && mark == 2 {
            -2
        } else {
            actor_id
        };
        state.current_turn = 0;
        return Ok(());
    }

    if state.moves >= 9 {
        state.winner = -1;
        state.current_turn = 0;
        return Ok(());
    }

    state.current_turn = if actor_id == state.player_x {
        state.player_o
    } else {
        state.player_x
    };

    Ok(())
}

fn apply_bot_move(state: &mut MorpionState) {
    if !state.vs_bot || state.winner != 0 || state.current_turn != 0 {
        return;
    }

    let empties = available_cells(&state.board);
    let bot_cell = {
        let mut rng = rand::thread_rng();
        empties.choose(&mut rng).copied()
    };

    let Some(cell) = bot_cell else {
        state.winner = -1;
        return;
    };

    state.board[cell] = 2;
    state.moves = state.moves.saturating_add(1);

    if has_winner(&state.board, 2) {
        state.winner = -2;
        state.current_turn = 0;
        return;
    }

    if state.moves >= 9 {
        state.winner = -1;
        state.current_turn = 0;
        return;
    }

    state.current_turn = state.player_x;
}

fn game_components(session_id: i64, state: &MorpionState) -> Vec<CreateActionRow> {
    let mut rows = Vec::new();

    for row in 0..3 {
        let mut buttons = Vec::new();

        for col in 0..3 {
            let index = row * 3 + col;
            let value = state.board[index];
            let label = match value {
                1 => "X".to_string(),
                2 => "O".to_string(),
                _ => (index + 1).to_string(),
            };

            let style = match value {
                1 => ButtonStyle::Danger,
                2 => ButtonStyle::Success,
                _ => ButtonStyle::Secondary,
            };

            let disabled = state.winner != 0 || value != 0;
            buttons.push(
                CreateButton::new(format!("{}:{}:{}", GAME_PREFIX, session_id, index))
                    .label(label)
                    .style(style)
                    .disabled(disabled),
            );
        }

        rows.push(CreateActionRow::Buttons(buttons));
    }

    rows
}

fn game_embed(session_id: i64, state: &MorpionState, color: u32) -> CreateEmbed {
    let status = if state.winner == 0 {
        format!("Tour de {}.", player_name(state.current_turn))
    } else if state.winner == -1 {
        "Match nul.".to_string()
    } else if state.winner == -2 {
        "Le bot gagne.".to_string()
    } else {
        format!("Victoire de {}.", player_name(state.winner))
    };

    CreateEmbed::new()
        .title("Morpion interactif")
        .description(format!(
            "Session `#{}`\n\n```\n{}\n```",
            session_id,
            render_board(state)
        ))
        .field("Joueur X", player_name(state.player_x), true)
        .field("Joueur O", player_name(state.player_o), true)
        .field("Etat", status, false)
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

pub async fn handle_morpion(ctx: &Context, msg: &Message, _args: &[&str]) {
    let Some(pool) = pool(ctx).await else {
        let _ = msg
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().embed(
                    CreateEmbed::new()
                        .title("Morpion")
                        .description("Base de donnees indisponible, impossible de demarrer une session interactive.")
                        .color(0xED4245),
                ),
            )
            .await;
        return;
    };

    let player_x = msg.author.id.get() as i64;
    let player_o = msg
        .mentions
        .first()
        .filter(|user| user.id != msg.author.id)
        .map(|user| user.id.get() as i64)
        .unwrap_or(0);
    let vs_bot = player_o == 0;

    let state = MorpionState {
        board: vec![0; 9],
        player_x,
        player_o,
        current_turn: player_x,
        winner: 0,
        moves: 0,
        vs_bot,
    };

    let participants = if vs_bot {
        vec![player_x]
    } else {
        vec![player_x, player_o]
    };

    let participants_json =
        serde_json::to_string(&participants).unwrap_or_else(|_| "[]".to_string());
    let state_json = serde_json::to_string(&state).unwrap_or_else(|_| "{}".to_string());
    let bot_id = ctx.cache.current_user().id.get() as i64;

    let Ok(session) = db::create_game_session(
        &pool,
        bot_id,
        msg.guild_id.map(|id| id.get() as i64),
        msg.channel_id.get() as i64,
        player_x,
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
                .components(game_components(session.id, &state)),
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

    let Some((session_id, cell)) = parse_component_id(&component.data.custom_id) else {
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

    let Ok(mut state) = serde_json::from_str::<MorpionState>(&session.state_json) else {
        send_ephemeral(ctx, component, "Etat de session invalide.").await;
        return true;
    };

    if session.status != "active" || state.winner != 0 {
        let color = theme_color(ctx).await;
        let _ = component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(game_embed(session.id, &state, color))
                        .components(game_components(session.id, &state)),
                ),
            )
            .await;
        return true;
    }

    let actor_id = component.user.id.get() as i64;

    if state.vs_bot {
        if actor_id != state.player_x {
            send_ephemeral(
                ctx,
                component,
                "Seul le createur de la partie peut jouer contre le bot.",
            )
            .await;
            return true;
        }
    } else if actor_id != state.current_turn {
        send_ephemeral(ctx, component, "Ce n'est pas ton tour.").await;
        return true;
    }

    if let Err(error) = apply_player_move(&mut state, cell, actor_id) {
        send_ephemeral(ctx, component, error).await;
        return true;
    }

    if state.vs_bot && state.winner == 0 {
        state.current_turn = 0;
        apply_bot_move(&mut state);
    }

    let status = if state.winner == 0 {
        "active"
    } else {
        "finished"
    };
    let state_json = serde_json::to_string(&state).unwrap_or_else(|_| session.state_json.clone());
    let _ = db::update_game_session_state(&pool, session.id, &state_json, status).await;

    let color = theme_color(ctx).await;
    let _ = component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embed(game_embed(session.id, &state, color))
                    .components(game_components(session.id, &state)),
            ),
        )
        .await;

    true
}

pub struct MorpionCommand;
pub static COMMAND_DESCRIPTOR: MorpionCommand = MorpionCommand;

impl crate::commands::command_contract::CommandSpec for MorpionCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "morpion",
            category: "game",
            params: "aucun",
            description: "Jouer a morpion.",
            examples: &["+morpion"],
            default_aliases: &["tic", "tactoe"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
