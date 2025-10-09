pub enum ColorMode {
  Dark,
  Light,
}

impl ColorMode {
  pub fn is<T: Into<ColorModeInput>>(color_mode: T) -> bool {
    match color_mode.into() {
      ColorModeInput::String(s) => matches!(s.to_lowercase().as_str(), "dark" | "light"),
      ColorModeInput::ColorMode(c) => match c {
        ColorMode::Dark => true,
        ColorMode::Light => true,
      },
    }
  }
}

pub enum ColorModeInput {
  String(String),
  ColorMode(ColorMode),
}

impl From<String> for ColorModeInput {
  fn from(s: String) -> Self {
    ColorModeInput::String(s)
  }
}

impl From<ColorMode> for ColorModeInput {
  fn from(c: ColorMode) -> Self {
    ColorModeInput::ColorMode(c)
  }
}