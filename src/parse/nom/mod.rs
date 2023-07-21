mod error;
mod fen;
mod movement;
mod ply;
mod position;
mod result;
mod tag;

use nom::sequence::pair;
use nom::combinator::all_consuming;

use crate::{model::Pgn, parse::nom::tag::parse_tags};

use self::{error::PgnParseError, fen::parse_fen, movement::parse_moves, result::parse_result};

pub static DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn parse(input: &str) -> Result<Pgn, PgnParseError> {
    let (_, (mut tags, ply)) =
        all_consuming(pair(parse_tags, parse_moves))(input)
            .map_err(|e| PgnParseError::new(e.to_string()))?;

    let fen = &tags
        .remove("FEN")
        .unwrap_or_else(|| DEFAULT_FEN.to_string());

    let (_, fen) = parse_fen(fen)
        .map_err(|e| PgnParseError::new(format!("Failed to parse FEN string: {e}")))?;

    let result = &tags
        .remove("Result")
        .ok_or_else(|| PgnParseError::new("Missing 'Result' tag".to_string()))?;

    let (_, result) = parse_result(result)
        .map_err(|e| PgnParseError::new(format!("Failed to parse result: {e}")))?;

    Ok(Pgn::new(tags, fen, result, ply))
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use crate::model::{
        AvailableCastle, BoardBuilder, Fen, GameResult, Movement, Piece, PieceColour, PieceType,
        Ply, PlyMetadata, Position, Tags,
    };

    use super::*;

    #[test]
    fn parses_pgn() {
        let file = fs::read_to_string("./resources/test/test.pgn").unwrap();
        let pgn = parse(&file).unwrap();

        assert_eq!(pgn, expected());
    }

    fn expected() -> Pgn {
        let mut tags: Tags = HashMap::new();
        tags.insert("White".to_string(), "Player, One".to_string());
        tags.insert("Black".to_string(), "Player, Two".to_string());

        let ply_list = vec![
            PlyMetadata::new(
                1,
                Ply::Move {
                    movement: Movement::new(
                        PieceType::Pawn,
                        PieceColour::White,
                        Position::new(3, 4).unwrap(),
                    ),
                    qualifier: None,
                    check: None,
                },
                None,
            ),
            PlyMetadata::new(
                1,
                Ply::Move {
                    movement: Movement::new(
                        PieceType::Pawn,
                        PieceColour::Black,
                        Position::new(4, 4).unwrap(),
                    ),
                    qualifier: None,
                    check: None,
                },
                None,
            ),
        ];

        Pgn::new(tags, expected_fen(), GameResult::Ongoing, ply_list)
    }

    fn expected_fen() -> Fen {
        let mut board_builder = BoardBuilder::new();
        board_builder
            .available_castles(vec![AvailableCastle::BlackKingside])
            .piece(
                Piece::new(PieceColour::Black, PieceType::Rook),
                Position::new(7, 0).unwrap(),
            );

        let board = board_builder.build();

        Fen::new(board, PieceColour::White, 1)
    }
}
