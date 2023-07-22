use crate::model::piece::PieceType;
use crate::model::position::Position;

use super::PieceColour;

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
pub enum Check {
    Check,
    Checkmate,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlyMovement {
    Move {
        movement: Movement,
        qualifier: Option<MoveQualifier>,
        check: Option<Check>,
    },
    KingsideCastle {
        colour: PieceColour,
        check: Option<Check>,
    },
    QueensideCastle {
        colour: PieceColour,
        check: Option<Check>,
    },
    Promotion {
        movement: Movement,
        promotes_to: PieceType,
        qualifier: Option<MoveQualifier>,
        check: Option<Check>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ply {
    move_number: i8,
    ply: PlyMovement,
    comment: Option<String>,
}

impl Ply {
    pub fn new(move_number: i8, ply: PlyMovement, comment: Option<String>) -> Self {
        Ply {
            move_number,
            ply,
            comment,
        }
    }
}
