#![allow(dead_code)]

use fenrs::{execute_moves, parse};
use std::fs;

fn main() {
    let file = fs::read_to_string("./samples/Candidates2022.pgn").unwrap();
    let pgns = parse(&file).unwrap();
    for pgn in pgns.iter() {
        let _boards = execute_moves(pgn.fen().starting_board(), pgn.ply()).unwrap();
    }
}
