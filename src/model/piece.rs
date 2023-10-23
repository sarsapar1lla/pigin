const BLACK_PAWN: &str = "♙";
const BLACK_KNIGHT: &str = "♘";
const BLACK_BISHOP: &str = "♗";
const BLACK_ROOK: &str = "♖";
const BLACK_QUEEN: &str = "♕";
const BLACK_KING: &str = "♔";

const WHITE_PAWN: &str = "♟";
const WHITE_KNIGHT: &str = "♞";
const WHITE_BISHOP: &str = "♝";
const WHITE_ROOK: &str = "♜";
const WHITE_QUEEN: &str = "♛";
const WHITE_KING: &str = "♚";

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

    pub fn unicode(&self) -> &'static str {
        match (self.colour, self.piece_type) {
            // Black pieces
            (PieceColour::Black, PieceType::Pawn) => BLACK_PAWN,
            (PieceColour::Black, PieceType::Knight) => BLACK_KNIGHT,
            (PieceColour::Black, PieceType::Bishop) => BLACK_BISHOP,
            (PieceColour::Black, PieceType::Rook) => BLACK_ROOK,
            (PieceColour::Black, PieceType::Queen) => BLACK_QUEEN,
            (PieceColour::Black, PieceType::King) => BLACK_KING,
            // White pieces
            (PieceColour::White, PieceType::Pawn) => WHITE_PAWN,
            (PieceColour::White, PieceType::Knight) => WHITE_KNIGHT,
            (PieceColour::White, PieceType::Bishop) => WHITE_BISHOP,
            (PieceColour::White, PieceType::Rook) => WHITE_ROOK,
            (PieceColour::White, PieceType::Queen) => WHITE_QUEEN,
            (PieceColour::White, PieceType::King) => WHITE_KING,
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.unicode())
    }
}
