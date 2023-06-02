use crossterm::style::Color;

#[derive(PartialEq, Clone, Copy)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Char,
    Comment,
    MultilineComment,
    PrimaryKeywords,
    SecondaryKeywords,
}

impl Type {
    pub fn to_color(self) -> Color {
        match self {
            Type::Number => Color::Yellow,
            Type::Match => Color::Blue,
            Type::String => Color::Green,
            Type::Char => Color::Yellow,
            Type::Comment | Type::MultilineComment => Color::DarkGrey,
            Type::PrimaryKeywords => Color::Magenta,
            Type::SecondaryKeywords => Color::Yellow,
            Type::None => Color::Reset,
        }
    }
}
