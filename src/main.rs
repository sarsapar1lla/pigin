#![allow(dead_code)]

mod engine;
mod model;
mod parse;

use parse::{parse_fen, DEFAULT_FEN};

fn main() {
    let fen = parse_fen(DEFAULT_FEN);
    println!("{:?}", fen)
}
