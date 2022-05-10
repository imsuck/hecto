use crate::Terminal;
use crossterm::{
	event::{KeyCode, KeyModifiers},
	Result,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
	pub x: usize,
	pub y: usize,
}

pub struct Editor {
	should_quit: bool,
	terminal: Terminal,
	cursor_position: Position,
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
			cursor_position: Position { x: 2, y: 0 },
		}
	}

	fn refresh_screen(&self) -> Result<()> {
		Terminal::cursor_hide();
		Terminal::cursor_position(&Position { x: 0, y: 0 });

		if self.should_quit {
			Terminal::clear_screen();
			println!("Goodbye.\r");
		} else {
			self.draw_rows();
			Terminal::cursor_position(&self.cursor_position);
		}
		Terminal::cursor_show();
		Terminal::flush()
	}
	fn process_keypress(&mut self) -> Result<()> {
		use KeyCode::{Char, Down, Esc, Left, Right, Up};

		let pressed_key = Terminal::read_key()?;
		match (pressed_key.modifiers, pressed_key.code) {
			(KeyModifiers::CONTROL, Char('q')) | (_, Esc) => self.should_quit = true,
			(_, Up) | (_, Down) | (_, Left) | (_, Right) => self.move_cursor(pressed_key.code),
			_ => (),
		}

		Ok(())
	}
	fn move_cursor(&mut self, key: KeyCode) {
		use KeyCode::{Down, Left, Right, Up};

		let Position { mut x, mut y } = self.cursor_position;
		match key {
			Up => y = y.saturating_sub(1),
			Down => y = y.saturating_add(1),
			Left => x = x.saturating_sub(1),
			Right => x = x.saturating_add(1),
			_ => (),
		}
		self.cursor_position = Position { x, y };
	}
	fn draw_welcome_message(&self) {
		let mut welcome_message = format!("Hecto editor -- version {}\r", VERSION);
		let width = self.terminal.size().width as usize;
		let len = welcome_message.len();
		let padding = width.saturating_sub(len) / 2;
		let spaces = " ".repeat(padding.saturating_sub(1));
		welcome_message = format!("~{}{}", spaces, welcome_message);
		welcome_message.truncate(width);
		println!("{}\r", welcome_message);
	}
	fn draw_rows(&self) {
		let height = self.terminal.size().height;
		for row in 0..height - 1 {
			Terminal::clear_current_line();
			if row == height / 3 {
				self.draw_welcome_message();
			} else {
				println!("~\r");
			}
		}
	}
}

fn die(e: &std::io::Error) {
	Terminal::clear_screen();
	panic!("{}", e);
}
