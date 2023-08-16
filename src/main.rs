#![allow(dead_code)]

use std::fs;

use fenrs::{parse, execute_moves, Game, launch};

fn main() {
    let file = fs::read_to_string("./samples/example.pgn").unwrap();
    let pgn = parse(&file).unwrap().remove(0);
    let boards = execute_moves(pgn.fen().starting_board(), pgn.ply()).unwrap();
    let games = vec![Game::new(pgn, boards)];
    launch(games).unwrap();
}
