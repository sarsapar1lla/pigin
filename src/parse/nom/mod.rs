mod error;
mod fen;
mod movement;
mod ply;
mod position;
mod tag;

use nom::sequence::pair;

use crate::{model::Pgn, parse::nom::tag::parse_tags};

use self::movement::parse_moves;

use super::error::ParseError;

pub static DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn parse() -> Result<Pgn, ParseError> {
    let file = "[Tag \"Value\"]\n[Another \"Something\"]\n\n1. e4 e5 {This is a comment} 2.Nc3 Nf6";
    let (remaining, (mut tags, ply)) =
        pair(parse_tags, parse_moves)(file).map_err(|e| ParseError(e.to_string()))?;

    if !remaining.is_empty() {
        return Err(ParseError(format!(
            "File contains unconsumed content: '{remaining}'"
        )));
    }

    let fen = &tags
        .remove("FEN")
        .unwrap_or_else(|| DEFAULT_FEN.to_string());

    let result = &tags
        .remove("Result")
        .ok_or_else(|| ParseError("Missing 'Result' tag".to_string()));

    // Ok(Pgn::new(tags, fen, result, ply));

    todo!()
}
