use std::fmt::Display;

use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        block::{self, Title},
        Block, Borders, Padding, Paragraph, Wrap,
    },
    Frame,
};

use crate::model::{
    Check, GameResult, MoveQualifier, Movement, Piece, PieceColour, PieceType, Ply, PlyMovement,
    Position, COLUMNS, ROWS,
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

const AVERAGE_PLY_LENGTH: u16 = 8;

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
        let row = usize::try_from(self.row())
            .ok()
            .and_then(|row| ROWS.chars().nth(row))
            .ok_or(std::fmt::Error)?;
        let col = usize::try_from(self.col())
            .ok()
            .and_then(|col| COLUMNS.chars().nth(col))
            .ok_or(std::fmt::Error)?;

        write!(f, "{col}{row}")
    }
}

impl Display for MoveQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            MoveQualifier::Col(col) => {
                let col = usize::try_from(*col).map_err(|_| std::fmt::Error)?;
                COLUMNS.chars().nth(col).ok_or(std::fmt::Error)?.to_string()
            }
            MoveQualifier::Row(row) => {
                let row = usize::try_from(*row).map_err(|_| std::fmt::Error)?;
                ROWS.chars().nth(row).ok_or(std::fmt::Error)?.to_string()
            }
            MoveQualifier::Position(position) => position.to_string(),
        };
        write!(f, "{text}")
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

impl std::fmt::Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameResult::WhiteWin => "1-0",
                GameResult::BlackWin => "0-1",
                GameResult::Draw => "½-½",
                GameResult::Ongoing => "*",
            }
        )
    }
}

pub fn render(
    frame: &mut Frame,
    ply: &[Ply],
    current_ply: usize,
    game_result: GameResult,
    area: Rect,
) {
    let mut spans: Vec<Span> = ply
        .iter()
        .enumerate()
        .map(|(idx, p)| {
            if idx == current_ply {
                highlighted_ply(p)
            } else {
                standard_ply(p)
            }
        })
        .collect();

    spans.push(standard_game_result(&game_result));

    let spans_per_page: usize = (area.area() / AVERAGE_PLY_LENGTH).into();
    let current_page = current_ply / spans_per_page;

    let pages = (spans.len() / spans_per_page) + 1;
    let page = spans
        .chunks(spans_per_page)
        .nth(current_page)
        .unwrap()
        .to_vec();

    let title = Title::from(format!("Page {}/{}", current_page + 1, pages))
        .position(block::Position::Bottom)
        .alignment(Alignment::Right);

    let paragraph = Paragraph::new(vec![Line::from(page)])
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title(if pages > 1 { title } else { Title::default() })
                .borders(Borders::RIGHT)
                .padding(Padding::horizontal(1)),
        );

    frame.render_widget(paragraph, area);
}

fn standard_ply(ply: &Ply) -> Span {
    Span::styled(format!("{ply}"), Style::default().fg(Color::DarkGray))
}

fn highlighted_ply(ply: &Ply) -> Span {
    Span::styled(format!("{ply}"), Style::default().fg(Color::Yellow))
}

fn standard_game_result(game_result: &GameResult) -> Span {
    Span::styled(
        format!("{game_result}"),
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::LightGreen),
    )
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
        Some(&piece_type) => format!("={}", Piece::new(*movement.piece().colour(), piece_type)),
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
        (_, PieceType::Pawn) => "",
        // Black pieces
        (PieceColour::Black, PieceType::Knight) => BLACK_KNIGHT,
        (PieceColour::Black, PieceType::Bishop) => BLACK_BISHOP,
        (PieceColour::Black, PieceType::Rook) => BLACK_ROOK,
        (PieceColour::Black, PieceType::Queen) => BLACK_QUEEN,
        (PieceColour::Black, PieceType::King) => BLACK_KING,
        // White pieces
        (PieceColour::White, PieceType::Knight) => WHITE_KNIGHT,
        (PieceColour::White, PieceType::Bishop) => WHITE_BISHOP,
        (PieceColour::White, PieceType::Rook) => WHITE_ROOK,
        (PieceColour::White, PieceType::Queen) => WHITE_QUEEN,
        (PieceColour::White, PieceType::King) => WHITE_KING,
    }
}
