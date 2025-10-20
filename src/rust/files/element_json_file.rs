#[cfg(not(test))]
use crate::element_directory::ElementDirectory;
use crate::utils::uri::Uri;
#[cfg(not(test))]
use napi::bindgen_prelude::Reference;
#[cfg(not(test))]
use napi::Env;
use napi_derive::napi;
use std::fs;
use std::sync::{Arc, Mutex};
use tree_sitter::Parser;

#[napi]
pub struct ElementJsonFile {
  parser: Arc<Mutex<Parser>>,
  source_code: String,
  uri: Uri,
  #[cfg(not(test))]
  element_directory: Reference<ElementDirectory>,
}

#[napi]
impl ElementJsonFile {
  #[cfg(test)]
  pub fn create(element_json_file_uri: String, source_code: String) -> Self {
    let uri = Uri::file(element_json_file_uri);
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_json::LANGUAGE.into()).unwrap();

    Self {
      parser: Arc::new(Mutex::new(parser)),
      source_code,
      uri: uri.clone(),
    }
  }

  #[napi]
  #[cfg(not(test))]
  pub fn create(element_directory: Reference<ElementDirectory>, element_json_file_uri: String) -> Option<ElementJsonFile> {
    let uri = Uri::file(element_json_file_uri);
    if !uri.fs_path().ends_with(".json") {
      return None;
    }

    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_json::LANGUAGE.into()).unwrap();
    if !fs::metadata(uri.fs_path()).map(|m| m.is_file()).unwrap_or(false) {
      return None;
    }

    Some(Self {
      parser: Arc::new(Mutex::new(parser)),
      source_code: fs::read_to_string(uri.fs_path()).unwrap_or_default(),
      uri,
      element_directory,
    })
  }

  #[napi]
  pub fn reload(element_json_file: &mut ElementJsonFile) {
    element_json_file.set_content(fs::read_to_string(element_json_file.get_uri().fs_path()).unwrap_or_default());
  }

  #[napi]
  #[cfg(not(test))]
  pub fn find_all(element_directory: Reference<ElementDirectory>, env: Env) -> Vec<ElementJsonFile> {
    let mut element_json_files = Vec::new();
    let resource_files = match fs::read_dir(element_directory.get_uri().fs_path()) {
      Ok(resource_directories) => resource_directories
        .flatten()
        .filter(|entry| entry.metadata().map(|m| m.is_file()).unwrap_or(false))
        .filter(|entry| entry.path().extension().map(|extension| extension == "json").unwrap_or(false)),
      Err(_) => return element_json_files,
    };

    for file_entry in resource_files {
      let file_path = file_entry.path().to_string_lossy().to_string();
      if let Some(element_json_file) = Self::create(element_directory.clone(env).unwrap(), file_path) {
        element_json_files.push(element_json_file);
      }
    }

    element_json_files
  }

  #[napi]
  pub fn get_uri(&self) -> Uri {
    self.uri.clone()
  }

  #[napi]
  #[cfg(not(test))]
  pub fn get_element_directory(&self, env: Env) -> Reference<ElementDirectory> {
    self.element_directory.clone(env).unwrap()
  }

  #[napi]
  pub fn get_content(&self) -> String {
    self.source_code.clone()
  }

  pub fn set_content(&mut self, source_code: String) {
    self.source_code = source_code;
  }

  #[napi]
  pub fn parse(&mut self) -> serde_json::Value {
    serde_json5::from_str(&self.source_code).unwrap()
  }

  pub fn get_parser(&self) -> Arc<Mutex<Parser>> {
    Arc::clone(&self.parser)
  }
}

#[cfg(test)]
mod tests {
  use crate::references::element_json_file_reference::ElementJsonFileReference;

  use super::*;

  fn slice(s: &str, start: usize, end_exclusive: usize) -> String {
    let mut byte_start = 0usize;
    let mut byte_end = s.len();
    for (i, (bpos, _)) in s.char_indices().enumerate() {
      if i == start {
        byte_start = bpos;
      }
      if i == end_exclusive {
        byte_end = bpos;
        break;
      }
    }
    s[byte_start..byte_end].to_string()
  }

  #[test]
  fn test_get_reference() {
    let mock_str = String::from("{ \"string\": [{ \"name\": \"test1\", \"value\": \"test1-value\" }] }");
    let element_json_file = ElementJsonFile::create("test.json".to_string(), mock_str.clone());
    let references = ElementJsonFileReference::find_all(element_json_file);
    assert_eq!(references.len(), 1);
    assert_eq!(references[0].get_name_full_text(), "\"test1\"");
    assert_eq!(references[0].get_value_full_text(), "\"test1-value\"");
    assert_eq!(
      slice(&mock_str, references[0].get_name_start() as usize, references[0].get_name_end() as usize),
      references[0].get_name_full_text()
    );
    assert_eq!(
      slice(&mock_str, references[0].get_value_start() as usize, references[0].get_value_end() as usize),
      references[0].get_value_full_text()
    );
  }
}
