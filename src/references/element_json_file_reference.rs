use napi_derive::napi;

#[napi]
pub struct ElementJsonFileReference {
  name_start: u32,
  name_end: u32,
  name_text: String,
  value_start: u32,
  value_end: u32,
  value_text: String,
}

#[napi]
impl ElementJsonFileReference {
  pub fn new(name_start: u32, name_end: u32, name_text: String, value_start: u32, value_end: u32, value_text: String) -> Self {
    Self {
      name_start,
      name_end,
      name_text,
      value_start,
      value_end,
      value_text,
    }
  }

  #[napi]
  pub fn get_name_start(&self) -> u32 {
    self.name_start
  }

  #[napi]
  pub fn get_name_end(&self) -> u32 {
    self.name_end
  }

  #[napi]
  pub fn get_value_start(&self) -> u32 {
    self.value_start
  }

  #[napi]
  pub fn get_value_end(&self) -> u32 {
    self.value_end
  }

  #[napi]
  pub fn get_name_text(&self) -> String {
    self.name_text.clone()
  }

  #[napi]
  pub fn get_value_text(&self) -> String {
    self.value_text.clone()
  }
}
