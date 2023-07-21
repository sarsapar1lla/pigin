use std::collections::HashMap;

use super::{Piece, Position};

#[derive(Debug, PartialEq, Eq)]
pub enum AvailableCastle {
    WhiteKingside,
    WhiteQueenside,
    BlackKingside,
    BlackQueenside,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    grid: HashMap<Position, Piece>,
    available_castles: Vec<AvailableCastle>,
    en_passant_square: Option<Position>,
}

// TODO: add tests
impl Board {
    pub fn occupant(&self, position: &Position) -> Option<&Piece> {
        self.grid.get(position)
    }

    pub fn add(&mut self, piece: Piece, position: &Position) {
        self.grid.insert(*position, piece);
    }

    pub fn remove(&mut self, position: &Position) {
        self.grid.remove(position);
    }
}

pub struct BoardBuilder {
    grid: HashMap<Position, Piece>,
    available_castles: Vec<AvailableCastle>,
    en_passant_square: Option<Position>,
}

// TODO: add tests
impl BoardBuilder {
    pub fn new() -> Self {
        BoardBuilder {
            grid: HashMap::new(),
            available_castles: Vec::new(),
            en_passant_square: None,
        }
    }

    pub fn available_castles(
        &mut self,
        available_castles: Vec<AvailableCastle>,
    ) -> &mut BoardBuilder {
        self.available_castles = available_castles;
        self
    }

    pub fn en_passant_square(&mut self, position: Position) -> &mut BoardBuilder {
        let _result = self.en_passant_square.insert(position);
        self
    }

    pub fn piece(&mut self, piece: Piece, position: Position) -> &mut BoardBuilder {
        self.grid.insert(position, piece);
        self
    }

    pub fn build(self) -> Board {
        Board {
            grid: self.grid,
            available_castles: self.available_castles,
            en_passant_square: self.en_passant_square,
        }
    }
}
