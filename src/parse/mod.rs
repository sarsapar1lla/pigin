mod error;
mod fen;
mod movement;
mod pgn;
mod ply;
mod position;
mod result;
mod tag;

use nom::{combinator::all_consuming, multi::many1};

use crate::model::Pgn;

use self::error::PgnParseError;

pub fn parse(input: &str) -> Result<Vec<Pgn>, PgnParseError> {
    let (_, pgns) = all_consuming(many1(pgn::parse))(input)
        .map_err(|e| PgnParseError::new(format!("Failed to parse games: {e}")))?;

    Ok(pgns)
}
