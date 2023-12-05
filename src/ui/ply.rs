use std::fmt::Display;

use crate::model::{
    Check, MoveQualifier, Movement, Piece, PieceColour, PieceType, Ply, PlyMovement, Position,
    COLUMNS, ROWS,
};

const BLACK_PAWN: &str = "P";
const BLACK_KNIGHT: &str = "N";
const BLACK_BISHOP: &str = "B";
const BLACK_ROOK: &str = "R";
const BLACK_QUEEN: &str = "Q";
const BLACK_KING: &str = "K";

const WHITE_PAWN: &str = "P";
const WHITE_KNIGHT: &str = "N";
const WHITE_BISHOP: &str = "B";
const WHITE_ROOK: &str = "R";
const WHITE_QUEEN: &str = "Q";
const WHITE_KING: &str = "K";

impl Display for Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Check::Check => "+".to_string(),
                Check::Checkmate => "#".to_string(),
            }
        )
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self.colour(), self.piece_type()) {
                // Black pieces
                (PieceColour::Black, PieceType::Pawn) => BLACK_PAWN,
                (PieceColour::Black, PieceType::Knight) => BLACK_KNIGHT,
                (PieceColour::Black, PieceType::Bishop) => BLACK_BISHOP,
                (PieceColour::Black, PieceType::Rook) => BLACK_ROOK,
                (PieceColour::Black, PieceType::Queen) => BLACK_QUEEN,
                (PieceColour::Black, PieceType::King) => BLACK_KING,
                // White pieces
                (PieceColour::White, PieceType::Pawn) => WHITE_PAWN,
                (PieceColour::White, PieceType::Knight) => WHITE_KNIGHT,
                (PieceColour::White, PieceType::Bishop) => WHITE_BISHOP,
                (PieceColour::White, PieceType::Rook) => WHITE_ROOK,
                (PieceColour::White, PieceType::Queen) => WHITE_QUEEN,
                (PieceColour::White, PieceType::King) => WHITE_KING,
            }
        )
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let row = ROWS.chars().nth(self.row() as usize).unwrap();
        let col = COLUMNS.chars().nth(self.col() as usize).unwrap();

        write!(f, "{col}{row}")
    }
}

impl Display for MoveQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MoveQualifier::Col(col) => COLUMNS.chars().nth(*col as usize).unwrap().to_string(),
                MoveQualifier::Row(row) => ROWS.chars().nth(*row as usize).unwrap().to_string(),
                MoveQualifier::Position(position) => position.to_string(),
            }
        )
    }
}

impl Display for Ply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ply = match self.movement() {
            PlyMovement::KingsideCastle { colour, check } => {
                format_castle(self.move_number(), check.as_ref(), *colour, "O-O")
            }
            PlyMovement::QueensideCastle { colour, check } => {
                format_castle(self.move_number(), check.as_ref(), *colour, "O-O-O")
            }
            PlyMovement::Move {
                movement,
                qualifier,
                check,
                capture,
            } => format_move(
                self.move_number(),
                movement,
                qualifier.as_ref(),
                check.as_ref(),
                *capture,
                None,
            ),
            PlyMovement::Promotion {
                movement,
                promotes_to,
                qualifier,
                check,
                capture,
            } => format_move(
                self.move_number(),
                movement,
                qualifier.as_ref(),
                check.as_ref(),
                *capture,
                Some(promotes_to),
            ),
        };

        write!(f, "{ply} ")
    }
}

fn format_castle(
    move_number: i16,
    check: Option<&Check>,
    colour: PieceColour,
    castle_string: &str,
) -> String {
    let move_number = move_number_string(colour, move_number);
    let check_string = check.map_or(String::new(), ToString::to_string);
    format!("{move_number}{castle_string}{check_string}")
}

fn format_move(
    move_number: i16,
    movement: &Movement,
    qualifier: Option<&MoveQualifier>,
    check: Option<&Check>,
    capture: bool,
    promotes_to: Option<&PieceType>,
) -> String {
    let move_number = move_number_string(*movement.piece().colour(), move_number);
    let qualifier_string = qualifier.map_or(String::new(), ToString::to_string);
    let capture_string = if capture { "x" } else { "" };
    let check_string = check.map_or(String::new(), ToString::to_string);
    let promotion_string = match promotes_to {
        None => String::new(),
        Some(&piece_type) => Piece::new(*movement.piece().colour(), piece_type).to_string(),
    };
    format!(
        "{move_number}{}{qualifier_string}{capture_string}{}{promotion_string}{check_string}",
        format_piece_for_ply(movement.piece()),
        movement.position(),
    )
}

fn move_number_string(colour: PieceColour, move_number: i16) -> String {
    match colour {
        PieceColour::White => format!("{move_number}."),
        PieceColour::Black => String::new(),
    }
}

fn format_piece_for_ply(piece: Piece) -> &'static str {
    match (piece.colour(), piece.piece_type()) {
        // Black pieces
        (PieceColour::Black, PieceType::Pawn) => "",
        (PieceColour::Black, PieceType::Knight) => BLACK_KNIGHT,
        (PieceColour::Black, PieceType::Bishop) => BLACK_BISHOP,
        (PieceColour::Black, PieceType::Rook) => BLACK_ROOK,
        (PieceColour::Black, PieceType::Queen) => BLACK_QUEEN,
        (PieceColour::Black, PieceType::King) => BLACK_KING,
        // White pieces
        (PieceColour::White, PieceType::Pawn) => "",
        (PieceColour::White, PieceType::Knight) => WHITE_KNIGHT,
        (PieceColour::White, PieceType::Bishop) => WHITE_BISHOP,
        (PieceColour::White, PieceType::Rook) => WHITE_ROOK,
        (PieceColour::White, PieceType::Queen) => WHITE_QUEEN,
        (PieceColour::White, PieceType::King) => WHITE_KING,
    }
}
