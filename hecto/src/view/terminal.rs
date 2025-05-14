use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Attribute, Print};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, DisableLineWrap, EnableLineWrap,
    EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
};
use crossterm::{queue, Command};
use std::io::{stdout, Error, Write};

#[derive(Default, Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Position {
    pub const fn saturating_sub(&self, other: Self) -> Self {
        Self {
            col: self.col.saturating_sub(other.col),
            row: self.row.saturating_sub(other.row),
        }
    }
}

pub struct Terminal {}

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::enable_line_wrap()?;
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::disable_line_wrap()?;
        Self::clear_screen()?;
        Self::execute()
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))
    }

    pub fn cleat_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)
    }

    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))
    }

    pub fn get_size() -> Result<Size, Error> {
        let (width, height) = size()?;

        Ok(Size {
            #[allow(clippy::as_conversions)]
            width: width as usize,
            #[allow(clippy::as_conversions)]
            height: height as usize,
        })
    }

    pub fn execute() -> Result<(), Error> {
        let mut stdout = stdout();
        stdout.flush()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)
    }

    pub fn print_now(row: usize, line_text: &str) -> Result<(), Error> {
        Self::move_caret_to(Position { row, col: 0 })?;
        Self::cleat_line()?;
        Self::print(line_text)
    }

    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)
    }

    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)
    }

    pub fn disable_line_wrap() -> Result<(), Error> {
        Self::queue_command(DisableLineWrap)
    }

    pub fn enable_line_wrap() -> Result<(), Error> {
        Self::queue_command(EnableLineWrap)
    }

    pub fn set_title(title: &str) -> Result<(), Error> {
        Self::queue_command(SetTitle(title))
    }

    pub fn print_inverted_row(row: usize, line_text: &str) -> Result<(), Error> {
        let width = Self::get_size()?.width;
        Self::print_now(
            row,
            &format!(
                "{}{:width$.width$}{}",
                Attribute::Reverse,
                line_text,
                Attribute::Reset
            ),
        )
    }
}
