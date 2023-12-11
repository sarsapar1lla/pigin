use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Clear, Row, Table},
    Frame,
};

use crate::model::{GameResult, Tags};

pub fn render(frame: &mut Frame, tags: &Tags, result: GameResult, area: Rect) {
    let header_cells = ["Tag", "Value"].iter().map(|header| Cell::from(*header));

    let header = Row::new(header_cells)
        .style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .height(1);

    let mut rows: Vec<Row> = ["Event", "Round", "Date", "Site", "WhiteElo", "BlackElo"]
        .iter()
        .filter_map(|&tag| {
            tags.get(tag)
                .map(|value| Row::new([Cell::from(tag.to_owned()), Cell::from(value.clone())]))
        })
        .collect();

    rows.push(Row::new([
        Cell::from("Result"),
        Cell::from(format!("{result}")),
    ]));

    let table = Table::new(rows)
        .header(header)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL))
        .widths(&[Constraint::Percentage(20), Constraint::Percentage(80)]);

    let area = centered_rect(70, 100, area);

    frame.render_widget(Clear, area);
    frame.render_widget(table, area);
}

// Lifted from the ratatui examples: https://github.com/ratatui-org/ratatui/blob/main/examples/popup.rs
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
