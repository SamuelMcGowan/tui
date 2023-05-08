#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,

    #[default]
    Default = 9,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weight {
    #[default]
    Normal,
    Bold,
    Dim,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,

    pub weight: Weight,
    pub underline: bool,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_fg(mut self, fg: Color) -> Self {
        self.fg = fg;
        self
    }

    pub fn with_bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }

    pub fn with_weight(mut self, weight: Weight) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }
}
