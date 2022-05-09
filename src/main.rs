#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

fn main() -> crossterm::Result<()> {
	Editor::default().run()?;

	Ok(())
}
