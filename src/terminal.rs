use crossterm::{
	cursor::{Hide, MoveTo, Show},
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
	pub fn clear_screen() {
		execute!(stdout(), Clear(ClearType::All)).ok();
	}
	pub fn cursor_position(x: u16, y: u16) {
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
}
