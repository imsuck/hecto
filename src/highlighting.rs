use crossterm::style::Color;

#[derive(PartialEq)]
pub enum Type {
    None,
    Number,
    Match,
    String,
}

impl Type {
    pub fn to_color(&self) -> Color {
        match self {
            Type::Number => Color::Rgb {
                r: 220,
                g: 163,
                b: 163,
            },
            Type::Match => Color::Rgb {
                r: 38,
                g: 139,
                b: 210,
            },
            Type::String => Color::Rgb {
                r: 211,
                g: 54,
                b: 130,
            },
            Type::None => Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
        }
    }
}
