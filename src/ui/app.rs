use std::io::Stdout;
use std::ops::Index;

use ratatui::layout::{Layout, Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::{prelude::CrosstermBackend, Frame, Terminal};

use crate::model::{Board, Game, PieceColour, Position, MAX_POSITION, Ply};

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
                        }
                    }
                    Command::GameBackwards => {
                        if self.current_game > 0 {
                            self.current_game -= 1;
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

            let game = &self.games[self.current_game];
            let current_ply = self.current_ply[self.current_game];

            self.terminal
                .draw(|frame| {
                    render(
                        frame,
                        self.current_game,
                        current_ply,
                        self.perspective,
                        game,
                    )
                })
                .map_err(|e| UiError::new(format!("Failed to draw frame: {e}")))?;
        }
        Ok(())
    }
}

fn render(
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    current_game: usize,
    current_ply: usize,
    perspective: PieceColour,
    game: &Game,
) {
    let regions = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .margin(frame.size().height / 5)
        .split(frame.size());

    render_ply(frame, game.pgn().ply(), current_ply, regions[0]);

    let board = &game.boards()[current_ply];
    render_board(frame, board, perspective, regions[1]);
}

fn render_ply(
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    ply: &[Ply],
    current_ply: usize,
    area: Rect
) {
    let spans: Vec<Span> = ply.iter().enumerate()
        .map(|(idx, p)| if idx == current_ply { highlighted_ply(p) } else { standard_ply(p) })
        .collect();

    let paragraph = Paragraph::new(vec![Line::from(spans)])
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(paragraph, area);
}

fn standard_ply(ply: &Ply) -> Span {
    Span::styled(format!("{ply}"), Style::default().fg(Color::White))
}

fn highlighted_ply(ply: &Ply) -> Span {
    Span::styled(format!("{ply}"), Style::default().fg(Color::Yellow))
}

fn render_board(
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    board: &Board,
    perspective: PieceColour,
    area: Rect
) {
    let positions = |i: i8| {
        let row = match perspective {
            PieceColour::White => i,
            PieceColour::Black => MAX_POSITION - i,
        };

        (0..MAX_POSITION).map(move |column| Position::new(row, column))
    };

    let text: Vec<Line> = (0..=MAX_POSITION)
        .map(|i| positions(i))
        .map(|positions| {
            Line::from(
                positions
                    .map(|position| square(position, board))
                    .collect::<Vec<Span>>(),
            )
        })
        .collect();

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL));
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

    if (position.row() + position.col()) % 2 == 0 {
        Span::styled(text, Style::default().bg(Color::LightBlue).fg(colour))
    } else {
        Span::styled(text, Style::default().bg(Color::LightRed).fg(colour))
    }
}
