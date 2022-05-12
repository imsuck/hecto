use crate::Document;
use crate::Row;
use crate::Terminal;
use crossterm::{
    event::{KeyCode, KeyModifiers},
    Result,
};
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
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
        let args: Vec<String> = env::args().collect();

        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(file_name).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
        }
    }

    fn refresh_screen(&self) -> Result<()> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());

        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<()> {
        use KeyCode::{Char, Down, End, Esc, Home, Left, PageDown, PageUp, Right, Up};

        let pressed_key = Terminal::read_key()?;
        match (pressed_key.modifiers, pressed_key.code) {
            (KeyModifiers::CONTROL, Char('q')) | (_, Esc) => self.should_quit = true,
            (_, Up | Down | Left | Right | PageUp | PageDown | End | Home) => {
                self.move_cursor(pressed_key.code);
            },
            _ => (),
        }

        Ok(())
    }

    fn move_cursor(&mut self, key: KeyCode) {
        use KeyCode::{Down, End, Home, Left, PageDown, PageUp, Right, Up};

		let terminal_height = self.terminal.size().height as usize;
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        match key {
            Up => y = y.saturating_sub(1),
            Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            },
            Left => x = x.saturating_sub(1),
            Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            },
            PageUp => {
				y = if y > terminal_height {
					y - terminal_height
				} else {
					0
				}
			},
            PageDown => {
				y = if y.saturating_add(terminal_height) < height {
					y + terminal_height as usize
				} else {
					height
				}
			},
            Home => x = 0,
            End => x = width,
            _ => (),
        }

        self.scroll();
        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y };
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
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

    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();

            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
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
