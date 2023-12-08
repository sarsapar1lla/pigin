#![allow(dead_code)]

use std::fs;

use fenrs::{execute_moves, launch, parse, Game};

fn main() {
    let file = fs::read_to_string("./resources/test/acceptance/astzhuop23.pgn").unwrap();
    let pgns = parse(&file).unwrap();

    let mut games: Vec<Game> = Vec::new();

    for pgn in pgns {
        let boards = execute_moves(pgn.fen().starting_board(), pgn.ply()).unwrap();
        let game = Game::new(pgn, boards);
        games.push(game);
    }

    launch(games).unwrap();
}
