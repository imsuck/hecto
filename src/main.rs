#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::unused_self)]
mod editor;
use editor::Editor;

fn main() {
	Editor::default().run();
}
