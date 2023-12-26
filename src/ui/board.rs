use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph},
    Frame,
};

use crate::model::{Board, PieceColour, Position, MAX_POSITION};

pub fn render(frame: &mut Frame, board: &Board, perspective: PieceColour, area: Rect) {
    let positions = |i: i8| {
        let row = match perspective {
            PieceColour::White => i,
            PieceColour::Black => MAX_POSITION - i,
        };

        (0..=MAX_POSITION).map(move |column| Position::new(row, column))
    };

    let board_text: Vec<Line> = (0..=MAX_POSITION)
        .map(positions)
        .map(|positions| {
            Line::from(
                positions
                    .map(|position| square(position, board))
                    .collect::<Vec<Span>>(),
            )
        })
        .collect();

    let vertical_padding = (area.height - 8) / 2;

    let paragraph = Paragraph::new(board_text)
        .alignment(Alignment::Center)
        .block(Block::default().padding(Padding::vertical(vertical_padding)));

    frame.render_widget(paragraph, area);
}

fn square(position: Position, board: &Board) -> Span {
    let maybe_piece = board.occupant(position);
    let text = maybe_piece.map_or("   ".to_string(), |piece| format!(" {piece} "));

    let colour = maybe_piece.map_or(Color::Black, |piece| match piece.colour() {
        PieceColour::White => Color::White,
        PieceColour::Black => Color::DarkGray,
    });

    let style = Style::default().fg(colour);

    if (position.row() + position.col()) % 2 == 0 {
        Span::styled(text, style.bg(Color::LightBlue))
    } else {
        Span::styled(text, style.bg(Color::LightRed))
    }
}
