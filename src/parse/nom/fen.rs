use crate::model::AvailableCastle;
use crate::model::{
    Board, Fen, Piece, PieceColour, PieceType, Position, MAX_POSITION, MIN_POSITION,
};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{i8, u8};
use nom::combinator::all_consuming;
use nom::{
    character::complete::one_of,
    combinator::{map, map_res},
    multi::fold_many0,
    sequence::{terminated, tuple},
    IResult,
};

use super::error::PgnParseError;
use super::position::position;
use crate::model::BoardBuilder;

#[derive(Debug, PartialEq, Eq)]
enum FenCharacter {
    Empty(i8),
    NewRow,
    Piece(Piece),
}

// TODO: add tests
pub fn parse_fen(input: &str) -> IResult<&str, Fen> {
    let parser = all_consuming(tuple((
        fen_characters,
        active_colour,
        available_castles,
        en_passant_square,
        clock,
        clock,
    )));
    map_res(parser, |elements| {
        let starting_board = board_from(elements.0, elements.2, elements.3)?;
        Ok::<Fen, PgnParseError>(Fen::new(starting_board, elements.1, elements.5))
    })(input)
}

// TODO: add tests
fn board_from(
    fen_characters: Vec<FenCharacter>,
    available_castles: Vec<AvailableCastle>,
    en_passant_square: Option<Position>,
) -> Result<Board, PgnParseError> {
    let mut builder = BoardBuilder::new();

    builder.available_castles(available_castles);

    if let Some(position) = en_passant_square {
        builder.en_passant_square(position);
    }

    let mut row = MAX_POSITION;
    let mut col = MIN_POSITION;

    for character in fen_characters {
        match character {
            FenCharacter::NewRow => {
                row -= 1;
                col = MIN_POSITION
            }
            FenCharacter::Empty(spaces) => col += spaces,
            FenCharacter::Piece(piece) => {
                let position = Position::new(row, col).map_err(|e| {
                    PgnParseError::new(format!("Failed to create position for fen character: {e}"))
                })?;
                builder.piece(piece, position);
            }
        }
    }

    Ok(builder.build())
}

fn fen_characters(input: &str) -> IResult<&str, Vec<FenCharacter>> {
    let parser = alt((new_row, empty_spaces, piece));

    terminated(
        fold_many0(
            parser,
            Vec::new,
            |mut acc: Vec<FenCharacter>, item: FenCharacter| {
                acc.push(item);
                acc
            },
        ),
        tag(" "),
    )(input)
}

fn new_row(input: &str) -> IResult<&str, FenCharacter> {
    map(tag("/"), |_| FenCharacter::NewRow)(input)
}

fn empty_spaces(input: &str) -> IResult<&str, FenCharacter> {
    map_res(i8, |i| match i {
        i if (i >= 1) & (i <= 8) => Ok(FenCharacter::Empty(i)),
        _ => Err(PgnParseError::new(format!(
            "'{i}' is not a valid empty space"
        ))),
    })(input)
}

fn piece(input: &str) -> IResult<&str, FenCharacter> {
    map_res(one_of("pnbrqkPNBRQK"), |c: char| {
        match c {
            'p' => Ok((PieceColour::Black, PieceType::Pawn)),
            'n' => Ok((PieceColour::Black, PieceType::Knight)),
            'b' => Ok((PieceColour::Black, PieceType::Bishop)),
            'r' => Ok((PieceColour::Black, PieceType::Rook)),
            'q' => Ok((PieceColour::Black, PieceType::Queen)),
            'k' => Ok((PieceColour::Black, PieceType::King)),
            'P' => Ok((PieceColour::White, PieceType::Pawn)),
            'N' => Ok((PieceColour::White, PieceType::Knight)),
            'B' => Ok((PieceColour::White, PieceType::Bishop)),
            'R' => Ok((PieceColour::White, PieceType::Rook)),
            'Q' => Ok((PieceColour::White, PieceType::Queen)),
            'K' => Ok((PieceColour::White, PieceType::King)),
            _ => Err(PgnParseError::new(format!("'{c}' is not a valid piece"))),
        }
        .map(|capture: (PieceColour, PieceType)| {
            FenCharacter::Piece(Piece::new(capture.0, capture.1))
        })
    })(input)
}

fn active_colour(input: &str) -> IResult<&str, PieceColour> {
    map_res(terminated(one_of("wb"), tag(" ")), |c: char| match c {
        'w' => Ok(PieceColour::White),
        'b' => Ok(PieceColour::Black),
        _ => Err(PgnParseError::new(format!(
            "'{c}' is not a valid active colour"
        ))),
    })(input)
}

fn available_castles(input: &str) -> IResult<&str, Vec<AvailableCastle>> {
    let none_parser = map(tag("-"), |_| Vec::new());
    let some_parser = fold_many0(
        available_castle,
        Vec::new,
        |mut acc: Vec<AvailableCastle>, item: AvailableCastle| {
            acc.push(item);
            acc
        },
    );
    terminated(alt((none_parser, some_parser)), tag(" "))(input)
}

fn available_castle(input: &str) -> IResult<&str, AvailableCastle> {
    map_res(one_of("KQkq"), |c: char| match c {
        'K' => Ok(AvailableCastle::WhiteKingside),
        'Q' => Ok(AvailableCastle::WhiteQueenside),
        'k' => Ok(AvailableCastle::BlackKingside),
        'q' => Ok(AvailableCastle::BlackQueenside),
        _ => Err(PgnParseError::new(format!(
            "'{c}' is not a valid available castle"
        ))),
    })(input)
}

fn en_passant_square(input: &str) -> IResult<&str, Option<Position>> {
    let none_parser = map(tag("-"), |_| None);
    let some_parser = map(position, |p| Some(p));

    terminated(alt((none_parser, some_parser)), tag(" "))(input)
}

fn clock(input: &str) -> IResult<&str, u8> {
    u8(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fen_characters_tests {
        use super::*;

        #[test]
        fn returns_error_if_not_fen_characters() {
            let result = fen_characters("something else");
            assert!(result.is_err())
        }

        #[test]
        fn parses_fen_characters() {
            let result = fen_characters("p7/B6n something").unwrap();
            let expected = vec![
                FenCharacter::Piece(Piece::new(PieceColour::Black, PieceType::Pawn)),
                FenCharacter::Empty(7),
                FenCharacter::NewRow,
                FenCharacter::Piece(Piece::new(PieceColour::White, PieceType::Bishop)),
                FenCharacter::Empty(6),
                FenCharacter::Piece(Piece::new(PieceColour::Black, PieceType::Knight)),
            ];
            assert_eq!(result, ("something", expected))
        }
    }

    mod new_row_tests {
        use super::*;

        #[test]
        fn returns_error_if_not_new_row() {
            let result = new_row("p/");
            assert!(result.is_err())
        }

        #[test]
        fn parses_new_row() {
            let result = new_row("/8").unwrap();
            assert_eq!(result, ("8", FenCharacter::NewRow))
        }
    }

    mod empty_spaces_tests {
        use super::*;

        #[test]
        fn returns_error_if_not_empty_space() {
            let result = empty_spaces("0");
            assert!(result.is_err())
        }

        #[test]
        fn parses_empty_spaces() {
            let result = empty_spaces("5/").unwrap();
            assert_eq!(result, ("/", FenCharacter::Empty(5)))
        }
    }

    mod piece_tests {
        use super::*;

        #[test]
        fn returns_error_if_not_piece() {
            let result = piece("jj");
            assert!(result.is_err())
        }

        #[test]
        fn parses_piece() {
            let result = piece("nj").unwrap();
            assert_eq!(
                result,
                (
                    "j",
                    FenCharacter::Piece(Piece::new(PieceColour::Black, PieceType::Knight,))
                )
            )
        }
    }

    mod active_colour_tests {
        use super::*;

        #[test]
        fn returns_error_if_not_active_colour() {
            let result = active_colour("c");
            assert!(result.is_err())
        }

        #[test]
        fn parses_active_colour() {
            let result = active_colour("b something").unwrap();
            assert_eq!(result, ("something", PieceColour::Black))
        }
    }

    mod available_castles_test {
        use super::*;

        #[test]
        fn returns_error_if_not_available_castles() {
            let result = available_castles("something ");
            assert!(result.is_err())
        }

        #[test]
        fn returns_empty_vec_if_no_available_castles() {
            let result = available_castles("- something").unwrap();
            assert_eq!(result, ("something", Vec::new()))
        }

        #[test]
        fn parses_available_castles() {
            let result = available_castles("KQq something").unwrap();
            assert_eq!(
                result,
                (
                    "something",
                    vec![
                        AvailableCastle::WhiteKingside,
                        AvailableCastle::WhiteQueenside,
                        AvailableCastle::BlackQueenside
                    ]
                )
            )
        }
    }

    mod en_passant_square_tests {
        use super::*;

        #[test]
        fn returns_error_if_not_en_passent_square() {
            let result = en_passant_square("something ");
            assert!(result.is_err())
        }

        #[test]
        fn returns_none_if_no_en_passent_square() {
            let result = en_passant_square("- something").unwrap();
            assert_eq!(result, ("something", None))
        }

        #[test]
        fn parses_some_en_passent_square() {
            let result = en_passant_square("e4 something").unwrap();
            assert_eq!(result, ("something", Some(Position::new(3, 4).unwrap())))
        }
    }
}
