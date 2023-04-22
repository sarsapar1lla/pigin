use super::position::Position;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceColour {
    Black,
    White,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Piece {
    colour: PieceColour,
    piece_type: PieceType,
    position: Position,
    unicode: String,
    has_moved: bool,
}

impl Piece {
    pub fn new(
        piece_colour: PieceColour,
        piece_type: PieceType,
        position: Position,
        has_moved: bool,
    ) -> Self {
        Piece {
            unicode: Self::unicode(piece_colour, &piece_type),
            colour: piece_colour,
            piece_type,
            position,
            has_moved,
        }
    }

    pub fn colour(&self) -> &PieceColour {
        &self.colour
    }

    pub fn piece_type(&self) -> &PieceType {
        &self.piece_type
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn has_moved(&self) -> &bool {
        &self.has_moved
    }

    fn unicode(piece_colour: PieceColour, piece_type: &PieceType) -> String {
        match (piece_colour, piece_type) {
            // Black pieces
            (PieceColour::Black, PieceType::Pawn) => "♟",
            (PieceColour::Black, PieceType::Knight) => "♞",
            (PieceColour::Black, PieceType::Bishop) => "♝",
            (PieceColour::Black, PieceType::Rook) => "♜",
            (PieceColour::Black, PieceType::Queen) => "♛",
            (PieceColour::Black, PieceType::King) => "♚",
            // White pieces
            (PieceColour::White, PieceType::Pawn) => "♙",
            (PieceColour::White, PieceType::Knight) => "♘",
            (PieceColour::White, PieceType::Bishop) => "♗",
            (PieceColour::White, PieceType::Rook) => "♖",
            (PieceColour::White, PieceType::Queen) => "♕",
            (PieceColour::White, PieceType::King) => "♔",
        }
        .to_string()
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.unicode)
    }
}
