use std::{collections::HashMap, str::Lines};

use regex::Regex;

use super::{
    error::ParseError,
    piece::{parse_colour, parse_piece_type},
    ply::parse_ply,
};
use crate::model::{Fen, GameResult, Pgn, Piece, PieceColour, PlyMetadata, Position, Tags};

pub static DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn parse_file(content: &str) -> Result<Pgn, ParseError> {
    let lines = content.lines();

    let (mut tags, remaining) = parse_tags(lines.clone())?;

    let fen = parse_fen(
        &tags
            .remove("FEN")
            .unwrap_or_else(|| DEFAULT_FEN.to_string()),
    )?;
    let result = game_result_from_string(
        &tags
            .remove("Result")
            .ok_or_else(|| ParseError("Missing 'Result' tag".to_string()))?,
    )?;

    let cleaned_remaining = clean_move_string(remaining, &result);

    let ply_list = parse_ply_list(cleaned_remaining)?;

    Ok(Pgn::new(tags, fen, result, ply_list))
}

fn parse_tags(lines: Lines) -> Result<(Tags, String), ParseError> {
    let mut tags: Tags = HashMap::new();
    let mut remaining = String::new();

    for line in lines {
        let tag = tag_from_string(line)?;
        match tag {
            Some(tag) => {
                tags.insert(tag.0, tag.1);
            }
            None => remaining.push_str(&format!("{line}\n")),
        }
    }

    Ok((tags, remaining))
}

fn clean_move_string(mut move_string: String, result: &GameResult) -> String {
    // Result is included at the end of the file if not ongoing
    if result != &GameResult::Ongoing {
        move_string = move_string.replace(&result.to_string(), "");
    }

    move_string = move_string.trim().to_string();

    // Remove multiple whitespaces
    let elements: Vec<&str> = move_string.split_whitespace().collect();
    elements.join(" ")
}

fn parse_ply_list(mut remaining: String) -> Result<Vec<PlyMetadata>, ParseError> {
    let white_move_regex = Regex::new(r"^(\d+)\. ?([=\+#\w-]+)( \{.+\})?")
        .map_err(|e| ParseError(format!("Failed to compile regex: {e}")))?;
    let black_move_regex = Regex::new(r"^ ([=\+#\w-]+)( \{.+\})?")
        .map_err(|e| ParseError(format!("Failed to compile regex: {e}")))?;

    let mut ply_list: Vec<PlyMetadata> = Vec::new();

    loop {
        let white_captures = white_move_regex.captures(&remaining).ok_or_else(|| {
            ParseError(format!("Failed to capture white move given '{remaining}'"))
        })?;

        let full_white_capture = white_captures
            .get(0)
            .ok_or_else(|| ParseError(format!("Failed to capture white move given '{remaining}'")))
            .map(|capture| capture.as_str())?;

        let move_number: i8 = white_captures
            .get(1)
            .ok_or_else(|| ParseError(format!("Failed to capture move number given '{remaining}'")))
            .and_then(|capture| {
                capture
                    .as_str()
                    .parse()
                    .map_err(|e| ParseError(format!("Failed to parse to i8: {e}")))
            })?;

        let white_comment = white_captures
            .get(3)
            .map(|capture| capture.as_str().trim().to_string().replace(['{', '}'], ""));
        let white_ply = white_captures
            .get(2)
            .ok_or_else(|| ParseError(format!("Failed to capture white ply given '{remaining}'")))
            .and_then(|capture| parse_ply(capture.as_str(), PieceColour::White))
            .map(|ply| PlyMetadata::new(move_number, ply, white_comment))?;

        ply_list.push(white_ply);

        remaining = remaining.replacen(full_white_capture, "", 1);

        if remaining.is_empty() {
            break;
        }

        let black_captures = black_move_regex.captures(&remaining).ok_or_else(|| {
            ParseError(format!("Failed to capture black move given '{remaining}'"))
        })?;

        let full_black_capture = black_captures
            .get(0)
            .ok_or_else(|| ParseError(format!("Failed to capture black move given '{remaining}'")))
            .map(|capture| capture.as_str())?;

        let black_comment = black_captures
            .get(2)
            .map(|capture| capture.as_str().trim().to_string().replace(['{', '}'], ""));
        let black_ply = black_captures
            .get(1)
            .ok_or_else(|| ParseError(format!("Failed to capture white ply given '{remaining}'")))
            .and_then(|capture| parse_ply(capture.as_str(), PieceColour::Black))
            .map(|ply| PlyMetadata::new(move_number, ply, black_comment))?;

        ply_list.push(black_ply);

        remaining = remaining.replacen(full_black_capture, "", 1);
        remaining = remaining.trim().to_string();

        if remaining.is_empty() {
            break;
        }
    }

    Ok(ply_list)
}

fn tag_from_string(s: &str) -> Result<Option<(String, String)>, ParseError> {
    let captures = Regex::new(r#"^\[(\w+) "(.+)"\]$"#)
        .map_err(|e| ParseError(format!("Failed to compile regex: {e}")))?
        .captures(s);

    match captures {
        None => Ok(None),
        Some(captures) => {
            let key = captures.get(1).unwrap().as_str().to_string();

            let value = captures.get(2).unwrap().as_str().to_string();

            Ok(Some((key, value)))
        }
    }
}

fn game_result_from_string(s: &str) -> Result<GameResult, ParseError> {
    match s {
        "1-0" => Ok(GameResult::WhiteWin),
        "0-1" => Ok(GameResult::BlackWin),
        "1/2-1/2" => Ok(GameResult::Draw),
        "*" => Ok(GameResult::Ongoing),
        _ => Err(ParseError(format!("'{s}' is not a recognised game result"))),
    }
}

fn parse_fen(s: &str) -> Result<Fen, ParseError> {
    let parts: Vec<&str> = s.split(' ').collect();

    let pieces = parts
        .first()
        .and_then(|s| parse_pieces_from_string(s).ok())
        .ok_or_else(|| ParseError(format!("Could not extract pieces from FEN string '{s}'")))?;

    let active_colour = parts
        .get(1)
        .and_then(|s| parse_colour(s).ok())
        .ok_or_else(|| {
            ParseError(format!(
                "Could not extract active colour from FEN string '{s}'"
            ))
        })?;

    let move_number: i8 = parts.get(5).and_then(|s| s.parse().ok()).ok_or_else(|| {
        ParseError(format!(
            "Could not extract move number from FEN string '{s}'"
        ))
    })?;

    Ok(Fen::new(pieces, active_colour, move_number))
}

fn parse_pieces_from_string(s: &str) -> Result<Vec<Piece>, ParseError> {
    let mut pieces: Vec<Piece> = Vec::new();

    let mut row = 7;
    let mut col = 0;

    for p in s.chars() {
        if p == '/' {
            row -= 1;
            col = 0;
            continue;
        }

        if p.is_numeric() {
            col += p
                .to_digit(10)
                .ok_or_else(|| ParseError(format!("Could not convert '{p}' to digit")))
                .and_then(|d| {
                    i8::try_from(d).map_err(|e| {
                        ParseError(format!("Could not convert digit '{d}' to i8: {e}"))
                    })
                })?;
            continue;
        }

        let piece_colour = if p.is_uppercase() {
            PieceColour::White
        } else {
            PieceColour::Black
        };
        let piece_type = parse_piece_type(p.to_uppercase().to_string().as_str())?;
        let position = Position::new(row, col).map_err(|e| ParseError(e.to_string()))?;
        pieces.push(Piece::new(piece_colour, piece_type, position, false));

        col += 1;
    }
    Ok(pieces)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::model::{Movement, PieceType, Ply, Tags};

    // FEN
    #[test]
    fn parses_pieces_from_string() {
        let pieces = parse_pieces_from_string("r2b2q/5PN").unwrap();
        let expected = vec![
            Piece::new(
                PieceColour::Black,
                PieceType::Rook,
                Position::new(7, 0).unwrap(),
                false,
            ),
            Piece::new(
                PieceColour::Black,
                PieceType::Bishop,
                Position::new(7, 3).unwrap(),
                false,
            ),
            Piece::new(
                PieceColour::Black,
                PieceType::Queen,
                Position::new(7, 6).unwrap(),
                false,
            ),
            Piece::new(
                PieceColour::White,
                PieceType::Pawn,
                Position::new(6, 5).unwrap(),
                false,
            ),
            Piece::new(
                PieceColour::White,
                PieceType::Knight,
                Position::new(6, 6).unwrap(),
                false,
            ),
        ];
        assert_eq!(pieces, expected)
    }

    #[test]
    fn returns_error_if_invalid_pieces_string_format() {
        let pieces = parse_pieces_from_string("invalid");
        assert!(pieces.is_err())
    }

    #[test]
    fn parses_fen_from_string() {
        let fen = parse_fen("r/8/8/8/8/8/8/8 b - - 49 76").unwrap();
        assert_eq!(
            fen,
            Fen::new(
                vec![Piece::new(
                    PieceColour::Black,
                    PieceType::Rook,
                    Position::new(7, 0).unwrap(),
                    false
                )],
                PieceColour::Black,
                76
            )
        )
    }

    // PGN
    #[test]
    fn parses_pgn_file() {
        let content = std::fs::read_to_string("resources/test/test.pgn").unwrap();
        let pgn = parse_file(&content).unwrap();

        let mut expected_tags: Tags = HashMap::new();
        expected_tags.insert("White".to_string(), "Player, One".to_string());
        expected_tags.insert("Black".to_string(), "Player, Two".to_string());

        let expected_ply = vec![
            PlyMetadata::new(
                1,
                Ply::Move {
                    movement: Movement::new(
                        PieceType::Pawn,
                        PieceColour::White,
                        Position::new(3, 4).unwrap(),
                    ),
                    qualifier: None,
                },
                None,
            ),
            PlyMetadata::new(
                1,
                Ply::Move {
                    movement: Movement::new(
                        PieceType::Pawn,
                        PieceColour::Black,
                        Position::new(4, 4).unwrap(),
                    ),
                    qualifier: None,
                },
                None,
            ),
            PlyMetadata::new(
                2,
                Ply::Move {
                    movement: Movement::new(
                        PieceType::Knight,
                        PieceColour::White,
                        Position::new(2, 5).unwrap(),
                    ),
                    qualifier: None,
                },
                None,
            ),
            PlyMetadata::new(
                2,
                Ply::Move {
                    movement: Movement::new(
                        PieceType::Knight,
                        PieceColour::Black,
                        Position::new(5, 2).unwrap(),
                    ),
                    qualifier: None,
                },
                Some("This is a comment".to_string()),
            ),
        ];

        println!("{}", pgn.ply_list().len());
        for ply in pgn.ply_list() {
            println!("{ply:?}");
        }

        assert_eq!(pgn.tags(), &expected_tags);
        assert_eq!(pgn.result(), GameResult::Draw);
        for ply in expected_ply {
            println!("{ply:?}");
            println!("{}", pgn.ply_list().contains(&ply));
            assert!(pgn.ply_list().contains(&ply))
        }
    }
}
