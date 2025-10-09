use crate::utils::color_mode::ColorMode;
use crate::utils::device_type::DeviceType;
use crate::utils::language_code::LanguageCode;
use crate::utils::mcc::MCC;
use crate::utils::mnc::MNC;
use crate::utils::orientation::Orientation;
use crate::utils::region_code::RegionCode;
use crate::utils::screen_density::ScreenDensity;
use napi_derive::napi;

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

#[napi(object)]
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

/// 限定词解析阶段（按照规范顺序）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum QualifierStage {
  MccMncOrLanguage = 0, // 第一阶段：MCC_MNC 或语言相关
  Orientation = 1,      // 第二阶段：横竖屏
  DeviceType = 2,       // 第三阶段：设备类型
  ColorMode = 3,        // 第四阶段：颜色模式
  ScreenDensity = 4,    // 第五阶段：屏幕密度
  Finished = 5,         // 完成
}

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
   *
   * **注意**：如果任何一个限定词无法识别或顺序错误，将返回空向量，表示整个限定词字符串无效。
   */
  #[napi]
  pub fn analyze_qualifier(qualifiers: String) -> Vec<Qualifier> {
    if qualifiers.is_empty() {
      return Vec::new();
    }

    // 按照中划线分割，获取所有部分
    let parts: Vec<&str> = qualifiers.split('-').filter(|part| !part.is_empty()).collect();

    let mut result = Vec::new();
    let mut stage = QualifierStage::MccMncOrLanguage;

    // 处理每个部分，严格验证顺序
    for (index, part) in parts.iter().enumerate() {
      let is_first_part = index == 0;

      match Self::parse_qualifier_part_with_stage(part, is_first_part, &mut stage, &mut result) {
        Ok(_) => {}
        Err(_) => {
          // 解析失败或顺序错误，返回空向量
          return Vec::new();
        }
      }
    }

    result
  }

  /// 根据当前阶段解析限定词部分
  ///
  /// # Returns
  /// Ok(()) 如果成功解析；Err(()) 如果解析失败或顺序错误
  fn parse_qualifier_part_with_stage(part: &str, is_first_part: bool, stage: &mut QualifierStage, result: &mut Vec<Qualifier>) -> Result<(), ()> {
    // 第一部分特殊处理：可以从任何阶段开始
    if is_first_part && *stage == QualifierStage::MccMncOrLanguage {
      // 尝试解析复合限定词
      if part.contains('_') {
        if part.starts_with("mcc") && part.contains("_mnc") {
          // MCC_MNC 解析
          if Self::try_parse_mcc_mnc(part, result) {
            *stage = QualifierStage::Orientation;
            return Ok(());
          } else {
            return Err(());
          }
        } else {
          // 语言/区域组合
          if Self::parse_language_region(part, result) {
            *stage = QualifierStage::Orientation;
            return Ok(());
          } else {
            return Err(());
          }
        }
      }

      // 单个限定词：尝试按顺序找到合适的起始阶段
      // 首先尝试作为语言或区域
      if LanguageCode::is(part.to_string()) {
        result.push(Qualifier {
          qualifier_type: QualifierType::LanguageCode,
          qualifier_value: part.to_string(),
        });
        *stage = QualifierStage::Orientation;
        return Ok(());
      }

      if RegionCode::is(part.to_string()) {
        result.push(Qualifier {
          qualifier_type: QualifierType::RegionCode,
          qualifier_value: part.to_string(),
        });
        *stage = QualifierStage::Orientation;
        return Ok(());
      }

      // 不是语言/区域，尝试后续阶段（允许跳过前面的阶段）
      return Self::try_parse_ordered_qualifier(part, stage, result);
    }

    // 后续部分按照固定顺序解析
    if part.contains('_') {
      // 语言相关 - 即使在 MCC_MNC 之后，也可以有语言部分
      // 只要还没到达 Orientation 之后的阶段
      if *stage <= QualifierStage::Orientation && Self::parse_language_region(part, result) {
        *stage = QualifierStage::Orientation;
        return Ok(());
      }
      return Err(()); // Orientation 之后的阶段不应该有下划线
    }

    // 尝试按顺序解析单个限定词
    Self::try_parse_ordered_qualifier(part, stage, result)
  }

  /// 按照固定顺序解析单个限定词
  fn try_parse_ordered_qualifier(part: &str, stage: &mut QualifierStage, result: &mut Vec<Qualifier>) -> Result<(), ()> {
    // 尝试按照当前阶段解析

    // 横竖屏阶段
    if *stage <= QualifierStage::Orientation && Orientation::is(part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::Orientation,
        qualifier_value: part.to_string(),
      });
      *stage = QualifierStage::DeviceType;
      return Ok(());
    }

    // 设备类型阶段
    if *stage <= QualifierStage::DeviceType && DeviceType::is(part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::DeviceType,
        qualifier_value: part.to_string(),
      });
      *stage = QualifierStage::ColorMode;
      return Ok(());
    }

    // 颜色模式阶段
    if *stage <= QualifierStage::ColorMode && ColorMode::is(part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::ColorMode,
        qualifier_value: part.to_string(),
      });
      *stage = QualifierStage::ScreenDensity;
      return Ok(());
    }

    // 屏幕密度阶段
    if *stage <= QualifierStage::ScreenDensity && ScreenDensity::is(part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::ScreenDensity,
        qualifier_value: part.to_string(),
      });
      *stage = QualifierStage::Finished;
      return Ok(());
    }

    // 无法识别或顺序错误
    Err(())
  }

  /// 尝试解析 MCC_MNC 组合
  ///
  /// 格式: `mcc【code】_mnc【code】` 或 `mcc【code】_mnc【code】_<language_parts>`
  ///
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

    // 解析并验证 MNC（必须有效）
    if !MNC::is_code(mnc_part.to_string(), mcc_value) {
      // MNC 无效，回滚已添加的 MCC
      result.pop();
      return false;
    }

    result.push(Qualifier {
      qualifier_type: QualifierType::MNC,
      qualifier_value: mnc_part.to_string(),
    });

    // 如果有额外的语言/区域部分，继续解析
    if parts.len() > 2 {
      let initial_len = result.len();
      let remaining = parts[2..].join("_");
      if !Self::parse_language_region(&remaining, result) {
        // 语言/区域解析失败，回滚所有内容
        result.truncate(initial_len - 2); // 回滚 MCC 和 MNC
        return false;
      }
    }

    true
  }

  /// 解析语言、文字、国家或地区组合
  ///
  /// 格式: <language>_<script>_<region> 或其部分组合
  /// 例如: zh_Hant_CN, en_US, zh_CN
  ///
  /// # Returns
  /// 如果所有部分都能识别，返回 true；否则返回 false
  fn parse_language_region(part: &str, result: &mut Vec<Qualifier>) -> bool {
    let parts: Vec<&str> = part.split('_').filter(|p| !p.is_empty()).collect();

    let initial_len = result.len();

    for part in parts {
      if !Self::classify_and_add_language_or_region(part, result) {
        // 遇到无法识别的部分，回滚已添加的内容
        result.truncate(initial_len);
        return false;
      }
    }

    true
  }

  /// 分类并添加语言代码或区域代码
  ///
  /// # Returns
  /// 如果能够识别并添加，返回 true；否则返回 false
  fn classify_and_add_language_or_region(part: &str, result: &mut Vec<Qualifier>) -> bool {
    // 两字母且是有效语言代码 -> 语言代码
    if part.len() == 2 && LanguageCode::is(part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::LanguageCode,
        qualifier_value: part.to_string(),
      });
      return true;
    }
    // 是有效的区域代码（通常是2字母大写）-> 区域代码
    else if RegionCode::is(part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::RegionCode,
        qualifier_value: part.to_string(),
      });
      return true;
    }
    // 3-4字母（可能是文字代码，如 Hant, Hans, Latn 等）-> 作为语言代码接受
    // 这是为了支持 ISO 15924 script codes
    // 或其他有效的语言代码
    else if (part.len() >= 3 && part.len() <= 4 && part.chars().all(|c| c.is_alphabetic())) || LanguageCode::is(part.to_string()) {
      result.push(Qualifier {
        qualifier_type: QualifierType::LanguageCode,
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
    // 无效的 MCC 代码 - 应该导致整个解析失败
    let result = Utils::analyze_qualifier("mcc9999-zh_CN".to_string());
    assert_eq!(result.len(), 0, "无效的 MCC 应该导致解析失败");
  }

  #[test]
  fn test_analyze_qualifier_invalid_mnc() {
    // 无效的 MNC 代码 - 应该导致整个解析失败
    let result = Utils::analyze_qualifier("mcc460_mnc9999-zh_CN".to_string());
    assert_eq!(result.len(), 0, "无效的 MNC 应该导致解析失败");
  }

  #[test]
  fn test_analyze_qualifier_invalid_language() {
    // 无效的语言代码 - 应该导致整个解析失败
    let result = Utils::analyze_qualifier("xyz".to_string());
    assert_eq!(result.len(), 0, "无效的语言代码应该导致解析失败");
  }

  #[test]
  fn test_analyze_qualifier_invalid_device_type() {
    // 无效的设备类型 - 应该导致整个解析失败
    let result = Utils::analyze_qualifier("zh_CN-superdevice-mdpi".to_string());
    assert_eq!(result.len(), 0, "包含无效设备类型应该导致解析失败");
  }

  #[test]
  fn test_analyze_qualifier_invalid_orientation() {
    // 无效的屏幕方向 - 应该导致整个解析失败
    let result = Utils::analyze_qualifier("diagonal-phone".to_string());
    assert_eq!(result.len(), 0, "无效的屏幕方向应该导致解析失败");
  }

  #[test]
  fn test_analyze_qualifier_invalid_color_mode() {
    // 无效的颜色模式 - 应该导致整个解析失败
    let result = Utils::analyze_qualifier("phone-blue-mdpi".to_string());
    assert_eq!(result.len(), 0, "无效的颜色模式应该导致解析失败");
  }

  #[test]
  fn test_analyze_qualifier_invalid_screen_density() {
    // 无效的屏幕密度 - 应该导致整个解析失败
    let result = Utils::analyze_qualifier("phone-dark-ultra4k".to_string());
    assert_eq!(result.len(), 0, "无效的屏幕密度应该导致解析失败");
  }

  #[test]
  fn test_analyze_qualifier_wrong_order_device_before_orientation() {
    // 错误的顺序：设备类型在屏幕方向之前（正确应该是 vertical-phone）
    // 顺序错误应该导致解析失败
    let result = Utils::analyze_qualifier("phone-vertical".to_string());
    assert_eq!(result.len(), 0, "顺序错误应该导致解析失败");
  }

  #[test]
  fn test_analyze_qualifier_correct_order() {
    // 正确的顺序
    let result = Utils::analyze_qualifier("vertical-phone".to_string());
    assert_eq!(result.len(), 2);
    assert!(matches!(result[0].qualifier_type, QualifierType::Orientation));
    assert_eq!(result[0].qualifier_value, "vertical");
    assert!(matches!(result[1].qualifier_type, QualifierType::DeviceType));
    assert_eq!(result[1].qualifier_value, "phone");
  }

  #[test]
  fn test_analyze_qualifier_wrong_order_2in1_before_language() {
    // 2in1-zh 顺序错误（设备类型不应该在语言之前）
    let result = Utils::analyze_qualifier("2in1-zh".to_string());
    assert_eq!(result.len(), 0, "设备类型在第一位应该失败");
  }

  #[test]
  fn test_analyze_qualifier_correct_order_language_device() {
    // zh-2in1 顺序正确
    let result = Utils::analyze_qualifier("zh-2in1".to_string());
    assert_eq!(result.len(), 2);
    assert!(matches!(result[0].qualifier_type, QualifierType::LanguageCode));
    assert_eq!(result[0].qualifier_value, "zh");
    assert!(matches!(result[1].qualifier_type, QualifierType::DeviceType));
    assert_eq!(result[1].qualifier_value, "2in1");
  }

  #[test]
  fn test_analyze_qualifier_mixed_valid_invalid() {
    // 混合有效和无效的限定词 - 应该导致整个解析失败
    let result = Utils::analyze_qualifier("zh_CN-invalid-phone-wrongcolor-mdpi".to_string());
    assert_eq!(result.len(), 0, "包含无效限定词应该导致解析失败");
  }

  #[test]
  fn test_analyze_qualifier_only_invalid() {
    // 只有无效的限定词 - 应该导致整个解析失败
    let result = Utils::analyze_qualifier("invalid-wrong-bad".to_string());
    assert_eq!(result.len(), 0, "全部无效应该返回空向量");
  }

  #[test]
  fn test_analyze_qualifier_multiple_dashes() {
    // 多个连续的中划线（空部分会被过滤）
    let result = Utils::analyze_qualifier("zh_CN--phone".to_string());
    assert_eq!(result.len(), 3);
    assert!(matches!(result[0].qualifier_type, QualifierType::LanguageCode));
    assert!(matches!(result[1].qualifier_type, QualifierType::RegionCode));
    assert!(matches!(result[2].qualifier_type, QualifierType::DeviceType));
  }

  #[test]
  fn test_analyze_qualifier_trailing_dash() {
    // 末尾有中划线（空部分会被过滤）
    let result = Utils::analyze_qualifier("zh_CN-phone-".to_string());
    assert_eq!(result.len(), 3);
    assert!(matches!(result[0].qualifier_type, QualifierType::LanguageCode));
    assert!(matches!(result[1].qualifier_type, QualifierType::RegionCode));
    assert!(matches!(result[2].qualifier_type, QualifierType::DeviceType));
  }

  #[test]
  fn test_analyze_qualifier_leading_dash() {
    // 开头有中划线（空部分会被过滤）
    let result = Utils::analyze_qualifier("-zh_CN-phone".to_string());
    assert_eq!(result.len(), 3);
    assert!(matches!(result[0].qualifier_type, QualifierType::LanguageCode));
    assert!(matches!(result[1].qualifier_type, QualifierType::RegionCode));
    assert!(matches!(result[2].qualifier_type, QualifierType::DeviceType));
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
    // 只有 MCC 没有 MNC（mcc460 不符合 mcc_mnc 格式，会被当作普通字符串处理）
    let result = Utils::analyze_qualifier("mcc460-zh_CN".to_string());
    // mcc460 单独出现无法识别为有效限定词
    assert_eq!(result.len(), 0, "单独的 MCC 应该无法识别");
  }

  #[test]
  fn test_analyze_qualifier_duplicate_qualifiers() {
    // 重复的限定词应该被拒绝（违反顺序规则）
    let result = Utils::analyze_qualifier("phone-phone".to_string());
    assert_eq!(result.len(), 0, "重复的限定词应该导致解析失败");
  }

  #[test]
  fn test_analyze_qualifier_correct_order_skip_stages() {
    // 测试跳过某些阶段
    let result1 = Utils::analyze_qualifier("zh-dark".to_string());
    assert_eq!(result1.len(), 2); // 语言 + 颜色（跳过方向和设备）

    let result2 = Utils::analyze_qualifier("phone-mdpi".to_string());
    assert_eq!(result2.len(), 2); // 设备 + 密度（跳过颜色）

    let result3 = Utils::analyze_qualifier("vertical-dark".to_string());
    assert_eq!(result3.len(), 2); // 方向 + 颜色（跳过设备）
  }
}
