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
const FIRST_PLY_KEY: char = 'z';
const LAST_PLY_KEY: char = 'x';
const PREVIOUS_GAME_KEY: char = 'w';
const NEXT_GAME_KEY: char = 's';
const FLIP_PERSPECTIVE_KEY: char = 'e';
const QUIT_KEY: char = 'q';

pub enum Command {
    PlyForwards,
    PlyBackwards,
    FirstPly,
    LastPly,
    GameForwards,
    GameBackwards,
    FlipPerspective,
    Quit,
}

pub fn read() -> Result<Option<Command>, UiError> {
    let event = event::read().map_err(|e| UiError::new(format!("Failed to read event: {e}")))?;
    if let Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char(PREVIOUS_PLY_KEY) => Ok(Some(Command::PlyBackwards)),
                KeyCode::Char(NEXT_PLY_KEY) => Ok(Some(Command::PlyForwards)),
                KeyCode::Char(FIRST_PLY_KEY) => Ok(Some(Command::FirstPly)),
                KeyCode::Char(LAST_PLY_KEY) => Ok(Some(Command::LastPly)),
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

pub fn render(frame: &mut Frame, area: Rect) {
    let title: Vec<Span> = [
        command("Ply →", NEXT_PLY_KEY, Color::LightGreen),
        command("Ply ←", PREVIOUS_PLY_KEY, Color::LightBlue),
        command("Start ↩", FIRST_PLY_KEY, Color::LightGreen),
        command("End ↪", LAST_PLY_KEY, Color::LightBlue),
        command("Game →", NEXT_GAME_KEY, Color::LightGreen),
        command("Game ←", PREVIOUS_GAME_KEY, Color::LightBlue),
        command("Board ↶", FLIP_PERSPECTIVE_KEY, Color::LightGreen),
        command("Quit", QUIT_KEY, Color::LightBlue),
    ]
    .concat();

    let commands = Block::default()
        .borders(Borders::TOP)
        .title(Title::from(title))
        .title_alignment(ratatui::layout::Alignment::Left);

    frame.render_widget(commands, area);
}

fn command(command_name: &str, command_key: char, background_colour: Color) -> [Span; 2] {
    let style = Style::default().bg(background_colour).fg(Color::Black);
    [
        Span::styled(format!(" {command_name}: {command_key} "), style),
        Span::from(" "),
    ]
}
