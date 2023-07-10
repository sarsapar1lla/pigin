use ndarray::Array2;

use super::PieceType;
use super::{Piece, Position};

pub struct BoardUpdateError;

#[derive(Debug, PartialEq, Eq)]
pub enum AvailableCastle {
    WhiteKingside,
    WhiteQueenside,
    BlackKingside,
    BlackQueenside,
}

pub enum UpdateType {
    Move,
    Castle,
}

pub struct BoardUpdate {
    update_type: UpdateType,
    from: Position,
    to: Position,
    promotes_to: Option<PieceType>,
    castling_availability: Vec<AvailableCastle>,
    en_passant_square: Option<Position>,
}

pub struct Board {
    grid: Array2<Piece>,
    castling_availability: Vec<AvailableCastle>,
    en_passant_square: Option<Position>,
}

impl Board {
    pub fn new(grid: Array2<Piece>, castling_availability: Vec<AvailableCastle>, en_passant_square: Option<Position>) -> Self {
        Board { grid, castling_availability, en_passant_square }
    }

    pub fn occupant(&self, position: &Position) -> Option<&Piece> {
        self.grid.get((position.row() as usize, position.col() as usize))
    }

    pub fn apply(&self, update: BoardUpdate) -> Result<Board, BoardUpdateError> {
        let mut grid = self.grid.clone();



        todo!()
    }
}
