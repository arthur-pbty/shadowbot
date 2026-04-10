use rand::seq::SliceRandom;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

const SIZE: usize = 5;
const MINES: usize = 5;

pub async fn handle_demineur(ctx: &Context, msg: &Message, _args: &[&str]) {
    let mut mine_positions = (0..(SIZE * SIZE)).collect::<Vec<_>>();
    {
        let mut rng = rand::thread_rng();
        mine_positions.shuffle(&mut rng);
    }
    mine_positions.truncate(MINES);

    let mut board = vec![vec![0u8; SIZE]; SIZE];
    for index in &mine_positions {
        let row = index / SIZE;
        let col = index % SIZE;
        board[row][col] = 9;
    }

    for row in 0..SIZE {
        for col in 0..SIZE {
            if board[row][col] == 9 {
                continue;
            }
            let mut count = 0u8;
            for dr in -1isize..=1 {
                for dc in -1isize..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let nr = row as isize + dr;
                    let nc = col as isize + dc;
                    if nr >= 0
                        && nr < SIZE as isize
                        && nc >= 0
                        && nc < SIZE as isize
                        && board[nr as usize][nc as usize] == 9
                    {
                        count += 1;
                    }
                }
            }
            board[row][col] = count;
        }
    }

    let rendered = board
        .iter()
        .map(|line| {
            line.iter()
                .map(|cell| {
                    if *cell == 9 {
                        "||*||".to_string()
                    } else {
                        format!("||{}||", cell)
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n");

    let embed = CreateEmbed::new()
        .title("Demineur")
        .description(format!(
            "Plateau genere. Clique les spoilers pour reveler les cases.\n\n{}",
            rendered
        ))
        .color(theme_color(ctx).await);

    send_embed(ctx, msg, embed).await;
}

pub struct DemineurCommand;
pub static COMMAND_DESCRIPTOR: DemineurCommand = DemineurCommand;

impl crate::commands::command_contract::CommandSpec for DemineurCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "demineur",
            category: "game",
            params: "aucun",
            description: "Jouer a un jeu demineur.",
            examples: &["+demineur"],
            default_aliases: &["mine"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
