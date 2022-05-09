mod editor;
use editor::Editor;

fn main() -> crossterm::Result<()> {
    let editor = Editor::default();
    editor.run()?;

    Ok(())
}
