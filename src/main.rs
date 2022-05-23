#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else,
    clippy::pattern_type_mismatch,
    clippy::expect_used,
    clippy::blanket_clippy_restriction_lints,
    clippy::as_conversions,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::case_sensitive_file_extension_comparisons
)]
mod document;
mod editor;
mod filetype;
mod highlighting;
mod row;
mod terminal;

pub use document::Document;
use editor::Editor;
pub use editor::{Position, SearchDirection};
pub use filetype::{FileType, HighlightingOptions};
pub use highlighting::Type;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
