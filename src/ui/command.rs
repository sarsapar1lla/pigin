use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use super::error::UiError;

const PREVIOUS_PLY_KEY: char = 'a';
const NEXT_PLY_KEY: char = 'd';
const PREVIOUS_GAME_KEY: char = 'w';
const NEXT_GAME_KEY: char = 's';
const FLIP_PERSPECTIVE_KEY: char = 'e';
const QUIT_KEY: char = 'q';

pub enum Command {
    PlyForwards,
    PlyBackwards,
    GameForwards,
    GameBackwards,
    FlipPerspective,
    Quit,
}

pub fn read() -> Result<Option<Command>, UiError> {
    let is_event = event::poll(Duration::from_millis(10))
        .map_err(|e| UiError::new(format!("Failed to poll for event: {e}")))?;
    if is_event {
        let event =
            event::read().map_err(|e| UiError::new(format!("Failed to read event: {e}")))?;
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char(PREVIOUS_PLY_KEY) => Ok(Some(Command::PlyBackwards)),
                KeyCode::Char(NEXT_PLY_KEY) => Ok(Some(Command::PlyForwards)),
                KeyCode::Char(PREVIOUS_GAME_KEY) => Ok(Some(Command::GameBackwards)),
                KeyCode::Char(NEXT_GAME_KEY) => Ok(Some(Command::GameForwards)),
                KeyCode::Char(FLIP_PERSPECTIVE_KEY) => Ok(Some(Command::FlipPerspective)),
                KeyCode::Char(QUIT_KEY) => Ok(Some(Command::Quit)),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
