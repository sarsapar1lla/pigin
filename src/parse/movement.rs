use crate::model::{PieceColour, Ply};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::{char, line_ending, space1};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::delimited;
use nom::{
    character::complete::{digit1, space0},
    combinator::map_res,
    sequence::{terminated, tuple},
    IResult,
};

use super::{ply, result};

pub fn parse(input: &str) -> IResult<&str, Vec<Ply>> {
    let result_only_parser = map(result::parse, |_| Vec::new());
    alt((result_only_parser, parse_moves))(input)
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Ply>> {
    let (remaining, mut first_move) =
        alt((parse_move, map(parse_partial_move, |ply| vec![ply])))(input)?;

    let (remaining, mut other_moves) = map(many0(parse_move), |list| {
        list.into_iter().flatten().collect()
    })(remaining)?;
    first_move.append(&mut other_moves);
    Ok((remaining, first_move))
}

fn parse_move(input: &str) -> IResult<&str, Vec<Ply>> {
    let (remaining, move_number) = white_move_number(input)?;
    let (remaining, white_ply) = ply::parse(remaining, PieceColour::White)?;
    let (remaining, white_comment) = opt(comment)(remaining)?;

    let (remaining, maybe_result) = opt(result::parse)(remaining)?;

    if maybe_result.is_some() {
        return Ok((
            remaining,
            vec![Ply::new(move_number, white_ply, white_comment)],
        ));
    }

    let (remaining, maybe_black_move_number) = opt(black_move_number)(remaining)?;

    let (remaining, black_ply) = ply::parse(remaining, PieceColour::Black)?;
    let (remaining, black_comment) = opt(comment)(remaining)?;

    let (remaining, _) = opt(result::parse)(remaining)?;

    Ok((
        remaining,
        vec![
            Ply::new(move_number, white_ply, white_comment),
            Ply::new(
                maybe_black_move_number.unwrap_or(move_number),
                black_ply,
                black_comment,
            ),
        ],
    ))
}

fn parse_partial_move(input: &str) -> IResult<&str, Ply> {
    let (remaining, move_number) = black_move_number(input)?;

    let (remaining, ply) = ply::parse(remaining, PieceColour::Black)?;
    let (remaining, comment) = opt(comment)(remaining)?;

    let (remaining, _) = opt(result::parse)(remaining)?;

    Ok((remaining, Ply::new(move_number, ply, comment)))
}

fn white_move_number(input: &str) -> IResult<&str, i16> {
    let terminator = tuple((char('.'), opt(line_ending), space0));
    map_res(terminated(digit1, terminator), |s: &str| s.parse::<i16>())(input)
}

fn black_move_number(input: &str) -> IResult<&str, i16> {
    let terminator = tuple((tag("..."), opt(line_ending), space0));
    map_res(terminated(digit1, terminator), |s: &str| s.parse::<i16>())(input)
}

fn comment(input: &str) -> IResult<&str, String> {
    alt((parenthesis_comment, semicolon_comment))(input)
}

fn parenthesis_comment(input: &str) -> IResult<&str, String> {
    let parser = terminated(
        delimited(char('{'), take_until("}"), char('}')),
        alt((space1, line_ending)),
    );
    map(parser, |s: &str| s.replace('\n', " "))(input)
}

fn semicolon_comment(input: &str) -> IResult<&str, String> {
    let parser = delimited(char(';'), take_until("\n"), line_ending);
    map(parser, |s: &str| s.trim().to_string())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_tests {
        use crate::model::{Movement, Piece, PieceType, PlyMovement, Position};

        use super::*;

        #[test]
        fn returns_err_if_not_moves() {
            let result = parse("something");
            assert!(result.is_err())
        }

        #[test]
        fn parses_result_only() {
            let result = parse("1-0 something").unwrap();
            assert_eq!(result, (" something", vec![]))
        }

        #[test]
        fn parses_moves() {
            let result = parse("1.e4 e5 2.Nc3 Nf6 something").unwrap();
            let expected = vec![
                Ply::new(
                    1,
                    PlyMovement::Move {
                        movement: Movement::new(
                            Piece::new(PieceColour::White, PieceType::Pawn),
                            Position::new(3, 4),
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
                            Position::new(4, 4),
                        ),
                        qualifier: None,
                        check: None,
                    },
                    None,
                ),
                Ply::new(
                    2,
                    PlyMovement::Move {
                        movement: Movement::new(
                            Piece::new(PieceColour::White, PieceType::Knight),
                            Position::new(2, 2),
                        ),
                        qualifier: None,
                        check: None,
                    },
                    None,
                ),
                Ply::new(
                    2,
                    PlyMovement::Move {
                        movement: Movement::new(
                            Piece::new(PieceColour::Black, PieceType::Knight),
                            Position::new(5, 5),
                        ),
                        qualifier: None,
                        check: None,
                    },
                    None,
                ),
            ];
            assert_eq!(result, ("something", expected))
        }

        #[test]
        fn parses_partial_moves() {
            let result = parse("1...e5 2.Nc3 Nf6 something").unwrap();
            let expected = vec![
                Ply::new(
                    1,
                    PlyMovement::Move {
                        movement: Movement::new(
                            Piece::new(PieceColour::Black, PieceType::Pawn),
                            Position::new(4, 4),
                        ),
                        qualifier: None,
                        check: None,
                    },
                    None,
                ),
                Ply::new(
                    2,
                    PlyMovement::Move {
                        movement: Movement::new(
                            Piece::new(PieceColour::White, PieceType::Knight),
                            Position::new(2, 2),
                        ),
                        qualifier: None,
                        check: None,
                    },
                    None,
                ),
                Ply::new(
                    2,
                    PlyMovement::Move {
                        movement: Movement::new(
                            Piece::new(PieceColour::Black, PieceType::Knight),
                            Position::new(5, 5),
                        ),
                        qualifier: None,
                        check: None,
                    },
                    None,
                ),
            ];
            assert_eq!(result, ("something", expected))
        }
    }

    mod parse_move_tests {
        use crate::model::{
            Check, MoveQualifier, Movement, Piece, PieceColour, PieceType, PlyMovement, Position,
        };

        use super::*;

        #[test]
        fn returns_error_if_not_move() {
            let result = parse_move("junk string");
            assert!(result.is_err())
        }

        #[test]
        fn parses_move() {
            let result = parse_move("1. e4 e5 2. d4 exd4+").unwrap();
            let expected_ply = vec![
                Ply::new(
                    1,
                    PlyMovement::Move {
                        movement: Movement::new(
                            Piece::new(PieceColour::White, PieceType::Pawn),
                            Position::new(3, 4),
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
                            Position::new(4, 4),
                        ),
                        qualifier: None,
                        check: None,
                    },
                    None,
                ),
            ];
            assert_eq!(result, ("2. d4 exd4+", expected_ply))
        }

        #[test]
        fn parses_move_with_black_move_numbers() {
            let result = parse_move("1. e4 1... e5 2. d4 2... exd4+").unwrap();
            let expected_ply = vec![
                Ply::new(
                    1,
                    PlyMovement::Move {
                        movement: Movement::new(
                            Piece::new(PieceColour::White, PieceType::Pawn),
                            Position::new(3, 4),
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
                            Position::new(4, 4),
                        ),
                        qualifier: None,
                        check: None,
                    },
                    None,
                ),
            ];
            assert_eq!(result, ("2. d4 2... exd4+", expected_ply))
        }

        #[test]
        fn parses_move_with_comments() {
            let result =
                parse_move("2. Bcd3 {A comment} O-O+ ; Another comment\n3. f7 Qb2").unwrap();
            let expected_ply = vec![
                Ply::new(
                    2,
                    PlyMovement::Move {
                        movement: Movement::new(
                            Piece::new(PieceColour::White, PieceType::Bishop),
                            Position::new(2, 3),
                        ),
                        qualifier: Some(MoveQualifier::Col(2)),
                        check: None,
                    },
                    Some("A comment".to_string()),
                ),
                Ply::new(
                    2,
                    PlyMovement::KingsideCastle {
                        colour: PieceColour::Black,
                        check: Some(Check::Check),
                    },
                    Some("Another comment".to_string()),
                ),
            ];
            assert_eq!(result, ("3. f7 Qb2", expected_ply))
        }

        #[test]
        fn parses_move_with_result_after_white_move() {
            let result = parse_move("2. e4 1-0 something").unwrap();
            let expected_ply = vec![Ply::new(
                2,
                PlyMovement::Move {
                    movement: Movement::new(
                        Piece::new(PieceColour::White, PieceType::Pawn),
                        Position::new(3, 4),
                    ),
                    qualifier: None,
                    check: None,
                },
                None,
            )];
            assert_eq!(result, (" something", expected_ply))
        }

        #[test]
        fn parses_move_with_result_after_black_move() {
            let result = parse_move("2. e4 d5 1-0 something").unwrap();
            let expected_ply = vec![
                Ply::new(
                    2,
                    PlyMovement::Move {
                        movement: Movement::new(
                            Piece::new(PieceColour::White, PieceType::Pawn),
                            Position::new(3, 4),
                        ),
                        qualifier: None,
                        check: None,
                    },
                    None,
                ),
                Ply::new(
                    2,
                    PlyMovement::Move {
                        movement: Movement::new(
                            Piece::new(PieceColour::Black, PieceType::Pawn),
                            Position::new(4, 3),
                        ),
                        qualifier: None,
                        check: None,
                    },
                    None,
                ),
            ];
            assert_eq!(result, (" something", expected_ply))
        }
    }

    mod parse_partial_move_tests {
        use crate::model::{Movement, Piece, PieceType, PlyMovement, Position};

        use super::*;

        #[test]
        fn returns_err_if_not_move() {
            let result = parse_partial_move("something");
            assert!(result.is_err())
        }

        #[test]
        fn parses_partial_move() {
            let result = parse_partial_move("2... e5 3. d4").unwrap();
            assert_eq!(
                result,
                (
                    "3. d4",
                    Ply::new(
                        2,
                        PlyMovement::Move {
                            movement: Movement::new(
                                Piece::new(PieceColour::Black, PieceType::Pawn),
                                Position::new(4, 4)
                            ),
                            qualifier: None,
                            check: None
                        },
                        None
                    )
                )
            )
        }

        #[test]
        fn parses_partial_move_with_comment() {
            let result = parse_partial_move("2... e5 {A comment} 3. d4").unwrap();
            assert_eq!(
                result,
                (
                    "3. d4",
                    Ply::new(
                        2,
                        PlyMovement::Move {
                            movement: Movement::new(
                                Piece::new(PieceColour::Black, PieceType::Pawn),
                                Position::new(4, 4)
                            ),
                            qualifier: None,
                            check: None
                        },
                        Some("A comment".to_string())
                    )
                )
            )
        }

        #[test]
        fn parses_partial_move_with_result() {
            let result = parse_partial_move("2... e5 1-0").unwrap();
            assert_eq!(
                result,
                (
                    "",
                    Ply::new(
                        2,
                        PlyMovement::Move {
                            movement: Movement::new(
                                Piece::new(PieceColour::Black, PieceType::Pawn),
                                Position::new(4, 4)
                            ),
                            qualifier: None,
                            check: None
                        },
                        None
                    )
                )
            )
        }
    }

    mod white_move_number_tests {
        use super::*;

        #[test]
        fn parses_move_number_with_space() {
            let result = white_move_number("1. e4").unwrap();
            assert_eq!(result, ("e4", 1))
        }

        #[test]
        fn parses_move_number_without_space() {
            let result = white_move_number("1.e4").unwrap();
            assert_eq!(result, ("e4", 1))
        }

        #[test]
        fn parses_move_number_with_newline() {
            let result = white_move_number("1.\ne4").unwrap();
            assert_eq!(result, ("e4", 1))
        }

        #[test]
        fn parses_move_number_with_line_ending() {
            let result = white_move_number("1.\r\ne4").unwrap();
            assert_eq!(result, ("e4", 1))
        }
    }

    mod black_move_number_tests {
        use super::*;

        #[test]
        fn parses_move_number_with_space() {
            let result = black_move_number("1... e4").unwrap();
            assert_eq!(result, ("e4", 1))
        }

        #[test]
        fn parses_move_number_without_space() {
            let result = black_move_number("1...e4").unwrap();
            assert_eq!(result, ("e4", 1))
        }

        #[test]
        fn parses_move_number_with_newline() {
            let result = black_move_number("1...\ne4").unwrap();
            assert_eq!(result, ("e4", 1))
        }

        #[test]
        fn parses_move_number_with_line_ending() {
            let result = black_move_number("1...\r\ne4").unwrap();
            assert_eq!(result, ("e4", 1))
        }
    }

    mod comment_tests {
        use super::*;

        #[test]
        fn parses_comment_in_parenthesis() {
            let result = comment("{Comment} d5").unwrap();
            assert_eq!(result, ("d5", "Comment".to_string()))
        }

        #[test]
        fn parses_comment_in_parenthesis_at_line_end() {
            let result = comment("{Comment}\nd5").unwrap();
            assert_eq!(result, ("d5", "Comment".to_string()))
        }

        #[test]
        fn parses_multiline_comment_in_parenthesis() {
            let result = comment("{Comment\ncontinued} d5").unwrap();
            assert_eq!(result, ("d5", "Comment continued".to_string()))
        }

        #[test]
        fn parses_semicolon_comment() {
            let result = comment("; This comment runs to the end of the line\nd5").unwrap();
            assert_eq!(
                result,
                ("d5", "This comment runs to the end of the line".to_string())
            )
        }
    }
}
