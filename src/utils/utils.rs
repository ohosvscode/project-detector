use napi_derive::napi;
use crate::utils::color_mode::ColorMode;
use crate::utils::device_type::DeviceType;
use crate::utils::language_code::LanguageCode;
use crate::utils::mcc::MCC;
use crate::utils::mnc::MNC;
use crate::utils::orientation::Orientation;
use crate::utils::region_code::RegionCode;
use crate::utils::screen_density::ScreenDensity;

#[napi]
pub enum QualifierType {
  MCC,
  MNC,
  RegionCode,
  Orientation,
  ScreenDensity,
  ColorMode,
  LanguageCode,
  DeviceType,
}

#[napi]
pub struct Qualifier {
  /**
   * The type of the qualifier.
   */
  pub qualifier_type: QualifierType,
  /**
   * The value of the qualifier.
   */
  pub qualifier_value: String,
}

#[napi]
pub struct Utils {}

#[napi]
impl Utils {
  /**
   * Check if the mcc is a valid MCC code with value.
   */
  #[napi]
  pub fn is_mcc(mcc: u32) -> bool {
    MCC::is(mcc)
  }

  /**
   * Check if the mcc is a valid MCC code with string `mcc`.
   * For example: "mcc310" => true, "mcc3100" => false
   */
  #[napi]
  pub fn is_mcc_code(mcc: String) -> bool {
    MCC::is_code(mcc)
  }

  /**
   * Check if the language code is a valid language code.
   * For example: "en" => true, "en-US" => false
   */
  #[napi]
  pub fn is_language_code(language_code: String) -> bool {
    LanguageCode::is(language_code)
  }

  /**
   * Check if the device type is a valid device type.
   * - phone
   * - tablet
   * - tv
   * - car
   * - wearable
   * - 2in1
   */
  #[napi]
  pub fn is_device_type(device_type: String) -> bool {
    DeviceType::is(device_type)
  }

  /**
   * Check if the color mode is a valid color mode.
   * - dark
   * - light
   */
  #[napi]
  pub fn is_color_mode(color_mode: String) -> bool {
    ColorMode::is(color_mode)
  }

  /**
   * Check if the mnc is a valid MNC code with value.
   */
  #[napi]
  pub fn is_mnc(mnc: u32, mcc: u32) -> bool {
    MNC::is(mcc, mnc)
  }

  /**
   * Check if the mnc is a valid MNC code with string `mnc` and `mcc`.
   * For example: "mnc00" => true, "mnc000" => false
   */
  #[napi]
  pub fn is_mnc_code(mnc: String, mcc: u32) -> bool {
    MNC::is_code(mnc, mcc)
  }

  /**
   * Check if the region code is a valid region code.
   * For example: "CN" => true, "US" => true, "AAA" => false
   */
  #[napi]
  pub fn is_region_code(region_code: String) -> bool {
    RegionCode::is(region_code)
  }

  /**
   * Check if the orientation is a valid orientation.
   * - vertical
   * - horizontal
   */
  #[napi]
  pub fn is_orientation(orientation: String) -> bool {
    Orientation::is(orientation)
  }

  /**
   * Check if the screen density is a valid screen density.
   * - sdpi
   * - mdpi
   * - ldpi
   * - xldpi
   * - xxldpi
   * - xxxldpi
   */
  #[napi]
  pub fn is_screen_density(screen_density: String) -> bool {
    ScreenDensity::is(screen_density)
  }

  /**
   * Analyze the qualifier and return the qualifier list.
   * 
   * 限定词目录由一个或多个表征应用场景或设备特征的限定词组合而成，限定词包括移动国家码和移动网络码、语言、文字、国家或地区、横竖屏、设备类型、颜色模式和屏幕密度，限定词之间通过下划线（_）或者中划线（-）连接。开发者在创建限定词目录时，需要遵守如下限定词目录命名规则。
   * 限定词的组合顺序：移动国家码_移动网络码-语言_文字_国家或地区-横竖屏-设备类型-颜色模式-屏幕密度。开发者可以根据应用的使用场景和设备特征，选择其中的一类或几类限定词组成目录名称。
   * 限定词的连接方式：移动国家码和移动网络码之间采用下划线（_）连接，语言、文字、国家或地区之间也采用下划线（_）连接，除此之外的其他限定词之间均采用中划线（-）连接。例如：`mcc460_mnc00-zh_Hant_CN`、`zh_CN-car-ldpi`。
   * 限定词的取值范围：每类限定词的取值必须符合限定词取值要求表中的条件，如表5。否则，将无法匹配目录中的资源文件。
   */
  #[napi]
  pub fn analyze_qualifier(qualifiers: String) -> Vec<Qualifier> {
    let mut result = Vec::new();
    
    if qualifiers.is_empty() {
      return result;
    }
    
    // 按照中划线分割，获取所有部分
    let parts: Vec<&str> = qualifiers
      .split('-')
      .filter(|part| !part.is_empty())
      .collect();
    
    // 处理每个部分
    for (index, part) in parts.iter().enumerate() {
      let is_first_part = index == 0;
      Self::parse_qualifier_part(part, is_first_part, &mut result);
    }
    
    result
  }
  
  /// 解析单个限定词部分
  /// 
  /// # Arguments
  /// * `part` - 要解析的部分
  /// * `is_first_part` - 是否为第一部分（可能包含 MCC_MNC 或语言相关）
  /// * `result` - 用于存储解析结果的向量
  fn parse_qualifier_part(part: &str, is_first_part: bool, result: &mut Vec<Qualifier>) {
    // 如果包含下划线，可能是复合限定词
    if part.contains('_') {
      if is_first_part && Self::try_parse_mcc_mnc(part, result) {
        // MCC_MNC 成功解析
        return;
      }
      // 尝试作为语言/区域组合解析
      Self::parse_language_region(part, result);
    } else {
      // 单个限定词
      Self::try_parse_single_qualifier(part, result);
    }
  }
  
  /// 尝试解析 MCC_MNC 组合
  /// 
  /// 格式: mcc<code>_mnc<code> 或 mcc<code>_mnc<code>_<language_parts>
  /// 
  /// # Returns
  /// 如果成功解析了 MCC_MNC，返回 true；否则返回 false
  fn try_parse_mcc_mnc(part: &str, result: &mut Vec<Qualifier>) -> bool {
    // 检查是否符合 MCC_MNC 格式
    if !part.starts_with("mcc") || !part.contains("_mnc") {
      return false;
    }
    
    let parts: Vec<&str> = part.split('_').collect();
    if parts.len() < 2 {
      return false;
    }
    
    let mcc_part = parts[0];
    let mnc_part = parts[1];
    
    // 解析并验证 MCC
    let mcc_value = if MCC::is_code(mcc_part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::MCC,
        qualifier_value: mcc_part.to_string(),
      });
      // 提取 MCC 数值用于 MNC 验证
      mcc_part.replace("mcc", "").parse::<u32>().unwrap_or(0)
    } else {
      return false; // MCC 无效，不继续解析
    };
    
    // 解析并验证 MNC
    if MNC::is_code(mnc_part.to_string(), mcc_value) {
      result.push(Qualifier {
        qualifier_type: QualifierType::MNC,
        qualifier_value: mnc_part.to_string(),
      });
    }
    
    // 如果有额外的语言/区域部分，继续解析
    if parts.len() > 2 {
      let remaining = parts[2..].join("_");
      Self::parse_language_region(&remaining, result);
    }
    
    true
  }
  
  /// 解析语言、文字、国家或地区组合
  /// 
  /// 格式: <language>_<script>_<region> 或其部分组合
  /// 例如: zh_Hant_CN, en_US, zh_CN
  fn parse_language_region(part: &str, result: &mut Vec<Qualifier>) {
    let parts: Vec<&str> = part
      .split('_')
      .filter(|p| !p.is_empty())
      .collect();
    
    for part in parts {
      Self::classify_and_add_language_or_region(part, result);
    }
  }
  
  /// 分类并添加语言代码或区域代码
  fn classify_and_add_language_or_region(part: &str, result: &mut Vec<Qualifier>) {
    // 两字母且是有效语言代码 -> 语言代码
    if part.len() == 2 && LanguageCode::is(part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::LanguageCode,
        qualifier_value: part.to_string(),
      });
    }
    // 是有效的区域代码 -> 区域代码
    else if RegionCode::is(part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::RegionCode,
        qualifier_value: part.to_string(),
      });
    }
    // 其他情况（如文字代码 Hant, Hans）-> 作为语言代码
    else {
      result.push(Qualifier {
        qualifier_type: QualifierType::LanguageCode,
        qualifier_value: part.to_string(),
      });
    }
  }
  
  /// 尝试解析单个限定词
  /// 
  /// 按照以下优先级检查：
  /// 1. 屏幕方向 (vertical, horizontal)
  /// 2. 设备类型 (phone, tablet, tv, car, wearable, 2in1)
  /// 3. 颜色模式 (dark, light)
  /// 4. 屏幕密度 (sdpi, mdpi, ldpi, xldpi, xxldpi, xxxldpi)
  /// 5. 语言代码
  /// 6. 区域代码
  /// 
  /// # Returns
  /// 如果成功识别，返回 true；否则返回 false
  fn try_parse_single_qualifier(part: &str, result: &mut Vec<Qualifier>) -> bool {
    // 屏幕方向
    if Orientation::is(part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::Orientation,
        qualifier_value: part.to_string(),
      });
      return true;
    }
    
    // 设备类型
    if DeviceType::is(part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::DeviceType,
        qualifier_value: part.to_string(),
      });
      return true;
    }
    
    // 颜色模式
    if ColorMode::is(part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::ColorMode,
        qualifier_value: part.to_string(),
      });
      return true;
    }
    
    // 屏幕密度
    if ScreenDensity::is(part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::ScreenDensity,
        qualifier_value: part.to_string(),
      });
      return true;
    }
    
    // 语言代码
    if LanguageCode::is(part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::LanguageCode,
        qualifier_value: part.to_string(),
      });
      return true;
    }
    
    // 区域代码
    if RegionCode::is(part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::RegionCode,
        qualifier_value: part.to_string(),
      });
      return true;
    }
    
    // 无法识别
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_analyze_qualifier_empty() {
    let result = Utils::analyze_qualifier("".to_string());
    assert_eq!(result.len(), 0);
  }

  #[test]
  fn test_analyze_qualifier_mcc_mnc_language_region() {
    // 测试示例: mcc460_mnc00-zh_Hant_CN
    let result = Utils::analyze_qualifier("mcc460_mnc00-zh_Hant_CN".to_string());
    assert_eq!(result.len(), 5);
    
    assert!(matches!(result[0].qualifier_type, QualifierType::MCC));
    assert_eq!(result[0].qualifier_value, "mcc460");
    
    assert!(matches!(result[1].qualifier_type, QualifierType::MNC));
    assert_eq!(result[1].qualifier_value, "mnc00");
    
    assert!(matches!(result[2].qualifier_type, QualifierType::LanguageCode));
    assert_eq!(result[2].qualifier_value, "zh");
    
    // Hant 作为文字/语言代码
    assert!(matches!(result[3].qualifier_type, QualifierType::LanguageCode));
    assert_eq!(result[3].qualifier_value, "Hant");
    
    assert!(matches!(result[4].qualifier_type, QualifierType::RegionCode));
    assert_eq!(result[4].qualifier_value, "CN");
  }

  #[test]
  fn test_analyze_qualifier_language_device_density() {
    // 测试示例: zh_CN-car-ldpi
    let result = Utils::analyze_qualifier("zh_CN-car-ldpi".to_string());
    assert_eq!(result.len(), 4);
    
    assert!(matches!(result[0].qualifier_type, QualifierType::LanguageCode));
    assert_eq!(result[0].qualifier_value, "zh");
    
    assert!(matches!(result[1].qualifier_type, QualifierType::RegionCode));
    assert_eq!(result[1].qualifier_value, "CN");
    
    assert!(matches!(result[2].qualifier_type, QualifierType::DeviceType));
    assert_eq!(result[2].qualifier_value, "car");
    
    assert!(matches!(result[3].qualifier_type, QualifierType::ScreenDensity));
    assert_eq!(result[3].qualifier_value, "ldpi");
  }

  #[test]
  fn test_analyze_qualifier_language_only() {
    let result = Utils::analyze_qualifier("en".to_string());
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0].qualifier_type, QualifierType::LanguageCode));
    assert_eq!(result[0].qualifier_value, "en");
  }

  #[test]
  fn test_analyze_qualifier_language_region() {
    let result = Utils::analyze_qualifier("en_US".to_string());
    assert_eq!(result.len(), 2);
    
    assert!(matches!(result[0].qualifier_type, QualifierType::LanguageCode));
    assert_eq!(result[0].qualifier_value, "en");
    
    assert!(matches!(result[1].qualifier_type, QualifierType::RegionCode));
    assert_eq!(result[1].qualifier_value, "US");
  }

  #[test]
  fn test_analyze_qualifier_orientation_device() {
    let result = Utils::analyze_qualifier("vertical-phone".to_string());
    assert_eq!(result.len(), 2);
    
    assert!(matches!(result[0].qualifier_type, QualifierType::Orientation));
    assert_eq!(result[0].qualifier_value, "vertical");
    
    assert!(matches!(result[1].qualifier_type, QualifierType::DeviceType));
    assert_eq!(result[1].qualifier_value, "phone");
  }

  #[test]
  fn test_analyze_qualifier_horizontal_tablet_dark_xxldpi() {
    let result = Utils::analyze_qualifier("horizontal-tablet-dark-xxldpi".to_string());
    assert_eq!(result.len(), 4);
    
    assert!(matches!(result[0].qualifier_type, QualifierType::Orientation));
    assert_eq!(result[0].qualifier_value, "horizontal");
    
    assert!(matches!(result[1].qualifier_type, QualifierType::DeviceType));
    assert_eq!(result[1].qualifier_value, "tablet");
    
    assert!(matches!(result[2].qualifier_type, QualifierType::ColorMode));
    assert_eq!(result[2].qualifier_value, "dark");
    
    assert!(matches!(result[3].qualifier_type, QualifierType::ScreenDensity));
    assert_eq!(result[3].qualifier_value, "xxldpi");
  }

  #[test]
  fn test_analyze_qualifier_color_mode_only() {
    let result = Utils::analyze_qualifier("dark".to_string());
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0].qualifier_type, QualifierType::ColorMode));
    assert_eq!(result[0].qualifier_value, "dark");
  }

  #[test]
  fn test_analyze_qualifier_screen_density_only() {
    let result = Utils::analyze_qualifier("mdpi".to_string());
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0].qualifier_type, QualifierType::ScreenDensity));
    assert_eq!(result[0].qualifier_value, "mdpi");
  }

  #[test]
  fn test_analyze_qualifier_device_type_only() {
    let result = Utils::analyze_qualifier("wearable".to_string());
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0].qualifier_type, QualifierType::DeviceType));
    assert_eq!(result[0].qualifier_value, "wearable");
  }

  #[test]
  fn test_analyze_qualifier_complex() {
    // 测试完整组合
    let result = Utils::analyze_qualifier("mcc460_mnc00-zh_CN-vertical-phone-light-xldpi".to_string());
    assert_eq!(result.len(), 8);
    
    assert!(matches!(result[0].qualifier_type, QualifierType::MCC));
    assert_eq!(result[0].qualifier_value, "mcc460");
    
    assert!(matches!(result[1].qualifier_type, QualifierType::MNC));
    assert_eq!(result[1].qualifier_value, "mnc00");
    
    assert!(matches!(result[2].qualifier_type, QualifierType::LanguageCode));
    assert_eq!(result[2].qualifier_value, "zh");
    
    assert!(matches!(result[3].qualifier_type, QualifierType::RegionCode));
    assert_eq!(result[3].qualifier_value, "CN");
    
    assert!(matches!(result[4].qualifier_type, QualifierType::Orientation));
    assert_eq!(result[4].qualifier_value, "vertical");
    
    assert!(matches!(result[5].qualifier_type, QualifierType::DeviceType));
    assert_eq!(result[5].qualifier_value, "phone");
    
    assert!(matches!(result[6].qualifier_type, QualifierType::ColorMode));
    assert_eq!(result[6].qualifier_value, "light");
    
    assert!(matches!(result[7].qualifier_type, QualifierType::ScreenDensity));
    assert_eq!(result[7].qualifier_value, "xldpi");
  }

  #[test]
  fn test_analyze_qualifier_invalid_mcc() {
    // 无效的 MCC 代码
    let result = Utils::analyze_qualifier("mcc9999-zh_CN".to_string());
    // 无效的 MCC 不应该被识别，只识别语言和区域
    assert_eq!(result.len(), 2);
    assert!(matches!(result[0].qualifier_type, QualifierType::LanguageCode));
    assert!(matches!(result[1].qualifier_type, QualifierType::RegionCode));
  }

  #[test]
  fn test_analyze_qualifier_invalid_mnc() {
    // 无效的 MNC 代码
    let result = Utils::analyze_qualifier("mcc460_mnc9999-zh_CN".to_string());
    // MCC 有效，但 MNC 无效
    assert!(result.len() >= 1);
    assert!(matches!(result[0].qualifier_type, QualifierType::MCC));
    assert_eq!(result[0].qualifier_value, "mcc460");
    // 无效的 MNC 不会被添加
  }

  #[test]
  fn test_analyze_qualifier_invalid_language() {
    // 无效的语言代码
    let result = Utils::analyze_qualifier("xyz".to_string());
    // 不识别无效的语言代码，结果为空
    assert_eq!(result.len(), 0);
  }

  #[test]
  fn test_analyze_qualifier_invalid_device_type() {
    // 无效的设备类型
    let result = Utils::analyze_qualifier("zh_CN-superdevice-mdpi".to_string());
    assert_eq!(result.len(), 3);
    assert!(matches!(result[0].qualifier_type, QualifierType::LanguageCode));
    assert!(matches!(result[1].qualifier_type, QualifierType::RegionCode));
    // superdevice 无效，跳过
    assert!(matches!(result[2].qualifier_type, QualifierType::ScreenDensity));
    assert_eq!(result[2].qualifier_value, "mdpi");
  }

  #[test]
  fn test_analyze_qualifier_invalid_orientation() {
    // 无效的屏幕方向
    let result = Utils::analyze_qualifier("diagonal-phone".to_string());
    // diagonal 无效，但 phone 有效
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0].qualifier_type, QualifierType::DeviceType));
    assert_eq!(result[0].qualifier_value, "phone");
  }

  #[test]
  fn test_analyze_qualifier_invalid_color_mode() {
    // 无效的颜色模式
    let result = Utils::analyze_qualifier("phone-blue-mdpi".to_string());
    // blue 不是有效的颜色模式（只有 dark 和 light）
    assert_eq!(result.len(), 2);
    assert!(matches!(result[0].qualifier_type, QualifierType::DeviceType));
    assert_eq!(result[0].qualifier_value, "phone");
    // blue 无效，跳过
    assert!(matches!(result[1].qualifier_type, QualifierType::ScreenDensity));
    assert_eq!(result[1].qualifier_value, "mdpi");
  }

  #[test]
  fn test_analyze_qualifier_invalid_screen_density() {
    // 无效的屏幕密度
    let result = Utils::analyze_qualifier("phone-dark-ultra4k".to_string());
    assert_eq!(result.len(), 2);
    assert!(matches!(result[0].qualifier_type, QualifierType::DeviceType));
    assert!(matches!(result[1].qualifier_type, QualifierType::ColorMode));
    // ultra4k 无效，不被识别
  }

  #[test]
  fn test_analyze_qualifier_wrong_order_device_before_orientation() {
    // 错误的顺序：设备类型在屏幕方向之前（正确应该是 vertical-phone）
    let result = Utils::analyze_qualifier("phone-vertical".to_string());
    // 虽然顺序错误，但两个都是有效的限定词
    assert_eq!(result.len(), 2);
    // 由于我们的实现按顺序解析，phone 会被识别为设备类型
    assert!(matches!(result[0].qualifier_type, QualifierType::DeviceType));
    assert_eq!(result[0].qualifier_value, "phone");
    assert!(matches!(result[1].qualifier_type, QualifierType::Orientation));
    assert_eq!(result[1].qualifier_value, "vertical");
  }

  #[test]
  fn test_analyze_qualifier_mixed_valid_invalid() {
    // 混合有效和无效的限定词
    let result = Utils::analyze_qualifier("zh_CN-invalid-phone-wrongcolor-mdpi".to_string());
    // 只有有效的会被识别
    assert!(result.len() >= 3);
    assert!(matches!(result[0].qualifier_type, QualifierType::LanguageCode));
    assert!(matches!(result[1].qualifier_type, QualifierType::RegionCode));
    // 包含 phone 和 mdpi
    assert!(result.iter().any(|q| matches!(q.qualifier_type, QualifierType::DeviceType)));
    assert!(result.iter().any(|q| matches!(q.qualifier_type, QualifierType::ScreenDensity)));
  }

  #[test]
  fn test_analyze_qualifier_only_invalid() {
    // 只有无效的限定词
    let result = Utils::analyze_qualifier("invalid-wrong-bad".to_string());
    // 全部无效，结果为空
    assert_eq!(result.len(), 0);
  }

  #[test]
  fn test_analyze_qualifier_multiple_dashes() {
    // 多个连续的中划线
    let result = Utils::analyze_qualifier("zh_CN--phone".to_string());
    assert!(result.len() >= 2);
    assert!(matches!(result[0].qualifier_type, QualifierType::LanguageCode));
    assert!(matches!(result[1].qualifier_type, QualifierType::RegionCode));
  }

  #[test]
  fn test_analyze_qualifier_trailing_dash() {
    // 末尾有中划线
    let result = Utils::analyze_qualifier("zh_CN-phone-".to_string());
    assert_eq!(result.len(), 3);
    assert!(matches!(result[0].qualifier_type, QualifierType::LanguageCode));
    assert!(matches!(result[1].qualifier_type, QualifierType::RegionCode));
    assert!(matches!(result[2].qualifier_type, QualifierType::DeviceType));
  }

  #[test]
  fn test_analyze_qualifier_leading_dash() {
    // 开头有中划线
    let result = Utils::analyze_qualifier("-zh_CN-phone".to_string());
    assert!(result.len() >= 2);
  }

  #[test]
  fn test_analyze_qualifier_case_sensitivity() {
    // 测试大小写敏感性
    let result_upper = Utils::analyze_qualifier("DARK".to_string());
    let result_lower = Utils::analyze_qualifier("dark".to_string());
    let result_mixed = Utils::analyze_qualifier("Dark".to_string());
    
    // 颜色模式应该不区分大小写
    assert_eq!(result_upper.len(), 1);
    assert_eq!(result_lower.len(), 1);
    assert_eq!(result_mixed.len(), 1);
  }

  #[test]
  fn test_analyze_qualifier_region_without_language() {
    // 只有区域没有语言（虽然不推荐，但语法上可能出现）
    let result = Utils::analyze_qualifier("CN".to_string());
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0].qualifier_type, QualifierType::RegionCode));
    assert_eq!(result[0].qualifier_value, "CN");
  }

  #[test]
  fn test_analyze_qualifier_mcc_without_mnc() {
    // 只有 MCC 没有 MNC
    let result = Utils::analyze_qualifier("mcc460-zh_CN".to_string());
    // mcc460 单独出现时需要正确处理
    assert!(result.len() >= 2);
  }

  #[test]
  fn test_analyze_qualifier_duplicate_qualifiers() {
    // 重复的限定词（不推荐但可能出现）
    let result = Utils::analyze_qualifier("phone-phone".to_string());
    // 会被识别两次
    assert_eq!(result.len(), 2);
    assert!(matches!(result[0].qualifier_type, QualifierType::DeviceType));
    assert!(matches!(result[1].qualifier_type, QualifierType::DeviceType));
  }
}