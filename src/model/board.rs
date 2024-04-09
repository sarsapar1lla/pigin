use std::collections::HashMap;

use super::{Piece, PieceColour, Position};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AvailableCastle {
    WhiteKingside,
    WhiteQueenside,
    BlackKingside,
    BlackQueenside,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
    grid: HashMap<Position, Piece>,
    active_colour: PieceColour,
    available_castles: Vec<AvailableCastle>,
    en_passant_square: Option<Position>,
    halfmove_clock: usize,
    fullmove_clock: usize,
}

// TODO: add tests
impl Board {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn active_colour(&self) -> &PieceColour {
        &self.active_colour
    }

    pub fn available_castles(&self) -> &[AvailableCastle] {
        &self.available_castles
    }

    pub fn en_passant_square(&self) -> Option<&Position> {
        self.en_passant_square.as_ref()
    }

    pub fn halfmove_clock(&self) -> usize {
        self.halfmove_clock
    }

    pub fn fullmove_clock(&self) -> usize {
        self.fullmove_clock
    }

    pub fn occupant(&self, position: Position) -> Option<&Piece> {
        self.grid.get(&position)
    }

    // TODO: review implementation
    pub fn search(&self, piece: Piece) -> Vec<Position> {
        self.grid
            .iter()
            .filter_map(|(&key, &val)| if val == piece { Some(key) } else { None })
            .collect()
    }

    pub fn add(&mut self, piece: Piece, position: Position) {
        self.grid.insert(position, piece);
    }

    pub fn remove(&mut self, position: Position) {
        self.grid.remove(&position);
    }

    pub fn update_active_colour(&mut self, active_colour: PieceColour) {
        self.active_colour = active_colour;
    }

    pub fn remove_available_castle(&mut self, available_castle: AvailableCastle) {
        let index = self
            .available_castles
            .iter()
            .position(|x| x == &available_castle);
        if let Some(index) = index {
            self.available_castles.remove(index);
        }
    }

    pub fn remove_en_passant_square(&mut self) {
        self.en_passant_square.take();
    }

    pub fn update_en_passant_square(&mut self, position: Position) {
        self.en_passant_square.replace(position);
    }

    pub fn update_halfmove_clock(&mut self, halfmove_clock: usize) {
        self.halfmove_clock = halfmove_clock;
    }

    pub fn update_fullmove_clock(&mut self, fullmove_clock: usize) {
        self.fullmove_clock = fullmove_clock;
    }
}

pub struct Builder {
    grid: HashMap<Position, Piece>,
    active_colour: PieceColour,
    available_castles: Vec<AvailableCastle>,
    en_passant_square: Option<Position>,
    halfmove_clock: usize,
    fullmove_clock: usize,
}

// TODO: add tests
impl Builder {
    fn new() -> Self {
        Builder {
            grid: HashMap::new(),
            active_colour: PieceColour::White,
            available_castles: Vec::new(),
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_clock: 1,
        }
    }

    pub fn piece(&mut self, piece: Piece, position: Position) -> &mut Builder {
        self.grid.insert(position, piece);
        self
    }

    pub fn active_colour(&mut self, active_colour: PieceColour) -> &mut Builder {
        self.active_colour = active_colour;
        self
    }

    pub fn available_castles(&mut self, available_castles: Vec<AvailableCastle>) -> &mut Builder {
        self.available_castles = available_castles;
        self
    }

    pub fn en_passant_square(&mut self, position: Position) -> &mut Builder {
        let _result = self.en_passant_square.insert(position);
        self
    }

    pub fn halfmove_clock(&mut self, halfmove_clock: usize) -> &mut Builder {
        self.halfmove_clock = halfmove_clock;
        self
    }

    pub fn fullmove_clock(&mut self, fullmove_clock: usize) -> &mut Builder {
        self.fullmove_clock = fullmove_clock;
        self
    }

    pub fn build(self) -> Board {
        Board {
            grid: self.grid,
            active_colour: self.active_colour,
            available_castles: self.available_castles,
            en_passant_square: self.en_passant_square,
            halfmove_clock: self.halfmove_clock,
            fullmove_clock: self.fullmove_clock,
        }
    }
}
