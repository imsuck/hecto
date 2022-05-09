use crossterm::{
    Result,
    terminal,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
};

pub struct Editor {}

impl Editor {
    pub fn run(&self) -> Result<()> {
        terminal::enable_raw_mode()?;

        loop {
            if let Event::Key(key) = event::read()? {
                // If it's Ctrl + Q we end the program
                match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,
                    _ => (),
                }

                if let Some(c) = handle_input(key) {
                    if c.len() == 1 {
                        let c = c.chars().next().unwrap_or('0');
                        if c.is_control() {
                            println!("{}\r", c as u8);
                        } else {
                            println!("{} ({})\r", c as u8, c)
                        }
                    } else {
                        println!("{}\r", c);
                    }
                }
            }
        }

        Ok(())
    }
    pub fn default() -> Self {
        Editor {}
    }
}

fn handle_input(key: KeyEvent) -> Option<String> {
    match key.modifiers {
        KeyModifiers::NONE => if let KeyCode::Char(c) = key.code {
            Some(c.to_string())
        } else {
            None
        },
        KeyModifiers::CONTROL => if let KeyCode::Char(c) = key.code {
            Some(((c as u8 & 0b0001_1111) as char).to_string())
        } else {
            None
        },
        _ => Some(format!("{:?}", key.code))
    }
}
