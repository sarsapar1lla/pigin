use nom::error::ParseError;
use nom::error::{Error, ErrorKind};
use nom::multi::many0;
use nom::sequence::{pair, terminated};
use nom::IResult;
use nom::{character::complete::line_ending, combinator::all_consuming};

use super::fen;
use super::movement;
use super::result;
use super::tag;
use crate::model::Pgn;

static DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// TODO: consider how to improve error handling here
pub fn parse(input: &str) -> IResult<&str, Pgn> {
    let (remaining, (mut tags, ply)) =
        terminated(pair(tag::parse, movement::parse), many0(line_ending))(input)?;

    let fen = &tags
        .remove("FEN")
        .unwrap_or_else(|| DEFAULT_FEN.to_string());

    let (_, fen) = fen::parse(fen)
        .map_err(|_| nom::Err::Error(Error::from_error_kind(input, ErrorKind::Tag)))?;

    let result = &tags
        .remove("Result")
        .ok_or_else(|| nom::Err::Error(Error::from_error_kind(input, ErrorKind::Tag)))?;

    let (_, result) = all_consuming(result::parse)(result)
        .map_err(|_| nom::Err::Error(Error::from_error_kind(input, ErrorKind::Tag)))?;

    Ok((remaining, Pgn::new(tags, fen, result, ply)))
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

        let remaining = r#"[White "Player, Three"]"#;

        assert_eq!(pgn, (remaining, expected()));
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
                        Position::new(3, 4),
                    ),
                    qualifier: None,
                    check: None,
                    capture: false,
                },
                None,
            ),
            Ply::new(
                1,
                PlyMovement::Move {
                    movement: Movement::new(
                        Piece::new(PieceColour::Black, PieceType::Pawn),
                        Position::new(4, 4),
                    ),
                    qualifier: None,
                    check: None,
                    capture: false,
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
                Position::new(7, 0),
            );

        let board = board_builder.build();

        Fen::new(board, PieceColour::White, 1)
    }
}
