use std::collections::HashMap;

use super::{Piece, PieceColour, PlyMetadata};

pub type Tags = HashMap<String, String>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameResult {
    BlackWin,
    WhiteWin,
    Draw,
    Ongoing,
}

impl ToString for GameResult {
    fn to_string(&self) -> String {
        match self {
            Self::BlackWin => "0-1".to_string(),
            Self::WhiteWin => "1-0".to_string(),
            Self::Draw => "1/2-1/2".to_string(),
            Self::Ongoing => "*".to_string(),
        }
    }
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
    ply_list: Vec<PlyMetadata>,
}

impl Pgn {
    pub fn new(tags: Tags, fen: Fen, result: GameResult, ply_list: Vec<PlyMetadata>) -> Self {
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

    pub fn ply_list(&self) -> &Vec<PlyMetadata> {
        &self.ply_list
    }
}
