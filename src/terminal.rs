use crossterm::{
	cursor::MoveTo,
	event::{self, Event, KeyEvent},
	execute,
	terminal::{enable_raw_mode, Clear, ClearType},
	Result,
};
use std::io::{stdout, Write};

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
				height: size.1,
			},
		})
	}
	pub fn size(&self) -> &Size {
		&self.size
	}
	pub fn clear_screen() -> Result<()> {
		execute!(stdout(), Clear(ClearType::All))
	}
	pub fn cursor_position(x: u16, y: u16) -> Result<()> {
		execute!(stdout(), MoveTo(x, y))
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
}
