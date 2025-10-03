use napi_derive::napi;
use crate::utils::color_mode::ColorMode;
use crate::utils::device_type::DeviceType;
use crate::utils::language_code::LanguageCode;
use crate::utils::mcc::MCC;
use crate::utils::mnc::MNC;

#[napi]
pub struct Utils {}

#[napi]
impl Utils {
  #[napi]
  pub fn is_mcc(mcc: u32) -> bool {
    MCC::is(mcc)
  }

  #[napi]
  pub fn is_mcc_code(mcc: String) -> bool {
    MCC::is_code(mcc)
  }

  #[napi]
  pub fn is_language_code(language_code: String) -> bool {
    LanguageCode::is(language_code)
  }

  #[napi]
  pub fn is_device_type(device_type: String) -> bool {
    DeviceType::is(device_type)
  }

  #[napi]
  pub fn is_color_mode(color_mode: String) -> bool {
    ColorMode::is(color_mode)
  }

  #[napi]
  pub fn is_mnc(mnc: u32, mcc: u32) -> bool {
    MNC::is(mcc, mnc)
  }

  #[napi]
  pub fn is_mnc_code(mnc: String, mcc: u32) -> bool {
    MNC::is_code(mnc, mcc)
  }
}