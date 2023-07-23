use crate::model::piece::PieceType;
use crate::model::position::Position;

use super::{Piece, PieceColour};

#[derive(Debug, PartialEq, Eq)]
pub struct Movement {
    piece: Piece,
    position: Position,
}

impl Movement {
    pub fn new(piece: Piece, position: Position) -> Self {
        Movement { piece, position }
    }

    pub fn piece(&self) -> Piece {
        self.piece
    }

    pub fn position(&self) -> Position {
        self.position
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
    movement: PlyMovement,
    comment: Option<String>,
}

impl Ply {
    pub fn new(move_number: i8, ply: PlyMovement, comment: Option<String>) -> Self {
        Ply {
            move_number,
            movement: ply,
            comment,
        }
    }

    pub fn move_number(&self) -> i8 {
        self.move_number
    }

    pub fn movement(&self) -> &PlyMovement {
        &self.movement
    }

    pub fn comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }
}
