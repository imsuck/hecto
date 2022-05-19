use std::env;
use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style::Color;
use crossterm::Result;

use crate::{Document, Row, Terminal};

const STATUS_FG_COLOR: Color = Color::Rgb {
    r: 63,
    g: 63,
    b: 63,
};
const STATUS_BG_COLOR: Color = Color::Rgb {
    r: 239,
    g: 239,
    b: 239,
};
const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 2;

#[non_exhaustive]
#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            text: message,
            time: Instant::now(),
        }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
    quit_times: u8,
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
        let mut initial_status =
            String::from("HELP: Ctrl-F = find | Ctrl-S = save | Ctrl-Q = quit");

        let document = if let Some(file_name) = args.get(1) {
            let doc = Document::open(file_name);
            if let Ok(doc) = doc {
                doc
            } else {
                initial_status = format!("ERR: Could not open file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
            status_message: StatusMessage::from(initial_status),
            quit_times: QUIT_TIMES,
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
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    fn save(&mut self) {
        if self.document.file_name.is_none() {
            let new_name = self.prompt("Save as: ").unwrap_or(None);

            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted.".to_owned());
                return;
            }

            self.document.file_name = new_name;
        }

        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("File successfully saved.".to_owned());
        } else {
            self.status_message = StatusMessage::from("Error writing file!".to_owned());
        }
    }

    #[allow(clippy::integer_arithmetic)]
    fn process_keypress(&mut self) -> Result<()> {
        let pressed_key = Terminal::read_key()?;
        match (pressed_key.modifiers, pressed_key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('q')) | (_, KeyCode::Esc) => {
                if self.quit_times > 0 && self.document.is_dirty() {
                    self.status_message = StatusMessage::from(format!(
                        "WARNING! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                        self.quit_times
                    ));
                    self.quit_times -= 1;
                    return Ok(());
                }

                self.should_quit = true;
            },
            (KeyModifiers::CONTROL, KeyCode::Char('s')) => self.save(),
            (KeyModifiers::CONTROL, KeyCode::Char('f')) => {
                if let Some(query) = self.prompt("Search: ").unwrap_or(None) {
                    if let Some(position) = self.document.find(&query) {
                        self.cursor_position = position;
                    } else {
                        self.status_message = StatusMessage::from(format!("Not found :{}.", query));
                    }
                }
            },

            (_, KeyCode::Char(c)) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(KeyCode::Right);
            },
            (
                _,
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::End
                | KeyCode::Home,
            ) => {
                self.move_cursor(pressed_key.code);
            },
            (_, KeyCode::Delete) => self.document.delete(&self.cursor_position),
            (_, KeyCode::Backspace) => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(KeyCode::Left);
                    self.document.delete(&self.cursor_position);
                }
            },
            (_, KeyCode::Enter) => {
                self.document.insert(&self.cursor_position, '\n');
                self.cursor_position.x = 0;
                self.cursor_position.y += 1;
            },
            _ => (),
        }

        self.scroll();

        if self.quit_times < QUIT_TIMES {
            self.quit_times = QUIT_TIMES;
            self.status_message = StatusMessage::from(String::new());
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
            Left =>
            {
                #[allow(clippy::integer_arithmetic)]
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;

                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            },
            Right =>
            {
                #[allow(clippy::integer_arithmetic)]
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            },
            PageUp => {
                y = if y > terminal_height {
                    y.saturating_sub(terminal_height)
                } else {
                    0
                }
            },
            PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y.saturating_add(terminal_height)
                } else {
                    height
                }
            },
            Home => x = 0,
            End => x = width,
            _ => (),
        }

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
        #[allow(clippy::integer_arithmetic, clippy::integer_division)]
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x.saturating_add(width);
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    #[allow(clippy::integer_arithmetic, clippy::integer_division)]
    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for terminal_row in 0..height {
            Terminal::clear_current_line();

            if let Some(row) = self
                .document
                .row(self.offset.y.saturating_add(terminal_row as usize))
            {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_status_bar(&self) {
        let width = self.terminal.size().width as usize;
        let mut file_name = "[No Name]".to_owned();

        let modified_indicator = if self.document.is_dirty() {
            " (modified)"
        } else {
            ""
        };

        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }

        let mut status = format!(
            "{} - {} lines{}",
            file_name,
            self.document.len(),
            modified_indicator
        );
        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );

        #[allow(clippy::integer_arithmetic)]
        let len = status.len() + line_indicator.len();

        status.push_str(&" ".repeat(width.saturating_sub(len)));
        status = format!("{}{}", status, line_indicator);
        status.truncate(width);

        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }

    fn prompt(&mut self, prompt: &str) -> Result<Option<String>> {
        let mut result = String::new();

        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;

            match Terminal::read_key()?.code {
                KeyCode::Backspace => {
                    if !result.is_empty() {
                        result.truncate(result.len().saturating_sub(1));
                    }
                },
                KeyCode::Enter => break,
                KeyCode::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                },
                KeyCode::Esc => {
                    result.truncate(0);
                    break;
                },
                _ => (),
            }
        }

        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None);
        }

        Ok(Some(result))
    }
}

#[allow(clippy::panic)]
fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
