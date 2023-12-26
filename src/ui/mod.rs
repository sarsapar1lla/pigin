use crate::model::Game;

use self::{app::App, error::UiError};
use std::io::{self, Stdout};

use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use ratatui::prelude::*;

mod app;
mod board;
mod command;
mod error;
mod games;
mod ply;
mod tags;

pub fn launch(games: Vec<Game>) -> Result<(), UiError> {
    let terminal = setup_terminal()?;
    let mut app = App::new(terminal, games);
    app.launch()?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, UiError> {
    let mut stdout = io::stdout();
    enable_raw_mode().map_err(|e| UiError::new(format!("Failed to enable raw mode: {e}")))?;
    execute!(stdout, EnterAlternateScreen)
        .map_err(|e| UiError::new(format!("Failed to enter alternate screen: {e}")))?;
    Terminal::new(CrosstermBackend::new(stdout))
        .map_err(|e| UiError::new(format!("Failed to create terminal: {e}")))
}
