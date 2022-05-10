#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::unused_self)]
mod editor;
mod terminal;
use editor::Editor;
pub use terminal::Terminal;

fn main() {
	Editor::default().run();
}
