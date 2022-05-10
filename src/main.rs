#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::unused_self, clippy::missing_errors_doc)]
mod editor;
mod terminal;
use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;

fn main() {
	Editor::default().run();
}
