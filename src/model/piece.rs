use super::position::Position;

#[derive(Debug, PartialEq, Eq)]
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
}

impl Piece {
    pub fn new(piece_colour: PieceColour, piece_type: PieceType, position: Position) -> Self {
        Piece {
            unicode: Self::unicode(&piece_colour, &piece_type),
            colour: piece_colour,
            piece_type,
            position,
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

    fn unicode(piece_colour: &PieceColour, piece_type: &PieceType) -> String {
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
