use crossterm::{
	event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
	execute,
	terminal::{enable_raw_mode, Clear, ClearType},
	Result,
};
use std::io::{stdout, Write};

pub struct Editor {
	should_quit: bool,
}

impl Editor {
	pub fn run(&mut self) -> Result<()> {
		enable_raw_mode()?;

		loop {
			self.refresh_screen()?;

			if self.should_quit {
				return Ok(());
			}

			self.process_keypress()?;
		}
	}
	pub fn default() -> Self {
		Self { should_quit: false }
	}
	fn refresh_screen(&self) -> Result<()> {
		execute!(stdout(), Clear(ClearType::All))?;

		stdout().flush()
	}
	fn process_keypress(&mut self) -> Result<()> {
		use KeyCode::{Char, Esc};

		let pressed_key = read_key()?;
		match (pressed_key.modifiers, pressed_key.code) {
			(KeyModifiers::CONTROL, Char('q')) | (_, Esc) => self.should_quit = true,
			_ => (),
		}

		Ok(())
	}
}

fn read_key() -> Result<KeyEvent> {
	loop {
		if let Event::Key(event) = event::read()? {
			return Ok(event);
		}
	}
}

fn _die(e: &crossterm::ErrorKind) {
	panic!("{}", e);
}
