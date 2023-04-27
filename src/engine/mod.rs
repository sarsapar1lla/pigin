use crate::model::{Piece, PieceColour, Ply};

mod move_generator;
mod transformer;

pub struct Engine {
    pieces: Vec<Piece>,
}

impl Engine {
    pub fn new() -> Self {
        Engine { pieces: Vec::new() }
    }

    pub fn pieces(&self) -> &Vec<Piece> {
        &self.pieces
    }

    pub fn execute(&mut self, _ply: Ply, _to_move: PieceColour) {
        todo!()
    }
}
