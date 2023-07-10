use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, one_of, space1},
    combinator::{map, map_res, opt},
    sequence::{pair, separated_pair, terminated, tuple},
    IResult,
};

use crate::model::{MoveQualifier, Movement, PieceColour};
use crate::model::{PieceType, Ply, Position};

use super::error::PgnParseError;

use super::position::{position, row, column};

// use super::error::ParserError;

pub fn ply(input: &str, colour: PieceColour) -> IResult<&str, Ply> {
    piece_move(input, colour)
        .or_else(|_| kingside_castle(input, colour))
        .or_else(|_| queenside_castle(input, colour))
}

fn piece_move(input: &str, colour: PieceColour) -> IResult<&str, Ply> {
    let (remainder, (maybe_piece_type, (maybe_move_qualifier, position), maybe_promotion)) =
        terminated(
            tuple((opt(piece_type), position_with_qualifier, opt(promotion))),
            ply_terminator,
        )(input)?;

    let movement = Movement::new(
        maybe_piece_type.unwrap_or(PieceType::Pawn),
        colour,
        position,
    );

    match maybe_promotion {
        None => Ok((
            remainder,
            Ply::Move {
                movement,
                qualifier: maybe_move_qualifier,
            },
        )),
        Some(promotion) => Ok((
            remainder,
            Ply::Promotion {
                movement,
                promotes_to: promotion,
                qualifier: maybe_move_qualifier,
            },
        )),
    }
}

fn position_with_qualifier(input: &str) -> IResult<&str, (Option<MoveQualifier>, Position)> {
    alt((
        map(
            separated_pair(move_qualifier, opt(tag("x")), position),
            |values: (MoveQualifier, Position)| (Some(values.0), values.1),
        ),
        map(position, |p: Position| (None as Option<MoveQualifier>, p)),
    ))(input)
}

fn promotion(input: &str) -> IResult<&str, PieceType> {
    let parser = pair(tag("="), piece_type);
    map(parser, |matches| matches.1)(input)
}

fn move_qualifier(input: &str) -> IResult<&str, MoveQualifier> {
    let parser = pair(opt(column), opt(row));
    map_res(parser, |values: (Option<i8>, Option<i8>)| {
        match (values.0, values.1) {
            (None, None) => Err(PgnParseError::new(format!(
                "'{input}' is not a valid move qualifier"
            ))),
            (Some(col), None) => Ok(MoveQualifier::Col(col)),
            (None, Some(row)) => Ok(MoveQualifier::Row(row)),
            (Some(col), Some(row)) => Ok(MoveQualifier::Position(
                Position::new(row, col).map_err(|e| PgnParseError::new(e.to_string()))?,
            )),
        }
    })(input)
}

fn kingside_castle(input: &str, colour: PieceColour) -> IResult<&str, Ply> {
    let castle_parser = alt((tag("O-O"), tag("0-0")));
    let parser = terminated(castle_parser, ply_terminator);
    map(parser, |_| Ply::KingsideCastle(colour))(input)
}

fn queenside_castle(input: &str, colour: PieceColour) -> IResult<&str, Ply> {
    let castle_parser = alt((tag("O-O-O"), tag("0-0-0")));
    let parser = terminated(castle_parser, ply_terminator);
    map(parser, |_| Ply::QueensideCastle(colour))(input)
}

fn piece_type(input: &str) -> IResult<&str, PieceType> {
    map_res(one_of("NBRQK"), |c: char| match c {
        'N' => Ok(PieceType::Knight),
        'B' => Ok(PieceType::Bishop),
        'R' => Ok(PieceType::Rook),
        'Q' => Ok(PieceType::Queen),
        'K' => Ok(PieceType::King),
        _ => Err(format!("Invalid piece type '{c}'")),
    })(input)
}

fn ply_terminator(input: &str) -> IResult<&str, &str> {
    // TODO: add checks and checkmate here
    alt((space1, line_ending))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod ply_terminator_tests {
        use super::*;

        #[test]
        fn parses_space() {
            let result = ply_terminator(" e5").unwrap();
            assert_eq!(result, ("e5", " "))
        }

        #[test]
        fn parses_newline() {
            let result = ply_terminator("\ne5").unwrap();
            assert_eq!(result, ("e5", "\n"))
        }
    }

    mod kingside_castle_tests {
        use super::*;

        #[test]
        fn returns_err_if_not_kingside_castle() {
            let result = queenside_castle("e4 e5", PieceColour::White);
            assert!(result.is_err())
        }

        #[test]
        fn parses_kingside_castle() {
            let result = kingside_castle("O-O f6", PieceColour::White).unwrap();
            assert_eq!(result, ("f6", Ply::KingsideCastle(PieceColour::White)))
        }

        #[test]
        fn parses_kingside_castle_at_line_end() {
            let result = kingside_castle("O-O\nf6", PieceColour::White).unwrap();
            assert_eq!(result, ("f6", Ply::KingsideCastle(PieceColour::White)))
        }

        #[test]
        fn parses_kingside_castle_with_zeros() {
            let result = kingside_castle("0-0 f6", PieceColour::White).unwrap();
            assert_eq!(result, ("f6", Ply::KingsideCastle(PieceColour::White)))
        }

        #[test]
        fn parses_kingside_castle_with_zeros_at_line_end() {
            let result = kingside_castle("0-0\nf6", PieceColour::White).unwrap();
            assert_eq!(result, ("f6", Ply::KingsideCastle(PieceColour::White)))
        }
    }

    mod queenside_castle_tests {
        use super::*;

        #[test]
        fn returns_err_if_not_queenside_castle() {
            let result = queenside_castle("e4 e5", PieceColour::White);
            assert!(result.is_err())
        }

        #[test]
        fn parses_queenside_castle() {
            let result = queenside_castle("O-O-O f6", PieceColour::White).unwrap();
            assert_eq!(result, ("f6", Ply::QueensideCastle(PieceColour::White)))
        }

        #[test]
        fn parses_queenside_castle_at_line_end() {
            let result = queenside_castle("O-O-O\nf6", PieceColour::White).unwrap();
            assert_eq!(result, ("f6", Ply::QueensideCastle(PieceColour::White)))
        }

        #[test]
        fn parses_queenside_castle_with_zeros() {
            let result = queenside_castle("0-0-0 f6", PieceColour::White).unwrap();
            assert_eq!(result, ("f6", Ply::QueensideCastle(PieceColour::White)))
        }

        #[test]
        fn parses_queenside_castle_with_zeros_at_line_end() {
            let result = queenside_castle("0-0-0\nf6", PieceColour::White).unwrap();
            assert_eq!(result, ("f6", Ply::QueensideCastle(PieceColour::White)))
        }
    }

    mod piece_move_tests {
        use super::*;

        #[test]
        fn returns_err_if_not_piece_move() {
            let result = piece_move("junk string", PieceColour::White);
            assert!(result.is_err())
        }

        #[test]
        fn parses_pawn_move() {
            let result = piece_move("a6 Bd3", PieceColour::White).unwrap();
            assert_eq!(
                result,
                (
                    "Bd3",
                    Ply::Move {
                        movement: Movement::new(
                            PieceType::Pawn,
                            PieceColour::White,
                            Position::new(5, 0).unwrap()
                        ),
                        qualifier: None
                    }
                )
            )
        }

        #[test]
        fn parses_pawn_capture() {
            let result = piece_move("axb6 Bd3", PieceColour::White).unwrap();
            assert_eq!(
                result,
                (
                    "Bd3",
                    Ply::Move {
                        movement: Movement::new(
                            PieceType::Pawn,
                            PieceColour::White,
                            Position::new(5, 1).unwrap()
                        ),
                        qualifier: Some(MoveQualifier::Col(0))
                    }
                )
            )
        }

        #[test]
        fn parses_pawn_capture_with_position_qualifier() {
            let result = piece_move("a5xb6 Bd3", PieceColour::White).unwrap();
            assert_eq!(
                result,
                (
                    "Bd3",
                    Ply::Move {
                        movement: Movement::new(
                            PieceType::Pawn,
                            PieceColour::White,
                            Position::new(5, 1).unwrap()
                        ),
                        qualifier: Some(MoveQualifier::Position(Position::new(4, 0).unwrap()))
                    }
                )
            )
        }

        #[test]
        fn parses_pawn_move_with_promotion() {
            let result = piece_move("a8=R Bd3", PieceColour::White).unwrap();
            assert_eq!(
                result,
                (
                    "Bd3",
                    Ply::Promotion {
                        movement: Movement::new(
                            PieceType::Pawn,
                            PieceColour::White,
                            Position::new(7, 0).unwrap()
                        ),
                        promotes_to: PieceType::Rook,
                        qualifier: None
                    }
                )
            )
        }

        #[test]
        fn parses_pawn_capture_with_promotion() {
            let result = piece_move("axb8=R Bd3", PieceColour::White).unwrap();
            assert_eq!(
                result,
                (
                    "Bd3",
                    Ply::Promotion {
                        movement: Movement::new(
                            PieceType::Pawn,
                            PieceColour::White,
                            Position::new(7, 1).unwrap()
                        ),
                        promotes_to: PieceType::Rook,
                        qualifier: Some(MoveQualifier::Col(0))
                    }
                )
            )
        }

        #[test]
        fn parses_piece_move() {
            let result = piece_move("Nd7 h2", PieceColour::White).unwrap();
            assert_eq!(
                result,
                (
                    "h2",
                    Ply::Move {
                        movement: Movement::new(
                            PieceType::Knight,
                            PieceColour::White,
                            Position::new(6, 3).unwrap()
                        ),
                        qualifier: None
                    }
                )
            )
        }

        #[test]
        fn parses_piece_move_with_column_qualifier() {
            let result = piece_move("Ncd7 h2", PieceColour::White).unwrap();
            assert_eq!(
                result,
                (
                    "h2",
                    Ply::Move {
                        movement: Movement::new(
                            PieceType::Knight,
                            PieceColour::White,
                            Position::new(6, 3).unwrap()
                        ),
                        qualifier: Some(MoveQualifier::Col(2))
                    }
                )
            )
        }

        #[test]
        fn parses_piece_move_with_row_qualifier() {
            let result = piece_move("N6d7 h2", PieceColour::White).unwrap();
            assert_eq!(
                result,
                (
                    "h2",
                    Ply::Move {
                        movement: Movement::new(
                            PieceType::Knight,
                            PieceColour::White,
                            Position::new(6, 3).unwrap()
                        ),
                        qualifier: Some(MoveQualifier::Row(5))
                    }
                )
            )
        }

        #[test]
        fn parses_piece_move_with_position_qualifier() {
            let result = piece_move("Nb6d7 h2", PieceColour::White).unwrap();
            assert_eq!(
                result,
                (
                    "h2",
                    Ply::Move {
                        movement: Movement::new(
                            PieceType::Knight,
                            PieceColour::White,
                            Position::new(6, 3).unwrap()
                        ),
                        qualifier: Some(MoveQualifier::Position(Position::new(5, 1).unwrap()))
                    }
                )
            )
        }
    }

    mod safe_position_tests {
        use super::*;

        #[test]
        fn returns_err_if_not_move() {
            let result = position_with_qualifier("junk string");
            assert!(result.is_err())
        }

        #[test]
        fn parses_position() {
            let result = position_with_qualifier("e4 e5").unwrap();
            assert_eq!(result, (" e5", (None, Position::new(3, 4).unwrap())))
        }

        #[test]
        fn parses_position_with_column_qualifier() {
            let result = position_with_qualifier("dxe4 e5").unwrap();
            assert_eq!(
                result,
                (
                    " e5",
                    (Some(MoveQualifier::Col(3)), Position::new(3, 4).unwrap())
                )
            )
        }

        #[test]
        fn parses_position_with_position_qualifier() {
            let result = position_with_qualifier("d3xe4 e5").unwrap();
            assert_eq!(
                result,
                (
                    " e5",
                    (
                        Some(MoveQualifier::Position(Position::new(2, 3).unwrap())),
                        Position::new(3, 4).unwrap()
                    )
                )
            )
        }
    }

    mod promotion_tests {
        use super::*;

        #[test]
        fn returns_err_if_not_a_promotion() {
            let result = promotion("e4");
            assert!(result.is_err())
        }

        #[test]
        fn parses_promotion() {
            let result = promotion("=Q e5").unwrap();
            assert_eq!(result, (" e5", PieceType::Queen))
        }
    }

    mod move_qualifier_tests {
        use super::*;

        #[test]
        fn returns_err_if_not_a_move_qualifier() {
            let result = move_qualifier("Nxe4");
            assert!(result.is_err())
        }

        #[test]
        fn parses_column_qualifier() {
            let result = move_qualifier("exd5").unwrap();
            assert_eq!(result, ("xd5", MoveQualifier::Col(4)))
        }

        #[test]
        fn parses_row_qualifier() {
            let result = move_qualifier("4xd5").unwrap();
            assert_eq!(result, ("xd5", MoveQualifier::Row(3)))
        }

        #[test]
        fn parses_position_qualifier() {
            let result = move_qualifier("e4xd5").unwrap();
            assert_eq!(
                result,
                ("xd5", MoveQualifier::Position(Position::new(3, 4).unwrap()))
            )
        }
    }
}
