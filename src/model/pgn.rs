use std::collections::HashMap;

use super::{board::Board, PieceColour, Ply};

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
    starting_board: Board,
    active_colour: PieceColour,
    move_number: u8,
}

impl Fen {
    pub fn new(starting_board: Board, active_colour: PieceColour, move_number: u8) -> Self {
        Fen {
            starting_board,
            active_colour,
            move_number,
        }
    }

    pub fn starting_board(&self) -> &Board {
        &self.starting_board
    }

    pub fn active_colour(&self) -> &PieceColour {
        &self.active_colour
    }

    pub fn move_number(&self) -> u8 {
        self.move_number
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pgn {
    tags: Tags,
    fen: Fen,
    result: GameResult,
    ply: Vec<Ply>,
}

impl Pgn {
    pub fn new(tags: Tags, fen: Fen, result: GameResult, ply_list: Vec<Ply>) -> Self {
        Pgn {
            tags,
            fen,
            result,
            ply: ply_list,
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

    pub fn ply(&self) -> &[Ply] {
        &self.ply
    }
}
