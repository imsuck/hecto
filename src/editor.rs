use crate::Terminal;
use crossterm::{
	event::{KeyCode, KeyModifiers},
	Result,
};

pub struct Editor {
	should_quit: bool,
	terminal: Terminal,
}

impl Editor {
	pub fn run(&mut self) {
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
		Self {
			should_quit: false,
			terminal: Terminal::default().expect("Failed to initialize terminal"),
		}
	}
	fn refresh_screen(&self) -> Result<()> {
		// execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
		Terminal::clear_screen()?;
		Terminal::cursor_position(0, 0)?;

		if self.should_quit {
			println!("Goodbye.\r");
		} else {
			self.draw_rows();
			Terminal::cursor_position(0, 0)?;
		}

		Terminal::flush()
	}
	fn process_keypress(&mut self) -> Result<()> {
		use KeyCode::{Char, Esc};

		let pressed_key = Terminal::read_key()?;
		match (pressed_key.modifiers, pressed_key.code) {
			(KeyModifiers::CONTROL, Char('q')) | (_, Esc) => self.should_quit = true,
			_ => (),
		}

		Ok(())
	}
	fn draw_rows(&self) {
		for _ in 0..self.terminal.size().height - 1 {
			println!("~\r");
		}
	}
}

fn die(e: &std::io::Error) {
	Terminal::clear_screen().ok();
	panic!("{}", e);
}
