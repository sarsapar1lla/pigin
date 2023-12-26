use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{block::Title, Block, Borders},
    Frame,
};

use super::error::UiError;

const PREVIOUS_PLY_KEY: char = 'a';
const NEXT_PLY_KEY: char = 'd';
const PREVIOUS_GAME_KEY: char = 'w';
const NEXT_GAME_KEY: char = 's';
const FLIP_PERSPECTIVE_KEY: char = 'e';
const METADATA_KEY: char = 'x';
const QUIT_KEY: char = 'q';

const NAVIGATE_LABEL: &str = " Navigate: w a s d ";
const FLIP_LABEL: &str = " Flip: e ";
const METADATA_LABEL: &str = " Toggle metadata: x ";
const QUIT_LABEL: &str = " Quit: q ";

pub enum Command {
    PlyForwards,
    PlyBackwards,
    GameForwards,
    GameBackwards,
    FlipPerspective,
    ToggleMetadata,
    Quit,
}

pub fn read() -> Result<Option<Command>, UiError> {
    let event = event::read().map_err(|e| UiError::new(format!("Failed to read event: {e}")))?;
    if let Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char(PREVIOUS_PLY_KEY) => Ok(Some(Command::PlyBackwards)),
                KeyCode::Char(NEXT_PLY_KEY) => Ok(Some(Command::PlyForwards)),
                KeyCode::Char(PREVIOUS_GAME_KEY) => Ok(Some(Command::GameBackwards)),
                KeyCode::Char(NEXT_GAME_KEY) => Ok(Some(Command::GameForwards)),
                KeyCode::Char(FLIP_PERSPECTIVE_KEY) => Ok(Some(Command::FlipPerspective)),
                KeyCode::Char(METADATA_KEY) => Ok(Some(Command::ToggleMetadata)),
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

pub fn render(frame: &mut Frame, area: Rect) {
    let title: Vec<Span> = [
        command(NAVIGATE_LABEL, Color::LightGreen),
        command(FLIP_LABEL, Color::LightBlue),
        command(METADATA_LABEL, Color::LightGreen),
        command(QUIT_LABEL, Color::LightBlue),
    ]
    .concat();

    let commands = Block::default()
        .borders(Borders::TOP)
        .title(Title::from(title))
        .title_alignment(ratatui::layout::Alignment::Left);

    frame.render_widget(commands, area);
}

fn command(label: &str, background_colour: Color) -> [Span; 2] {
    [
        Span::styled(
            label,
            Style::default().bg(background_colour).fg(Color::Black),
        ),
        Span::from(" "),
    ]
}
