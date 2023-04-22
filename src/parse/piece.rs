use super::error::ParseError;
use crate::model::{PieceColour, PieceType};

pub fn parse_colour(s: &str) -> Result<PieceColour, ParseError> {
    match s {
        "w" => Ok(PieceColour::White),
        "b" => Ok(PieceColour::Black),
        _ => Err(ParseError(format!("'{s}' is not a valid colour"))),
    }
}

pub fn parse_piece_type(s: &str) -> Result<PieceType, ParseError> {
    match s {
        "P" => Ok(PieceType::Pawn),
        "N" => Ok(PieceType::Knight),
        "B" => Ok(PieceType::Bishop),
        "R" => Ok(PieceType::Rook),
        "Q" => Ok(PieceType::Queen),
        "K" => Ok(PieceType::King),
        _ => Err(ParseError(format!("'{s}' is not a valid piece type"))),
    }
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
}
