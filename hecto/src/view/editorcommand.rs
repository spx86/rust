use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::convert::TryFrom;

use super::terminal::Size;

pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Down,
    Left,
    Right,
}

pub enum EditorCommand {
    Move(Direction),
    Quit,
    Resize(Size),
    Insert(char),
    Backspace,
    Delete,
    Enter,
}

#[allow(clippy::as_conversions)]
impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('w'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                    Ok(Self::Insert(character))
                }
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::Backspace, _) => Ok(Self::Backspace),
                (KeyCode::Delete, _) => Ok(Self::Delete),
                (KeyCode::Enter, _) => Ok(Self::Enter),
                (KeyCode::Tab, _) => Ok(Self::Insert('\t')),
                _ => Err(format!("Key Code not supported: {code:?}")),
            },
            Event::Resize(width_u16, height_u16) => {
                let height = height_u16 as usize;

                let width = width_u16 as usize;
                Ok(Self::Resize(Size { width, height }))
            }
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
