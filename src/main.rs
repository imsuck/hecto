#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::unused_self)]
mod editor;
use editor::Editor;

fn main() -> crossterm::Result<()> {
	Editor::default().run()?;

	Ok(())
}
