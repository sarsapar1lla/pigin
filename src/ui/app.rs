use std::io::Stdout;

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::block::Title;
use ratatui::{prelude::CrosstermBackend, Frame, Terminal};

use crate::model::{Board, Game, PieceColour};

use super::{command::Command, error::UiError};

use super::{board, centre, command, fen, games, ply, tags};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use ratatui::widgets::{Block, Borders, Clear, ListState, Padding, Paragraph};

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    games: Vec<Game>,
    current_game: usize,
    current_ply: Vec<usize>,
    max_ply: Vec<usize>,
    perspective: PieceColour,
    list_state: ListState,
    show_metadata: bool,
    display_fen: bool,
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
            list_state: ListState::default().with_selected(Some(0)),
            show_metadata: false,
            display_fen: false,
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
                    Command::GameForwards => {
                        if self.current_game < self.games.len() - 1 {
                            self.current_game += 1;
                            self.list_state.select(Some(self.current_game));
                        }
                    }
                    Command::GameBackwards => {
                        if self.current_game > 0 {
                            self.current_game -= 1;
                            self.list_state.select(Some(self.current_game));
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
                    Command::ToggleMetadata => {
                        self.show_metadata = !self.show_metadata;
                    }
                    Command::DisplayFen => {
                        self.display_fen = !self.display_fen;
                    }
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
                        &mut self.list_state,
                        self.show_metadata,
                        self.display_fen,
                    );
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
    list_state: &mut ListState,
    show_metadata: bool,
    display_fen: bool,
) {
    let regions = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Percentage(5),
        ])
        .split(frame.area());

    let ui_regions = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(regions[1]);

    let top_region = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(ui_regions[0]);

    let bottom_region = ui_regions[1];

    title(frame, regions[0]);

    let pgn = games[current_game].pgn();

    ply::render(frame, pgn.ply(), current_ply, pgn.result(), top_region[0]);

    let current_board = &games[current_game].boards()[current_ply];
    if display_fen {
        fen_string(frame, regions[1], current_board);
    }
    board::render(frame, current_board, perspective, top_region[1]);

    games::render(frame, games, bottom_region, list_state, show_metadata);

    if show_metadata {
        tags::render(frame, pgn.tags(), pgn.result(), bottom_region);
    }

    command::render(frame, regions[2]);
}

fn title(frame: &mut Frame, area: Rect) {
    let title: Vec<Span> = [
        Span::styled("pigin", Style::default().add_modifier(Modifier::ITALIC)),
        Span::from(" "),
    ]
    .into_iter()
    .collect();

    let title_block = Block::default()
        .borders(Borders::BOTTOM)
        .title(Title::from(Line::from(title)))
        .title_alignment(ratatui::layout::Alignment::Left);

    frame.render_widget(title_block, area);
}

fn fen_string(frame: &mut Frame, area: Rect, board: &Board) {
    let fen_string = fen::from_board(board)
        .unwrap_or_else(|_| String::from("Failed to generate FEN string for board"));

    let fen_area = centre::centered_rect(80, 20, area);
    let span = Span::styled(fen_string, Style::new().add_modifier(Modifier::ITALIC));
    let padding = Padding::new(0, 0, ((fen_area.height - 1) / 2) - 1, 0);
    let block = Paragraph::new(span)
        .block(
            Block::default()
                .title("FEN string")
                .title_alignment(ratatui::layout::Alignment::Left)
                .padding(padding)
                .borders(Borders::ALL),
        )
        .alignment(ratatui::prelude::Alignment::Center);

    frame.render_widget(Clear, fen_area);
    frame.render_widget(block, fen_area);
}
