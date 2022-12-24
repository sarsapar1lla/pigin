use std::num::ParseIntError;

use regex::Regex;

use crate::model::Fen;
use crate::model::Piece;
use crate::model::PieceColour;
use crate::model::PieceType;
use crate::model::Position;
use crate::model::{MoveQualifier, Movement, Ply};

static COLS: &str = "abcdefgh";
static ROWS: &str = "12345678";
pub static DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
static PLY_PATTERN: &str = r"^([NBRQK])?([a-h])?([1-8])?x?([a-h][1-8])=?([NBRQK])?$";

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError(String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn parse_fen(s: &str) -> Result<Fen, ParseError> {
    let parts: Vec<&str> = s.split(' ').collect();
    let pieces_str = parts.first();
    let active_colour_str = parts.get(1);
    let move_number_str = parts.get(5);

    let pieces =
        parse_pieces_from_string(pieces_str.ok_or_else(|| {
            ParseError(format!("Could not extract pieces from FEN string '{}'", s))
        })?)?;
    let active_colour = parse_colour(active_colour_str.ok_or_else(|| {
        ParseError(format!(
            "Could not extract active colour from FEN string '{}'",
            s
        ))
    })?)?;
    let move_number: i8 = move_number_str
        .ok_or_else(|| {
            ParseError(format!(
                "Could not extract move number from FEN string '{}'",
                s
            ))
        })?
        .parse()
        .map_err(|e: ParseIntError| ParseError(e.to_string()))?;

    Ok(Fen::new(pieces, active_colour, move_number))
}

pub fn parse_ply(s: &str) -> Result<Ply, ParseError> {
    // Handle castling cases first
    if s == "O-O" {
        return Ok(Ply::KingsideCastle);
    }

    if s == "O-O-O" {
        return Ok(Ply::QueensideCastle);
    }

    let regex_match = match_with_regex(s)?;

    match regex_match {
        RegexMatch {
            move_to,
            piece_type,
            move_qualifier,
            promoted_piece: None,
        } => {
            let movement = Movement::new(piece_type, move_to);
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
            let movement = Movement::new(piece_type, move_to);
            Ok(Ply::Promotion {
                movement,
                promotes_to: promoted_piece,
                qualifier: move_qualifier,
            })
        }
    }
}

fn parse_pieces_from_string(s: &str) -> Result<Vec<Piece>, ParseError> {
    let mut pieces: Vec<Piece> = Vec::new();

    let mut row = 7;
    let mut col = 0;

    for p in s.chars() {
        if p == '/' {
            row -= 1;
            col = 0;
            continue;
        }

        if p.is_numeric() {
            col += p
                .to_digit(10)
                .ok_or_else(|| ParseError(format!("Could not convert '{}' to digit", p)))?
                as i8;
            continue;
        }

        let piece_colour = if p.is_uppercase() {
            PieceColour::White
        } else {
            PieceColour::Black
        };
        let piece_type = parse_piece_type(p.to_uppercase().to_string().as_str())?;
        let position = Position::new(row, col);
        pieces.push(Piece::new(piece_colour, piece_type, position));

        col += 1;
    }
    Ok(pieces)
}

fn parse_colour(s: &str) -> Result<PieceColour, ParseError> {
    match s {
        "w" => Ok(PieceColour::White),
        "b" => Ok(PieceColour::Black),
        _ => Err(ParseError(format!("'{}' is not a valid colour", s))),
    }
}

fn parse_piece_type(s: &str) -> Result<PieceType, ParseError> {
    match s {
        "P" => Ok(PieceType::Pawn),
        "N" => Ok(PieceType::Knight),
        "B" => Ok(PieceType::Bishop),
        "R" => Ok(PieceType::Rook),
        "Q" => Ok(PieceType::Queen),
        "K" => Ok(PieceType::King),
        _ => Err(ParseError(format!("'{}' is not a valid piece type", s))),
    }
}

fn parse_position(s: &str) -> Result<Position, ParseError> {
    if s.len() != 2 {
        return Err(ParseError(format!("String '{}' should be length 2", s)));
    }

    let row_char: char = s
        .chars()
        .nth(1)
        .ok_or_else(|| ParseError(format!("Couldn't extract row from string '{}'", s)))?;
    let col_char: char = s
        .chars()
        .next()
        .ok_or_else(|| ParseError(format!("Couldn't extract column from string '{}'", s)))?;

    let row = get_row_from_char(row_char)?;
    let col = get_col_from_char(col_char)?;

    Ok(Position::new(row, col))
}

fn get_col_from_char(c: char) -> Result<i8, ParseError> {
    Ok(COLS
        .find(c)
        .ok_or_else(|| ParseError(format!("'{}' is not a valid column", c)))? as i8)
}

fn get_row_from_char(c: char) -> Result<i8, ParseError> {
    Ok(ROWS
        .find(c)
        .ok_or_else(|| ParseError(format!("'{}' is not a valid row", c)))? as i8)
}

// Ply
struct RegexMatch {
    move_to: Position,
    piece_type: PieceType,
    move_qualifier: Option<MoveQualifier>,
    promoted_piece: Option<PieceType>,
}

fn match_with_regex(s: &str) -> Result<RegexMatch, ParseError> {
    let pattern = Regex::new(PLY_PATTERN)
        .map_err(|_| ParseError("Could not compile regex pattern".to_string()))?;

    let captures = pattern
        .captures(s)
        .ok_or_else(|| ParseError(format!("'{}' could not be parsed", s)))?;

    let move_to = parse_position(
        captures
            .get(4)
            .ok_or_else(|| ParseError(format!("{} did not contain move position", s)))?
            .as_str(),
    )
    .map_err(|e| ParseError(e.to_string()))?;

    let piece_type = match captures.get(1) {
        None => PieceType::Pawn,
        Some(piece_type) => {
            parse_piece_type(piece_type.as_str()).map_err(|e| ParseError(e.to_string()))?
        }
    };

    let column_qualifier = match captures.get(2) {
        None => None,
        Some(col) => {
            let c =
                col.as_str().chars().next().ok_or_else(|| {
                    ParseError(format!("Column capture {} is empty", col.as_str()))
                })?;
            Some(get_col_from_char(c).map_err(|e| ParseError(e.to_string()))?)
        }
    };

    let row_qualifier = match captures.get(3) {
        None => None,
        Some(row) => {
            let c = row
                .as_str()
                .chars()
                .next()
                .ok_or_else(|| ParseError(format!("Row capture {} is empty", row.as_str())))?;
            Some(get_row_from_char(c).map_err(|e| ParseError(e.to_string()))?)
        }
    };

    let move_qualifier = match (row_qualifier, column_qualifier) {
        (None, None) => None,
        (Some(row), None) => Some(MoveQualifier::Row(row)),
        (None, Some(col)) => Some(MoveQualifier::Col(col)),
        (Some(row), Some(col)) => Some(MoveQualifier::Position(Position::new(row, col))),
    };

    let promoted_piece = match captures.get(5) {
        None => None,
        Some(piece) => {
            Some(parse_piece_type(piece.as_str()).map_err(|e| ParseError(e.to_string()))?)
        }
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

    // Piece colour
    #[test]
    fn parses_string_to_piece_colour() {
        assert_eq!(parse_colour("w"), Ok(PieceColour::White))
    }

    #[test]
    fn returns_error_when_not_valid_piece_colour() {
        assert_eq!(
            parse_colour("invalid"),
            Err(ParseError("'invalid' is not a valid colour".to_string()))
        )
    }

    // Piece type
    #[test]
    fn parses_string_to_piece_type() {
        assert_eq!(parse_piece_type("B"), Ok(PieceType::Bishop))
    }

    #[test]
    fn returns_error_when_not_valid_piece_type() {
        assert_eq!(
            parse_piece_type("invalid"),
            Err(ParseError(
                "'invalid' is not a valid piece type".to_string()
            ))
        )
    }

    // Position
    #[test]
    fn creates_position_from_string() {
        let position = parse_position("a4").unwrap();

        assert_eq!(position, Position::new(3, 0))
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
        let ply = parse_ply("O-O").unwrap();
        assert_eq!(ply, Ply::KingsideCastle)
    }

    #[test]
    fn parses_queenside_castle() {
        let ply = parse_ply("O-O-O").unwrap();
        assert_eq!(ply, Ply::QueensideCastle)
    }

    #[test]
    fn parses_pawn_move() {
        let ply = parse_ply("e4").unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(PieceType::Pawn, Position::new(3, 4)),
                qualifier: None
            }
        )
    }

    #[test]
    fn parses_piece_move() {
        let ply = parse_ply("Be5").unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(PieceType::Bishop, Position::new(4, 4)),
                qualifier: None
            }
        )
    }

    #[test]
    fn parses_piece_move_with_column_qualifier() {
        let ply = parse_ply("Rde5").unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(PieceType::Rook, Position::new(4, 4)),
                qualifier: Some(MoveQualifier::Col(3))
            }
        )
    }

    #[test]
    fn parses_piece_move_with_row_qualifier() {
        let ply = parse_ply("Q6e5").unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(PieceType::Queen, Position::new(4, 4)),
                qualifier: Some(MoveQualifier::Row(5))
            }
        )
    }

    #[test]
    fn parses_piece_move_with_position_qualifier() {
        let ply = parse_ply("Nf3e5").unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(PieceType::Knight, Position::new(4, 4)),
                qualifier: Some(MoveQualifier::Position(Position::new(2, 5)))
            }
        )
    }

    #[test]
    fn parses_pawn_capture() {
        let ply = parse_ply("exd5").unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(PieceType::Pawn, Position::new(4, 3)),
                qualifier: Some(MoveQualifier::Col(4))
            }
        )
    }

    #[test]
    fn parses_piece_capture() {
        let ply = parse_ply("Bxd5").unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(PieceType::Bishop, Position::new(4, 3)),
                qualifier: None
            }
        )
    }

    #[test]
    fn parses_piece_capture_with_column_qualifier() {
        let ply = parse_ply("Rdxe5").unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(PieceType::Rook, Position::new(4, 4)),
                qualifier: Some(MoveQualifier::Col(3))
            }
        )
    }

    #[test]
    fn parses_piece_capture_with_row_qualifier() {
        let ply = parse_ply("R2xe5").unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(PieceType::Rook, Position::new(4, 4)),
                qualifier: Some(MoveQualifier::Row(1))
            }
        )
    }

    #[test]
    fn parses_piece_capture_with_position_qualifier() {
        let ply = parse_ply("Qe2xe5").unwrap();
        assert_eq!(
            ply,
            Ply::Move {
                movement: Movement::new(PieceType::Queen, Position::new(4, 4)),
                qualifier: Some(MoveQualifier::Position(Position::new(1, 4)))
            }
        )
    }

    #[test]
    fn parses_pawn_promotion() {
        let ply = parse_ply("e8=Q").unwrap();
        assert_eq!(
            ply,
            Ply::Promotion {
                movement: Movement::new(PieceType::Pawn, Position::new(7, 4)),
                promotes_to: PieceType::Queen,
                qualifier: None
            }
        )
    }

    #[test]
    fn parses_pawn_capture_with_promotion() {
        let ply = parse_ply("exd8=N").unwrap();
        assert_eq!(
            ply,
            Ply::Promotion {
                movement: Movement::new(PieceType::Pawn, Position::new(7, 3)),
                promotes_to: PieceType::Knight,
                qualifier: Some(MoveQualifier::Col(4))
            }
        )
    }

    // FEN
    #[test]
    fn parses_pieces_from_string() {
        let pieces = parse_pieces_from_string("r2b2q/5PN").unwrap();
        let expected = vec![
            Piece::new(PieceColour::Black, PieceType::Rook, Position::new(7, 0)),
            Piece::new(PieceColour::Black, PieceType::Bishop, Position::new(7, 3)),
            Piece::new(PieceColour::Black, PieceType::Queen, Position::new(7, 6)),
            Piece::new(PieceColour::White, PieceType::Pawn, Position::new(6, 5)),
            Piece::new(PieceColour::White, PieceType::Knight, Position::new(6, 6)),
        ];
        assert_eq!(pieces, expected)
    }
}
