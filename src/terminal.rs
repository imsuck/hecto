use std::io::{stdout, Write};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyEvent};
use crossterm::style::{self, Color};
use crossterm::terminal::{enable_raw_mode, Clear, ClearType};
use crossterm::{execute, Result};

use crate::Position;

pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> crossterm::Result<Self> {
        let size = crossterm::terminal::size()?;
        enable_raw_mode()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn update_size(&mut self) {
        // TODO: Handle this?
        let size = crossterm::terminal::size().unwrap();
        self.size.width = size.0;
        self.size.height = size.1.saturating_sub(2);
    }

    pub fn clear_screen() {
        execute!(stdout(), Clear(ClearType::All)).ok();
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        let Position { x, y } = position;
        let x = *x as u16;
        let y = *y as u16;
        execute!(stdout(), MoveTo(x, y)).ok();
    }

    pub fn flush() -> Result<()> {
        stdout().flush()
    }

    pub fn read_key() -> Result<KeyEvent> {
        loop {
            if let Event::Key(event) = event::read()? {
                return Ok(event);
            }
        }
    }

    pub fn cursor_hide() {
        execute!(stdout(), Hide).ok();
    }

    pub fn cursor_show() {
        execute!(stdout(), Show).ok();
    }

    pub fn clear_current_line() {
        execute!(stdout(), Clear(ClearType::CurrentLine)).ok();
    }

    pub fn set_bg_color(color: Color) {
        execute!(stdout(), style::SetBackgroundColor(color)).ok();
    }

    pub fn reset_color() {
        execute!(stdout(), style::ResetColor).ok();
    }

    pub fn set_fg_color(color: Color) {
        execute!(stdout(), style::SetForegroundColor(color)).ok();
    }
}
