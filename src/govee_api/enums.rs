pub enum Cmd {
    Turn,
    Brightness,
    Color,
    ColorTem,
}

impl Cmd {
    pub fn value(&self) -> &str {
        match self {
            Cmd::Turn => "turn",
            Cmd::Brightness => "brightness",
            Cmd::Color => "color",
            Cmd::ColorTem => "colorTem",
        }
    }
}