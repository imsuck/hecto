use crossterm::{
	cursor::MoveTo,
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
	pub fn run(&mut self) {
		if let Err(e) = enable_raw_mode() {
			die(&e);
		}

		loop {
			if let Err(e) = self.refresh_screen() {
				die(&e);
			}

			if self.should_quit {
				return;
			}

			if let Err(e) = self.process_keypress() {
				die(&e);
			}
		}
	}
	pub fn default() -> Self {
		Self { should_quit: false }
	}
	fn refresh_screen(&self) -> Result<()> {
		execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
		if self.should_quit {
			println!("Goodbye.\r");
		}
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

fn die(e: &std::io::Error) {
	execute!(stdout(), Clear(ClearType::All)).ok();
	panic!("{}", e);
}
