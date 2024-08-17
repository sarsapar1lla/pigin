use std::{error::Error, fs};

use pigin::{execute_moves, launch, parse, pigin, Game, Pgn};

type PgnsResult = Result<Vec<Pgn>, Box<dyn Error>>;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = pigin().get_matches();
    let file_names: Vec<&String> = matches
        .get_many("file")
        .ok_or("'file' argument not provided")?
        .collect();

    let pgns = file_names
        .into_iter()
        .map(|file_name| pgns_from(file_name))
        .reduce(reduce)
        .unwrap_or_else(|| Ok(Vec::new()))?;

    let games = pgns
        .into_iter()
        .map(game_from)
        .collect::<Result<Vec<Game>, Box<dyn Error>>>()?;

    launch(games)?;
    Ok(())
}

fn pgns_from(file_name: &str) -> PgnsResult {
    let file = fs::read_to_string(file_name)?;
    parse(&file).map_err(|err| err.into())
}

fn game_from(pgn: Pgn) -> Result<Game, Box<dyn Error>> {
    let boards = execute_moves(pgn.fen().starting_board(), pgn.ply())?;
    Ok(Game::new(pgn, boards))
}

fn reduce(result_1: PgnsResult, result_2: PgnsResult) -> PgnsResult {
    match (result_1, result_2) {
        (Ok(mut v1), Ok(mut v2)) => {
            v1.append(&mut v2);
            Ok(v1)
        }
        (Err(e1), Err(e2)) => Err(format!("{e1}, {e2}").into()),
        (Err(e), _) => Err(e.to_string().into()),
        (_, Err(e)) => Err(e.to_string().into()),
    }
}
