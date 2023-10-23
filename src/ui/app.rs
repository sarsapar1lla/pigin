use std::borrow::Cow;
use std::io::Stdout;

use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::{prelude::CrosstermBackend, Frame, Terminal};

use crate::model::{Board, Game, PieceColour, PieceType, Position, MAX_POSITION, MIN_POSITION};

use super::{command::Command, error::UiError};

use super::command;
use super::piece::{Pawn, Rook};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use ratatui::widgets::*;

use ratatui::widgets::canvas::{Canvas, Rectangle, Shape};

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
    let white_player = game.pgn().tags().get("White").unwrap();
    let greeting = Paragraph::new(format!("Current game: {current_game}, current ply: {current_ply}, white player: {white_player}, perspective: {perspective:?}"));
    frame.render_widget(greeting, frame.size());

    let board = &game.boards()[current_ply];
    render_board(frame, board, perspective);
}

fn render_board(
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    board: &Board,
    perspective: PieceColour,
) {
    let board_canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL))
        .x_bounds([-5.0, 85.0])
        .y_bounds([-5.0, 85.0])
        .paint(|ctx| {
            for i in 0..=MAX_POSITION {
                for j in 0..=MAX_POSITION {
                    let position = Position::new(i, j);

                    // if i == MIN_POSITION {
                    //     ctx.print(
                    //         (j as f64 * 10.0) + 5.0,
                    //         (i as f64 * 10.0) - 5.0,
                    //         Line {
                    //             spans: vec![Span {
                    //                 content: Cow::Borrowed("a"),
                    //                 style: Style::default().add_modifier(Modifier::ITALIC),
                    //             }],
                    //             alignment: Some(ratatui::prelude::Alignment::Center),
                    //         },
                    //     )
                    // }

                    if let Some(piece) = board.occupant(position) {
                        let x = match perspective {
                            PieceColour::White => j,
                            PieceColour::Black => j
                        };
                        let y = match perspective {
                            PieceColour::White => i,
                            PieceColour::Black => MAX_POSITION - i
                        };
                        let colour = match piece.colour() {
                            PieceColour::White => ratatui::style::Color::White,
                            PieceColour::Black => ratatui::style::Color::DarkGray,
                        };
                        match piece.piece_type() {
                            PieceType::Pawn => {
                                ctx.draw(&Pawn {
                                    x: (x as f64 * 10.0) + 2.0,
                                    y: (y as f64 * 10.0) + 2.0,
                                    width: 6.0,
                                    colour,
                                });
                            }
                            PieceType::Rook => {
                                ctx.draw(&Rook {
                                    x: (x as f64 * 10.0) + 2.0,
                                    y: (y as f64 * 10.0) + 2.0,
                                    width: 6.0,
                                    colour,
                                });
                            }
                            _ => {
                                ctx.draw(&Pawn {
                                    x: (x as f64 * 10.0) + 2.0,
                                    y: (y as f64 * 10.0) + 2.0,
                                    width: 6.0,
                                    colour,
                                });
                            }
                        };
                    }

                    let square = Rectangle {
                        x: j as f64 * 10.0,
                        y: i as f64 * 10.0,
                        width: 10.0,
                        height: 10.0,
                        color: ratatui::style::Color::White,
                    };
                    ctx.draw(&square);
                }
            }
        });

    frame.render_widget(board_canvas, frame.size());
}
