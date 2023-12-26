#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceColour {
    Black,
    White,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Piece {
    colour: PieceColour,
    piece_type: PieceType,
}

impl Piece {
    pub fn new(piece_colour: PieceColour, piece_type: PieceType) -> Self {
        Piece {
            colour: piece_colour,
            piece_type,
        }
    }

    pub fn colour(&self) -> &PieceColour {
        &self.colour
    }

    pub fn piece_type(&self) -> &PieceType {
        &self.piece_type
    }
}
