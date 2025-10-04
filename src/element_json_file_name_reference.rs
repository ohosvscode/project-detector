use napi_derive::napi;
use napi::{bindgen_prelude::Reference, Env};
use crate::element_json_file::ElementJsonFile;

#[napi]
pub struct ElementJsonFileNameReference {
  start: u32,
  end: u32,
  text: String,
  element_json_file: Reference<ElementJsonFile>,
}

#[napi]
impl ElementJsonFileNameReference {
  #[napi]
  pub fn get_start(&self) -> u32 {
    self.start
  }

  #[napi]
  pub fn get_end(&self) -> u32 {
    self.end
  }

  #[napi]
  pub fn get_text(&self) -> String {
    self.text.clone()
  }

  #[napi]
  pub fn create(start: u32, end: u32, text: String, element_json_file: Reference<ElementJsonFile>) -> Self {
    Self {
      start,
      end,
      text,
      element_json_file,
    }
  }

  #[napi]
  pub fn get_element_json_file(&self, env: Env) -> Reference<ElementJsonFile> {
    self.element_json_file.clone(env).unwrap()
  }
}