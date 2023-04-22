use crate::model::piece::PieceType;
use crate::model::position::Position;

use super::PieceColour;

static PLY_PATTERN: &str = r"^([NBRQK])?([a-h])?([1-8])?x?([a-h][1-8])=?([NBRQK])?$";

#[derive(Debug)]
pub struct PlyParseError(String);

impl std::fmt::Display for PlyParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for PlyParseError {}

#[derive(Debug, PartialEq, Eq)]
pub struct Movement {
    piece_type: PieceType,
    piece_colour: PieceColour,
    position: Position,
}

impl Movement {
    pub fn new(piece_type: PieceType, piece_colour: PieceColour, position: Position) -> Self {
        Movement {
            piece_type,
            piece_colour,
            position,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MoveQualifier {
    Row(i8),
    Col(i8),
    Position(Position),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Ply {
    Move {
        movement: Movement,
        qualifier: Option<MoveQualifier>,
    },
    KingsideCastle(PieceColour),
    QueensideCastle(PieceColour),
    Promotion {
        movement: Movement,
        promotes_to: PieceType,
        qualifier: Option<MoveQualifier>,
    },
}
