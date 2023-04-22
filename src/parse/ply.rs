use regex::Regex;

use super::error::ParseError;
use super::piece::parse_piece_type;
use crate::model::PieceColour;
use crate::model::PieceType;
use crate::model::Position;
use crate::model::{MoveQualifier, Movement, Ply};

static COLS: &str = "abcdefgh";
static ROWS: &str = "12345678";
static PLY_PATTERN: &str = r"^([NBRQK])?([a-h])?([1-8])?x?([a-h][1-8])=?([NBRQK])?$";

pub fn parse_ply(s: &str, colour_to_move: PieceColour) -> Result<Ply, ParseError> {
    // Handle castling cases first
    if s == "O-O" {
        return Ok(Ply::KingsideCastle(colour_to_move));
    }

    if s == "O-O-O" {
        return Ok(Ply::QueensideCastle(colour_to_move));
    }

    let regex_match = match_with_regex(s)?;

    match regex_match {
        RegexMatch {
            move_to,
            piece_type,
            move_qualifier,
            promoted_piece: None,
        } => {
            let movement = Movement::new(piece_type, colour_to_move, move_to);
            Ok(Ply::Move {
                movement,
                qualifier: move_qualifier,
            })
        }
        RegexMatch {
            move_to,
            piece_type,
            move_qualifier,
            promoted_piece: Some(promoted_piece),
        } => {
            let movement = Movement::new(piece_type, colour_to_move, move_to);
            Ok(Ply::Promotion {
                movement,
                promotes_to: promoted_piece,
                qualifier: move_qualifier,
            })
        }
    }
}

fn parse_position(s: &str) -> Result<Position, ParseError> {
    if s.len() != 2 {
        return Err(ParseError(format!("String '{s}' should be length 2")));
    }

    let row = s
        .chars()
        .nth(1)
        .ok_or_else(|| ParseError(format!("Couldn't extract row from string '{s}'")))
        .and_then(get_row_from_char)?;
    let col = s
        .chars()
        .next()
        .ok_or_else(|| ParseError(format!("Couldn't extract column from string '{s}'")))
        .and_then(get_col_from_char)?;

    Position::new(row, col).map_err(|e| ParseError(e.to_string()))
}

fn get_col_from_char(c: char) -> Result<i8, ParseError> {
    COLS.find(c)
        .ok_or_else(|| ParseError(format!("'{c}' is not a valid column")))
        .and_then(|row| {
            i8::try_from(row)
                .map_err(|e| ParseError(format!("Failed to parse col '{c}' to i8: {e}")))
        })
}

fn get_row_from_char(c: char) -> Result<i8, ParseError> {
    ROWS.find(c)
        .ok_or_else(|| ParseError(format!("'{c}' is not a valid row")))
        .and_then(|row| {
            i8::try_from(row)
                .map_err(|e| ParseError(format!("Failed to parse row '{c}' to i8: {e}")))
        })
}

// Ply
struct RegexMatch {
    move_to: Position,
    piece_type: PieceType,
    move_qualifier: Option<MoveQualifier>,
    promoted_piece: Option<PieceType>,
}

fn match_with_regex(s: &str) -> Result<RegexMatch, ParseError> {
    let captures = Regex::new(PLY_PATTERN)
        .map_err(|_| ParseError("Could not compile regex pattern".to_string()))?
        .captures(s)
        .ok_or_else(|| ParseError(format!("'{s}' could not be parsed")))?;

    let move_to = captures
        .get(4)
        .ok_or_else(|| ParseError(format!("{s} did not contain move position")))
        .and_then(|capture| parse_position(capture.as_str()))?;

    let piece_type = match captures.get(1) {
        None => PieceType::Pawn,
        Some(piece_type) => {
            parse_piece_type(piece_type.as_str()).map_err(|e| ParseError(e.to_string()))?
        }
    };

    let column_qualifier = match captures.get(2) {
        None => None,
        Some(col) => Some(
            col.as_str()
                .chars()
                .next()
                .ok_or_else(|| ParseError(format!("Column capture {} is empty", col.as_str())))
                .and_then(get_col_from_char)?,
        ),
    };

    let row_qualifier = match captures.get(3) {
        None => None,
        Some(row) => Some(
            row.as_str()
                .chars()
                .next()
                .ok_or_else(|| ParseError(format!("Row capture {} is empty", row.as_str())))
                .and_then(get_row_from_char)?,
        ),
    };

    let move_qualifier = match (row_qualifier, column_qualifier) {
        (None, None) => None,
        (Some(row), None) => Some(MoveQualifier::Row(row)),
        (None, Some(col)) => Some(MoveQualifier::Col(col)),
        (Some(row), Some(col)) => Some(MoveQualifier::Position(
            Position::new(row, col).map_err(|e| ParseError(e.to_string()))?,
        )),
    };

    let promoted_piece = match captures.get(5) {
        None => None,
        Some(piece) => Some(parse_piece_type(piece.as_str())?),
    };

    Ok(RegexMatch {
        move_to,
        piece_type,
        move_qualifier,
        promoted_piece,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Position
    #[test]
    fn creates_position_from_string() {
        let position = parse_position("a4").unwrap();

        assert_eq!(position, Position::new(3, 0).unwrap())
    }

    #[test]
    fn fails_if_position_string_is_too_long() {
        let position = parse_position("a45");

        assert_eq!(
            position,
            Err(ParseError("String 'a45' should be length 2".to_string()))
        )
    }

    #[test]
    fn fails_if_position_string_is_too_short() {
        let position = parse_position("a");

        assert_eq!(
            position,
            Err(ParseError("String 'a' should be length 2".to_string()))
        )
    }

    #[test]
    fn fails_if_not_valid_position() {
        let position = parse_position("a9");

        assert_eq!(
            position,
            Err(ParseError("'9' is not a valid row".to_string()))
        )
    }

    // Ply
    #[test]
    fn parses_kingside_castle() {
        let ply = parse_ply("O-O", PieceColour::Black).unwrap();
        assert_eq!(ply, Ply::KingsideCastle(PieceColour::Black))
    }

    #[test]
    fn parses_queenside_castle() {
        let ply = parse_ply("O-O-O", PieceColour::White).unwrap();
        assert_eq!(ply, Ply::QueensideCastle(PieceColour::White))
    }

    #[test]
    fn parses_pawn_move() {
        let ply = parse_ply("e4", PieceColour::White).unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(
                    PieceType::Pawn,
                    PieceColour::White,
                    Position::new(3, 4).unwrap()
                ),
                qualifier: None
            }
        )
    }

    #[test]
    fn parses_piece_move() {
        let ply = parse_ply("Be5", PieceColour::Black).unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(
                    PieceType::Bishop,
                    PieceColour::Black,
                    Position::new(4, 4).unwrap()
                ),
                qualifier: None
            }
        )
    }

    #[test]
    fn parses_piece_move_with_column_qualifier() {
        let ply = parse_ply("Rde5", PieceColour::Black).unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(
                    PieceType::Rook,
                    PieceColour::Black,
                    Position::new(4, 4).unwrap()
                ),
                qualifier: Some(MoveQualifier::Col(3))
            }
        )
    }

    #[test]
    fn parses_piece_move_with_row_qualifier() {
        let ply = parse_ply("Q6e5", PieceColour::White).unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(
                    PieceType::Queen,
                    PieceColour::White,
                    Position::new(4, 4).unwrap()
                ),
                qualifier: Some(MoveQualifier::Row(5))
            }
        )
    }

    #[test]
    fn parses_piece_move_with_position_qualifier() {
        let ply = parse_ply("Nf3e5", PieceColour::Black).unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(
                    PieceType::Knight,
                    PieceColour::Black,
                    Position::new(4, 4).unwrap()
                ),
                qualifier: Some(MoveQualifier::Position(Position::new(2, 5).unwrap()))
            }
        )
    }

    #[test]
    fn parses_pawn_capture() {
        let ply = parse_ply("exd5", PieceColour::Black).unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(
                    PieceType::Pawn,
                    PieceColour::Black,
                    Position::new(4, 3).unwrap()
                ),
                qualifier: Some(MoveQualifier::Col(4))
            }
        )
    }

    #[test]
    fn parses_piece_capture() {
        let ply = parse_ply("Bxd5", PieceColour::White).unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(
                    PieceType::Bishop,
                    PieceColour::White,
                    Position::new(4, 3).unwrap()
                ),
                qualifier: None
            }
        )
    }

    #[test]
    fn parses_piece_capture_with_column_qualifier() {
        let ply = parse_ply("Rdxe5", PieceColour::White).unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(
                    PieceType::Rook,
                    PieceColour::White,
                    Position::new(4, 4).unwrap()
                ),
                qualifier: Some(MoveQualifier::Col(3))
            }
        )
    }

    #[test]
    fn parses_piece_capture_with_row_qualifier() {
        let ply = parse_ply("R2xe5", PieceColour::Black).unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(
                    PieceType::Rook,
                    PieceColour::Black,
                    Position::new(4, 4).unwrap()
                ),
                qualifier: Some(MoveQualifier::Row(1))
            }
        )
    }

    #[test]
    fn parses_piece_capture_with_position_qualifier() {
        let ply = parse_ply("Qe2xe5", PieceColour::Black).unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(
                    PieceType::Queen,
                    PieceColour::Black,
                    Position::new(4, 4).unwrap()
                ),
                qualifier: Some(MoveQualifier::Position(Position::new(1, 4).unwrap()))
            }
        )
    }

    #[test]
    fn parses_pawn_promotion() {
        let ply = parse_ply("e8=Q", PieceColour::White).unwrap();
        assert_eq!(
            ply,
            Ply::Promotion {
                movement: Movement::new(
                    PieceType::Pawn,
                    PieceColour::White,
                    Position::new(7, 4).unwrap()
                ),
                promotes_to: PieceType::Queen,
                qualifier: None
            }
        )
    }

    #[test]
    fn parses_pawn_capture_with_promotion() {
        let ply = parse_ply("exd8=N", PieceColour::Black).unwrap();
        assert_eq!(
            ply,
            Ply::Promotion {
                movement: Movement::new(
                    PieceType::Pawn,
                    PieceColour::Black,
                    Position::new(7, 3).unwrap()
                ),
                promotes_to: PieceType::Knight,
                qualifier: Some(MoveQualifier::Col(4))
            }
        )
    }
}
