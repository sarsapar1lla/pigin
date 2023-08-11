mod error;
mod fen;
mod movement;
mod ply;
mod position;
mod result;
mod tag;

use nom::multi::many0;
use nom::sequence::{pair, terminated};
use nom::{character::complete::line_ending, combinator::all_consuming};

use crate::model::Pgn;

use self::error::PgnParseError;

pub static DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn parse(input: &str) -> Result<Pgn, PgnParseError> {
    let (_, (mut tags, ply)) = all_consuming(terminated(
        pair(tag::parse, movement::parse),
        many0(line_ending),
    ))(input)
    .map_err(|e| PgnParseError::new(format!("Failed to parse tags and ply: {e}")))?;

    let fen = &tags
        .remove("FEN")
        .unwrap_or_else(|| DEFAULT_FEN.to_string());

    let (_, fen) = fen::parse(fen)
        .map_err(|e| PgnParseError::new(format!("Failed to parse FEN string: {e}")))?;

    let result = &tags
        .remove("Result")
        .ok_or_else(|| PgnParseError::new("Missing 'Result' tag".to_string()))?;

    let (_, result) = all_consuming(result::parse)(result)
        .map_err(|e| PgnParseError::new(format!("Failed to parse result: {e}")))?;

    Ok(Pgn::new(tags, fen, result, ply))
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use crate::model::{
        AvailableCastle, BoardBuilder, Fen, GameResult, Movement, Piece, PieceColour, PieceType,
        Ply, PlyMovement, Position, Tags,
    };

    use super::*;

    #[test]
    fn returns_err_if_not_valid_pgn() {
        let result = parse("something");
        assert!(result.is_err())
    }

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
            Ply::new(
                1,
                PlyMovement::Move {
                    movement: Movement::new(
                        Piece::new(PieceColour::White, PieceType::Pawn),
                        Position::try_from(3, 4).unwrap(),
                    ),
                    qualifier: None,
                    check: None,
                },
                None,
            ),
            Ply::new(
                1,
                PlyMovement::Move {
                    movement: Movement::new(
                        Piece::new(PieceColour::Black, PieceType::Pawn),
                        Position::try_from(4, 4).unwrap(),
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
                Position::try_from(7, 0).unwrap(),
            );

        let board = board_builder.build();

        Fen::new(board, PieceColour::White, 1)
    }
}
