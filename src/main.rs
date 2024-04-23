#![allow(dead_code)]

use std::{error::Error, fs};

use fenrs::{execute_moves, launch, parse, pigin, Game, Pgn};

fn main() -> Result<(), Box<dyn Error>> {
    let pigin = pigin();
    let matches = pigin.get_matches();
    let file_name: &String = matches
        .get_one("file")
        .ok_or("'file' argument not provided")?;

    let file = fs::read_to_string(file_name)?;
    let pgns = parse(&file)?;

    let games = pgns
        .into_iter()
        .map(game_from)
        .collect::<Result<Vec<Game>, Box<dyn Error>>>()?;

    launch(games)?;
    Ok(())
}

fn game_from(pgn: Pgn) -> Result<Game, Box<dyn Error>> {
    let boards = execute_moves(pgn.fen().starting_board(), pgn.ply())?;
    Ok(Game::new(pgn, boards))
}
