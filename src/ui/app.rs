use std::io::Stdout;

use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::block::Title;
use ratatui::{prelude::CrosstermBackend, Frame, Terminal};

use crate::model::{Board, Game, GameResult, PieceColour, Ply, Position, Tags, MAX_POSITION};

use super::{command::Command, error::UiError};

use super::command;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use ratatui::widgets::*;

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    games: Vec<Game>,
    current_game: usize,
    current_ply: Vec<usize>,
    max_ply: Vec<usize>,
    perspective: PieceColour,
    scroll_state: ScrollbarState,
}

impl App {
    pub fn new(terminal: Terminal<CrosstermBackend<Stdout>>, games: Vec<Game>) -> Self {
        let current_ply = games.iter().map(|_| 0).collect();
        let max_ply = games.iter().map(|g| g.boards().len() - 1).collect();
        App {
            terminal,
            games,
            current_game: 0,
            current_ply,
            max_ply,
            perspective: PieceColour::White,
            scroll_state: ScrollbarState::default(),
        }
    }

    pub fn launch(&mut self) -> Result<(), UiError> {
        self.run()?;
        self.restore_terminal()?;
        Ok(())
    }

    fn restore_terminal(&mut self) -> Result<(), UiError> {
        disable_raw_mode().map_err(|e| UiError::new(format!("Failed to disable raw mode: {e}")))?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)
            .map_err(|e| UiError::new(format!("Failed to leave alternate screen: {e}")))?;
        self.terminal
            .show_cursor()
            .map_err(|e| UiError::new(format!("Failed to show cursor: {e}")))?;
        Ok(())
    }

    fn run(&mut self) -> Result<(), UiError> {
        loop {
            if let Some(command) = command::read()? {
                match command {
                    Command::PlyForwards => {
                        let current_ply = self.current_ply[self.current_game];
                        if current_ply < self.max_ply[self.current_game] {
                            self.current_ply[self.current_game] += 1;
                        }
                    }
                    Command::PlyBackwards => {
                        let current_ply = self.current_ply[self.current_game];
                        if current_ply > 0 {
                            self.current_ply[self.current_game] -= 1;
                        }
                    }
                    Command::FirstPly => {
                        self.current_ply[self.current_game] = 0;
                    }
                    Command::LastPly => {
                        self.current_ply[self.current_game] = self.max_ply[self.current_game];
                    }
                    Command::GameForwards => {
                        if self.current_game < self.games.len() - 1 {
                            self.current_game += 1;
                            self.scroll_state.next();
                        }
                    }
                    Command::GameBackwards => {
                        if self.current_game > 0 {
                            self.current_game -= 1;
                            self.scroll_state.prev();
                        }
                    }
                    Command::FlipPerspective => match self.perspective {
                        PieceColour::White => {
                            self.perspective = PieceColour::Black;
                        }
                        PieceColour::Black => {
                            self.perspective = PieceColour::White;
                        }
                    },
                    Command::Quit => break,
                }
            }

            let current_ply = self.current_ply[self.current_game];

            self.terminal
                .draw(|frame| {
                    render(
                        frame,
                        self.current_game,
                        current_ply,
                        self.perspective,
                        &self.games,
                        &mut self.scroll_state,
                    )
                })
                .map_err(|e| UiError::new(format!("Failed to draw frame: {e}")))?;
        }
        Ok(())
    }
}

fn render(
    frame: &mut Frame,
    current_game: usize,
    current_ply: usize,
    perspective: PieceColour,
    games: &[Game],
    scrollbar_state: &mut ScrollbarState,
) {
    let regions = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Percentage(95), Constraint::Percentage(5)])
        // .margin(frame.size().height / 10)
        .split(frame.size());

    let ui_regions = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(regions[0]);

    let left_region = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(ui_regions[0]);

    let right_region = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(ui_regions[1]);

    render_ply(
        frame,
        games[current_game].pgn().ply(),
        current_ply,
        &games[current_game].pgn().result(),
        left_region[0],
    );

    render_games(frame, games, current_game, left_region[1], scrollbar_state);

    let board = &games[current_game].boards()[current_ply];
    render_board(frame, board, perspective, right_region[0]);

    render_metadata(
        frame,
        games[current_game].pgn().tags(),
        &games[current_game].pgn().result(),
        right_region[1],
    );

    command::render(frame, regions[1]);
}

fn render_ply(
    frame: &mut Frame,
    ply: &[Ply],
    current_ply: usize,
    game_result: &GameResult,
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

    spans.push(standard_game_result(game_result));

    let paragraph = Paragraph::new(vec![Line::from(spans)])
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL));

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

fn render_games(
    frame: &mut Frame,
    games: &[Game],
    current_game: usize,
    area: Rect,
    scrollbar_state: &mut ScrollbarState,
) {
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

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL))
        .scroll((current_game as u16, 0));

    let scrollbar = Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight);

    frame.render_widget(paragraph, area);
    frame.render_stateful_widget(scrollbar, area, scrollbar_state);
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
        spans.push(Span::styled(format!(" | {event}"), style))
    }

    if let Some(date) = date {
        spans.push(Span::styled(format!(" | {date}"), style));
    }

    spans
}

fn render_board(frame: &mut Frame, board: &Board, perspective: PieceColour, area: Rect) {
    let positions = |i: i8| {
        let row = match perspective {
            PieceColour::White => i,
            PieceColour::Black => MAX_POSITION - i,
        };

        (0..MAX_POSITION).map(move |column| Position::new(row, column))
    };

    let text: Vec<Line> = (0..=MAX_POSITION)
        .map(positions)
        .map(|positions| {
            Line::from(
                positions
                    .map(|position| square(position, board))
                    .collect::<Vec<Span>>(),
            )
        })
        .collect();

    let paragraph = Paragraph::new(text).alignment(Alignment::Center).block(
        Block::default()
            .title(Title::from(format!("Perspective: {perspective}")))
            .borders(Borders::ALL)
            .padding(Padding::vertical(1)),
    );

    frame.render_widget(paragraph, area);
}

fn square(position: Position, board: &Board) -> Span {
    let maybe_piece = board.occupant(position);
    let text = maybe_piece
        .map(|piece| format!(" {piece} "))
        .unwrap_or("   ".to_string());

    let colour = maybe_piece
        .map(|piece| match piece.colour() {
            PieceColour::White => Color::White,
            PieceColour::Black => Color::DarkGray,
        })
        .unwrap_or(Color::Black);

    let style = Style::default().fg(colour);

    if (position.row() + position.col()) % 2 == 0 {
        Span::styled(text, style.bg(Color::LightBlue))
    } else {
        Span::styled(text, style.bg(Color::LightRed))
    }
}

fn render_metadata(frame: &mut Frame, tags: &Tags, result: &GameResult, area: Rect) {
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
                .map(|value| Row::new([Cell::from(tag.to_owned()), Cell::from(value.to_owned())]))
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

impl std::fmt::Display for PieceColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PieceColour::White => "White",
                PieceColour::Black => "Black",
            }
        )
    }
}
