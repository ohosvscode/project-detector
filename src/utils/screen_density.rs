///
/// Screen density (DPI) level.
///
/// ---
///
/// 屏幕密度 (DPI) 等级。
///
pub enum ScreenDensity {
  Sdpi,
  Mdpi,
  Ldpi,
  Xldpi,
  Xxldpi,
  Xxxldpi,
}

impl ScreenDensity {
  pub fn is<T: Into<ScreenDensityInput>>(screen_density: T) -> bool {
    match screen_density.into() {
      ScreenDensityInput::String(s) => match s.to_lowercase().as_str() {
        "sdpi" => true,
        "mdpi" => true,
        "ldpi" => true,
        "xldpi" => true,
        "xxldpi" => true,
        "xxxldpi" => true,
        _ => false,
      },
      ScreenDensityInput::ScreenDensity(sd) => match sd {
        ScreenDensity::Sdpi => true,
        ScreenDensity::Mdpi => true,
        ScreenDensity::Ldpi => true,
        ScreenDensity::Xldpi => true,
        ScreenDensity::Xxldpi => true,
        ScreenDensity::Xxxldpi => true,
      },
    }
  }
}

pub enum ScreenDensityInput {
  String(String),
  ScreenDensity(ScreenDensity),
}

impl From<String> for ScreenDensityInput {
  fn from(s: String) -> Self {
    ScreenDensityInput::String(s)
  }
}
