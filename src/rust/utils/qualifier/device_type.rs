pub enum DeviceType {
  Phone,
  Tablet,
  Tv,
  Car,
  Wearable,
  TwoInOne,
}

impl DeviceType {
  pub fn is<T: Into<DeviceTypeInput>>(device_type: T) -> bool {
    match device_type.into() {
      DeviceTypeInput::String(s) => {
        let lower_device_type = s.to_lowercase();
        matches!(lower_device_type.as_str(), "phone" | "tablet" | "tv" | "car" | "wearable" | "2in1")
      }
      DeviceTypeInput::DeviceType(device_type) => {
        // 如果传入的是 DeviceType 枚举，直接返回 true
        // 因为如果能够构造出 DeviceType，说明它是有效的
        match device_type {
          DeviceType::Phone | DeviceType::Tablet | DeviceType::Tv | DeviceType::Car | DeviceType::Wearable | DeviceType::TwoInOne => true,
        }
      }
    }
  }
}

/// 用于表示设备类型输入的枚举
pub enum DeviceTypeInput {
  String(String),
  DeviceType(DeviceType),
}

/// 为 String 实现 Into<DeviceTypeInput>
impl From<String> for DeviceTypeInput {
  fn from(s: String) -> Self {
    DeviceTypeInput::String(s)
  }
}

/// 为 &str 实现 Into<DeviceTypeInput>
impl From<&str> for DeviceTypeInput {
  fn from(s: &str) -> Self {
    DeviceTypeInput::String(s.to_string())
  }
}

/// 为 DeviceType 实现 Into<DeviceTypeInput>
impl From<DeviceType> for DeviceTypeInput {
  fn from(dt: DeviceType) -> Self {
    DeviceTypeInput::DeviceType(dt)
  }
}
