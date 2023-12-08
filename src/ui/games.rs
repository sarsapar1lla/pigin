use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use crate::Game;

use super::error::UiError;

pub fn render(
    frame: &mut Frame,
    games: &[Game],
    current_game: usize,
    area: Rect,
    scrollbar_state: &mut ScrollbarState,
    dim: bool,
) -> Result<(), UiError> {
    let lines: Vec<Line> = games
        .iter()
        .enumerate()
        .map(|(idx, game)| {
            if idx == current_game {
                highlighted_game(game)
            } else {
                standard_game(game)
            }
        })
        .collect();

    scrollbar_state.content_length(lines.len());

    let scroll_position = u16::try_from(current_game)
        .map_err(|e| UiError::new(format!("Failed to convert scroll position: {e}")))?;

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL))
        .style(if dim { Style::default().dim() } else { Style::default() })
        .scroll((scroll_position, 0));

    let scrollbar = Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight);

    frame.render_widget(paragraph, area);
    frame.render_stateful_widget(scrollbar, area, scrollbar_state);
    Ok(())
}

pub fn render_error_message(frame: &mut Frame, area: Rect) {
    let paragraph = Paragraph::new(vec![Line::from("Oops! Something went wrong! 😔")])
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn highlighted_game(game: &Game) -> Line {
    Line::from(game_description(game, true))
}

fn standard_game(game: &Game) -> Line {
    Line::from(game_description(game, false))
}

fn game_description(game: &Game, highlighted: bool) -> Vec<Span> {
    let tags = game.pgn().tags();

    let white_player = tags.get_or_default("White", "Unknown");
    let black_player = tags.get_or_default("Black", "Unknown");

    let event = tags.get("Event");
    let date = tags.get("Date");

    let style = if highlighted {
        Style::default().add_modifier(Modifier::UNDERLINED)
    } else {
        Style::default()
    };

    let mut spans = vec![
        Span::styled(
            white_player,
            style.fg(Color::White).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" vs. ", style.add_modifier(Modifier::ITALIC)),
        Span::styled(
            black_player,
            style.fg(Color::DarkGray).add_modifier(Modifier::BOLD),
        ),
    ];

    if let Some(event) = event {
        spans.push(Span::styled(format!(" | {event}"), style));
    }

    if let Some(date) = date {
        spans.push(Span::styled(format!(" | {date}"), style));
    }

    spans
}
