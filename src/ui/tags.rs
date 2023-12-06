use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::model::{GameResult, Tags};

pub fn render(frame: &mut Frame, tags: &Tags, result: GameResult, area: Rect) {
    let header_cells = ["Tag", "Value"].iter().map(|header| Cell::from(*header));

    let header = Row::new(header_cells)
        .style(
            Style::default()
                .fg(Color::DarkGray)
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .height(1);

    let mut rows: Vec<Row> = ["Event", "Round", "Site", "WhiteElo", "BlackElo"]
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
        .widths(&[Constraint::Percentage(30), Constraint::Percentage(70)]);

    frame.render_widget(table, area);
}
