use strum_macros::EnumIter;

///
/// Device screen orientation.
///
/// ---
///
/// 设备屏幕方向。
///
#[derive(EnumIter, strum_macros::Display)]
pub enum Orientation {
  Vertical,
  Horizontal,
}

impl Orientation {
  pub fn is<T: Into<OrientationInput>>(orientation: T) -> bool {
    match orientation.into() {
      OrientationInput::String(s) => match s.to_lowercase().as_str() {
        "vertical" => true,
        "horizontal" => true,
        _ => false,
      },
      OrientationInput::Orientation(o) => match o {
        Orientation::Vertical => true,
        Orientation::Horizontal => true,
      },
    }
  }
}

pub enum OrientationInput {
  String(String),
  Orientation(Orientation),
}

impl From<String> for OrientationInput {
  fn from(s: String) -> Self {
    OrientationInput::String(s)
  }
}
