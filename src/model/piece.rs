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
    pub fn new(
        piece_colour: PieceColour,
        piece_type: PieceType,
    ) -> Self {
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

    fn unicode(&self) -> String {
        match (self.colour, self.piece_type) {
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
        write!(f, "{}", self.unicode())
    }
}
