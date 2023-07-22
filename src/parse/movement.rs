use crate::model::{PieceColour, PlyMetadata};
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::{char, line_ending, space1};
use nom::combinator::{map, opt};
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::{
    character::complete::{digit1, space0},
    combinator::map_res,
    sequence::{terminated, tuple},
    IResult,
};

use super::ply::ply;
use super::result::parse_result;

pub fn parse_moves(input: &str) -> IResult<&str, Vec<PlyMetadata>> {
    fold_many0(
        parse_move,
        Vec::new,
        |mut acc: Vec<PlyMetadata>, mut item: Vec<PlyMetadata>| {
            acc.append(&mut item);
            acc
        },
    )(input)
}

fn parse_move(input: &str) -> IResult<&str, Vec<PlyMetadata>> {
    let (remaining, move_number) = move_number(input)?;
    let (remaining, white_ply) = ply(remaining, PieceColour::White)?;
    let (remaining, white_comment) = opt(comment)(remaining)?;

    let (remaining, maybe_result) = opt(parse_result)(remaining)?;

    if maybe_result.is_some() {
        return Ok((
            remaining,
            vec![PlyMetadata::new(move_number, white_ply, white_comment)],
        ));
    }

    let (remaining, black_ply) = ply(remaining, PieceColour::Black)?;
    let (remaining, black_comment) = opt(comment)(remaining)?;

    let (remaining, _) = opt(parse_result)(remaining)?;

    Ok((
        remaining,
        vec![
            PlyMetadata::new(move_number, white_ply, white_comment),
            PlyMetadata::new(move_number, black_ply, black_comment),
        ],
    ))
}

fn move_number(input: &str) -> IResult<&str, i8> {
    map_res(terminated(digit1, tuple((char('.'), space0))), |s: &str| {
        s.parse::<i8>()
    })(input)
}

fn comment(input: &str) -> IResult<&str, String> {
    alt((parenthesis_comment, semicolon_comment))(input)
}

fn parenthesis_comment(input: &str) -> IResult<&str, String> {
    let parser = terminated(delimited(char('{'), take_until("}"), char('}')), space1);
    map(parser, |s: &str| s.replace('\n', " "))(input)
}

fn semicolon_comment(input: &str) -> IResult<&str, String> {
    let parser = delimited(char(';'), take_until("\n"), line_ending);
    map(parser, |s: &str| s.trim().to_string())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_move_tests {
        use crate::model::{Check, MoveQualifier, Movement, PieceColour, PieceType, Ply, Position};

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
            assert_eq!(result, ("2. d4 exd4+", expected_ply))
        }

        #[test]
        fn parses_move_with_comments() {
            let result =
                parse_move("2. Bcd3 {A comment} O-O+ ; Another comment\n3. f7 Qb2").unwrap();
            let expected_ply = vec![
                PlyMetadata::new(
                    2,
                    Ply::Move {
                        movement: Movement::new(
                            PieceType::Bishop,
                            PieceColour::White,
                            Position::new(2, 3).unwrap(),
                        ),
                        qualifier: Some(MoveQualifier::Col(2)),
                        check: None,
                    },
                    Some("A comment".to_string()),
                ),
                PlyMetadata::new(
                    2,
                    Ply::KingsideCastle {
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
            let expected_ply = vec![PlyMetadata::new(
                2,
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
            )];
            assert_eq!(result, (" something", expected_ply))
        }

        #[test]
        fn parses_move_with_result_after_black_move() {
            let result = parse_move("2. e4 d5 1-0 something").unwrap();
            let expected_ply = vec![
                PlyMetadata::new(
                    2,
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
                    2,
                    Ply::Move {
                        movement: Movement::new(
                            PieceType::Pawn,
                            PieceColour::Black,
                            Position::new(4, 3).unwrap(),
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

    mod move_number_tests {
        use super::*;

        #[test]
        fn parses_move_number_with_space() {
            let result = move_number("1. e4").unwrap();
            assert_eq!(result, ("e4", 1))
        }

        #[test]
        fn parses_move_number_without_space() {
            let result = move_number("1.e4").unwrap();
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
