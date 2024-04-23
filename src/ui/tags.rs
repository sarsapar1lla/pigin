use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Clear, Row, Table},
    Frame,
};

use crate::model::{GameResult, Tags};

use super::centre;

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

    let table = Table::new(
        rows,
        &[Constraint::Percentage(20), Constraint::Percentage(80)],
    )
    .header(header)
    .style(Style::default().fg(Color::White))
    .block(Block::default().borders(Borders::ALL));

    let area = centre::centered_rect(70, 100, area);

    frame.render_widget(Clear, area);
    frame.render_widget(table, area);
}
