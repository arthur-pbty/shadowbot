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

const GAME_KIND: &str = "puissance4";
const GAME_PREFIX: &str = "game:p4";
const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Puissance4State {
    board: Vec<u8>,
    player_red: i64,
    player_yellow: i64,
    current_turn: i64,
    winner: i64,
    moves: u8,
    vs_bot: bool,
}

async fn pool(ctx: &Context) -> Option<sqlx::PgPool> {
    let data = ctx.data.read().await;
    data.get::<db::DbPoolKey>().cloned()
}

fn index(row: usize, col: usize) -> usize {
    row * WIDTH + col
}

fn parse_component_id(custom_id: &str) -> Option<(i64, usize)> {
    let mut parts = custom_id.split(':');
    let scope = parts.next()?;
    let game = parts.next()?;
    let session_id = parts.next()?.parse::<i64>().ok()?;
    let col = parts.next()?.parse::<usize>().ok()?;

    if scope != "game" || game != "p4" || col >= WIDTH || parts.next().is_some() {
        return None;
    }

    Some((session_id, col))
}

fn player_name(id: i64) -> String {
    if id == 0 {
        "Bot".to_string()
    } else {
        format!("<@{}>", id)
    }
}

fn board_text(state: &Puissance4State) -> String {
    let mut lines = Vec::new();

    for row in 0..HEIGHT {
        let mut values = Vec::new();
        for col in 0..WIDTH {
            let value = state.board[index(row, col)];
            let cell = match value {
                1 => "R",
                2 => "Y",
                _ => ".",
            };
            values.push(cell.to_string());
        }
        lines.push(format!("| {} |", values.join(" ")));
    }

    lines.push("  1 2 3 4 5 6 7".to_string());
    lines.join("\n")
}

fn drop_piece(board: &mut [u8], col: usize, mark: u8) -> Option<usize> {
    for row in (0..HEIGHT).rev() {
        let idx = index(row, col);
        if board[idx] == 0 {
            board[idx] = mark;
            return Some(row);
        }
    }

    None
}

fn check_winner_at(board: &[u8], row: usize, col: usize, mark: u8) -> bool {
    let directions = [(1isize, 0isize), (0, 1), (1, 1), (1, -1)];

    for (dr, dc) in directions {
        let mut count = 1;

        for sign in [-1isize, 1isize] {
            let mut r = row as isize;
            let mut c = col as isize;

            loop {
                r += dr * sign;
                c += dc * sign;

                if r < 0 || r >= HEIGHT as isize || c < 0 || c >= WIDTH as isize {
                    break;
                }

                if board[index(r as usize, c as usize)] != mark {
                    break;
                }

                count += 1;
            }
        }

        if count >= 4 {
            return true;
        }
    }

    false
}

fn valid_columns(board: &[u8]) -> Vec<usize> {
    (0..WIDTH)
        .filter(|col| board[index(0, *col)] == 0)
        .collect::<Vec<_>>()
}

fn apply_turn(state: &mut Puissance4State, col: usize, actor_id: i64) -> Result<(), &'static str> {
    if state.winner != 0 {
        return Err("La partie est deja terminee.");
    }

    if state.current_turn != actor_id {
        return Err("Ce n'est pas ton tour.");
    }

    let mark = if actor_id == state.player_red {
        1
    } else if actor_id == state.player_yellow {
        2
    } else {
        return Err("Tu n'es pas un joueur de cette partie.");
    };

    let Some(row) = drop_piece(&mut state.board, col, mark) else {
        return Err("Cette colonne est pleine.");
    };

    state.moves = state.moves.saturating_add(1);

    if check_winner_at(&state.board, row, col, mark) {
        state.winner = if state.vs_bot && mark == 2 {
            -2
        } else {
            actor_id
        };
        state.current_turn = 0;
        return Ok(());
    }

    if state.moves as usize >= WIDTH * HEIGHT {
        state.winner = -1;
        state.current_turn = 0;
        return Ok(());
    }

    state.current_turn = if actor_id == state.player_red {
        state.player_yellow
    } else {
        state.player_red
    };

    Ok(())
}

fn apply_bot_turn(state: &mut Puissance4State) {
    if !state.vs_bot || state.winner != 0 || state.current_turn != 0 {
        return;
    }

    let valid = valid_columns(&state.board);
    let col = {
        let mut rng = rand::thread_rng();
        valid.choose(&mut rng).copied()
    };

    let Some(col) = col else {
        state.winner = -1;
        return;
    };

    let Some(row) = drop_piece(&mut state.board, col, 2) else {
        state.winner = -1;
        return;
    };

    state.moves = state.moves.saturating_add(1);

    if check_winner_at(&state.board, row, col, 2) {
        state.winner = -2;
        state.current_turn = 0;
        return;
    }

    if state.moves as usize >= WIDTH * HEIGHT {
        state.winner = -1;
        state.current_turn = 0;
        return;
    }

    state.current_turn = state.player_red;
}

fn game_components(session_id: i64, state: &Puissance4State) -> Vec<CreateActionRow> {
    let valid = valid_columns(&state.board);
    let mut first_row = Vec::new();
    let mut second_row = Vec::new();

    for col in 0..WIDTH {
        let button = CreateButton::new(format!("{}:{}:{}", GAME_PREFIX, session_id, col))
            .label((col + 1).to_string())
            .style(ButtonStyle::Primary)
            .disabled(state.winner != 0 || !valid.contains(&col));

        if col < 4 {
            first_row.push(button);
        } else {
            second_row.push(button);
        }
    }

    vec![
        CreateActionRow::Buttons(first_row),
        CreateActionRow::Buttons(second_row),
    ]
}

fn game_embed(session_id: i64, state: &Puissance4State, color: u32) -> CreateEmbed {
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
        .title("Puissance4 interactif")
        .description(format!(
            "Session `#{}`\n\n```\n{}\n```",
            session_id,
            board_text(state)
        ))
        .field("Rouge", player_name(state.player_red), true)
        .field("Jaune", player_name(state.player_yellow), true)
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

pub async fn handle_puissance4(ctx: &Context, msg: &Message, _args: &[&str]) {
    let Some(pool) = pool(ctx).await else {
        let _ = msg
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().embed(
                    CreateEmbed::new()
                        .title("Puissance4")
                        .description("Base de donnees indisponible, impossible de demarrer une session interactive.")
                        .color(0xED4245),
                ),
            )
            .await;
        return;
    };

    let player_red = msg.author.id.get() as i64;
    let player_yellow = msg
        .mentions
        .first()
        .filter(|user| user.id != msg.author.id)
        .map(|user| user.id.get() as i64)
        .unwrap_or(0);
    let vs_bot = player_yellow == 0;

    let state = Puissance4State {
        board: vec![0; WIDTH * HEIGHT],
        player_red,
        player_yellow,
        current_turn: player_red,
        winner: 0,
        moves: 0,
        vs_bot,
    };

    let participants = if vs_bot {
        vec![player_red]
    } else {
        vec![player_red, player_yellow]
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
        player_red,
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

    let Some((session_id, col)) = parse_component_id(&component.data.custom_id) else {
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

    let Ok(mut state) = serde_json::from_str::<Puissance4State>(&session.state_json) else {
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
        if actor_id != state.player_red {
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

    if let Err(error) = apply_turn(&mut state, col, actor_id) {
        send_ephemeral(ctx, component, error).await;
        return true;
    }

    if state.vs_bot && state.winner == 0 {
        state.current_turn = 0;
        apply_bot_turn(&mut state);
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

pub struct Puissance4Command;
pub static COMMAND_DESCRIPTOR: Puissance4Command = Puissance4Command;

impl crate::commands::command_contract::CommandSpec for Puissance4Command {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "puissance4",
            category: "game",
            params: "aucun",
            description: "Lancer une partie de puissance4.",
            examples: &["+puissance4"],
            default_aliases: &["connect4", "p4"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
