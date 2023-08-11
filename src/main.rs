#![allow(dead_code)]

use fenrs::{execute_moves, parse};
use std::fs;

fn main() {
    let file = fs::read_to_string("./resources/candidates_test/game_24.pgn").unwrap();
    let pgn = parse(&file).unwrap();
    let _boards = execute_moves(pgn.fen().starting_board(), pgn.ply()).unwrap();
}
