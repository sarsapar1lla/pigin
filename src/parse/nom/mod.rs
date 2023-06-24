mod error;
mod movement;
mod ply;
mod tag;

use crate::parse::nom::tag::parse_tags;

pub fn parse() {
    let file = "[Tag \"Value\"]\n[Another \"Something\"]\n\n1. e4 e5 {This is a comment} 2.Nc3 Nf6";
    let (remaining, tags) = parse_tags(file).unwrap();
}
