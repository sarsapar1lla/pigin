use std::collections::HashMap;

use super::{Piece, PieceColour, Ply};

pub type Tags = HashMap<String, String>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameResult {
    BlackWin,
    WhiteWin,
    Draw,
    Ongoing,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Fen {
    pieces: Vec<Piece>,
    active_colour: PieceColour,
    move_number: i8,
}

impl Fen {
    pub fn new(pieces: Vec<Piece>, active_colour: PieceColour, move_number: i8) -> Self {
        Fen {
            pieces,
            active_colour,
            move_number,
        }
    }

    pub fn pieces(&self) -> &Vec<Piece> {
        &self.pieces
    }

    pub fn active_colour(&self) -> &PieceColour {
        &self.active_colour
    }

    pub fn move_number(&self) -> i8 {
        self.move_number
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pgn {
    tags: Tags,
    fen: Fen,
    result: GameResult,
    ply_list: Vec<Ply>,
}

impl Pgn {
    pub fn new(tags: Tags, fen: Fen, result: GameResult, ply_list: Vec<Ply>) -> Self {
        Pgn {
            tags,
            fen,
            result,
            ply_list,
        }
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    pub fn fen(&self) -> &Fen {
        &self.fen
    }

    pub fn result(&self) -> GameResult {
        self.result
    }

    pub fn ply_list(&self) -> &Vec<Ply> {
        &self.ply_list
    }
}
