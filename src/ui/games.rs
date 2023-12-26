use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState},
    Frame,
};

use crate::Game;

pub fn render(
    frame: &mut Frame,
    games: &[Game],
    area: Rect,
    list_state: &mut ListState,
    dim: bool,
) {
    let list_items: Vec<ListItem> = games
        .iter()
        .map(|game| ListItem::new(Line::from(game_description(game))))
        .collect();

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::TOP.union(Borders::BOTTOM)))
        .style(if dim {
            Style::default().dim()
        } else {
            Style::default()
        })
        // .highlight_style(Style::default().bg(Color::Yellow))
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol(">>");

    frame.render_stateful_widget(list, area, list_state);
}

fn game_description(game: &Game) -> Vec<Span> {
    let tags = game.pgn().tags();

    let white_player = tags.get_or_default("White", "Unknown");
    let black_player = tags.get_or_default("Black", "Unknown");

    let event = tags.get("Event");
    let date = tags.get("Date");

    let style = Style::default();

    let mut spans = vec![
        Span::from(" "),
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
