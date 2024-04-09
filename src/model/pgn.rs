use std::collections::HashMap;

use super::{board::Board, PieceColour, Ply};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameResult {
    BlackWin,
    WhiteWin,
    Draw,
    Ongoing,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Fen {
    starting_board: Board,
    active_colour: PieceColour,
    fullmove_clock: usize,
}

impl Fen {
    pub fn new(starting_board: Board, active_colour: PieceColour, fullmove_clock: usize) -> Self {
        Fen {
            starting_board,
            active_colour,
            fullmove_clock,
        }
    }

    pub fn starting_board(&self) -> &Board {
        &self.starting_board
    }

    pub fn active_colour(&self) -> &PieceColour {
        &self.active_colour
    }

    pub fn fullmove_clock(&self) -> usize {
        self.fullmove_clock
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tags(HashMap<String, String>);

impl Tags {
    pub fn new(tags: HashMap<String, String>) -> Self {
        Self(tags)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    pub fn get_or_default(&self, key: &str, default: &str) -> String {
        self.get(key)
            .map_or_else(|| default.to_string(), ToString::to_string)
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.0.remove(key)
    }

    pub fn inner(&self) -> &HashMap<String, String> {
        &self.0
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
