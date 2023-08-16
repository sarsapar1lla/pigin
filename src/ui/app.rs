use std::{collections::HashMap, io::Stdout};

use ratatui::widgets::canvas::Canvas;
use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::model::{Game, PieceColour};

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
    current_ply: HashMap<usize, u16>,
    perspective: PieceColour,
}

impl App {
    pub fn new(terminal: Terminal<CrosstermBackend<Stdout>>, games: Vec<Game>) -> Self {
        let current_ply: HashMap<usize, u16> = (0..games.len()).map(|index| (index, 0)).collect();
        App {
            terminal,
            games,
            current_game: 0,
            current_ply,
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
                    Command::PlyForwards => match self.current_ply.get(&self.current_game) {
                        Some(current_ply) => {
                            self.current_ply.insert(self.current_game, current_ply + 1);
                        }
                        None => {
                            self.current_ply.insert(self.current_game, 1);
                        }
                    },
                    Command::PlyBackwards => match self.current_ply.get(&self.current_game) {
                        Some(current_ply) => {
                            self.current_ply.insert(self.current_game, current_ply - 1);
                        }
                        None => {
                            self.current_ply.insert(self.current_game, 0);
                        }
                    },
                    Command::GameForwards => {
                        self.current_game += 1;
                    }
                    Command::GameBackwards => {
                        self.current_game -= 1;
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

            self.terminal
                .draw(|frame| render(frame))
                .map_err(|e| UiError::new(format!("Failed to draw frame: {e}")))?;
        }
        Ok(())
    }
}

fn render(frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.size());
}
