#![allow(dead_code)]

use fenrs::{execute_moves, parse};
use std::fs;

fn main() {
    let file = fs::read_to_string("./resources/test/acceptance/chess_com_games_2023-08-13.pgn").unwrap();
    let pgns = parse(&file);
    match pgns {
        Err(error) => {
            let message = error.message();
            let length = message.len();
            println!(
                "{:?}, {:?}",
                message.get(0..500),
                message.get(length - 100..length)
            )
        }
        Ok(_) => {}
    }
    // for pgn in pgns.iter() {
    //     let _boards = execute_moves(pgn.fen().starting_board(), pgn.ply()).unwrap();
    // }
}
