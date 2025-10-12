use crate::{references::element_json_file_reference::ElementJsonFileReference, utils::uri::Uri};
#[cfg(not(test))]
use crate::element_directory::ElementDirectory;
#[cfg(not(test))]
use napi::bindgen_prelude::Reference;
#[cfg(not(test))]
use napi::Env;
#[cfg(not(test))]
use std::fs;
use napi_derive::napi;
use tree_sitter::Parser;

#[napi]
pub struct ElementJsonFile {
  parser: Parser,
  source_code: String,
  uri: Uri,
  #[cfg(not(test))]
  resource_directory: Reference<ElementDirectory>,
}

#[napi]
impl ElementJsonFile {
  #[cfg(test)]
  pub fn new(uri: &Uri, source_code: String) -> Self {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_json::LANGUAGE.into()).unwrap();

    Self { parser, source_code, uri: uri.clone() }
  }

  #[cfg(not(test))]
  pub fn new(resource_directory: Reference<ElementDirectory>, uri: &Uri, source_code: String) -> Self {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_json::LANGUAGE.into()).unwrap();

    Self { parser, source_code, uri: uri.clone(), resource_directory }
  }

  #[napi]
  #[cfg(not(test))]
  pub fn find_all(element_directory: Reference<ElementDirectory>, env: Env) -> Vec<ElementJsonFile> {
    let mut element_json_files = Vec::new();
    let resource_files = match fs::read_dir(&element_directory.get_uri().fs_path()) {
      Ok(resource_directories) => resource_directories
        .flatten()
        .filter(|entry| entry.metadata().map(|m| m.is_file()).unwrap_or(false))
        .filter(|entry| entry.path().extension().map(|extension| extension == "json").unwrap_or(false)),
      Err(_) => return element_json_files,
    };
    
    for file_entry in resource_files {
      let file_path = file_entry.path().to_string_lossy().to_string();
      let file_content = fs::read_to_string(file_path.clone()).unwrap_or_default();
      element_json_files.push(
        Self::new(
          element_directory.clone(env).unwrap(),
          &Uri::file(file_path),
          file_content,
        )
      );
    }

    element_json_files
  }

  #[napi]
  pub fn get_uri(&self) -> Uri {
    self.uri.clone()
  }

  #[napi]
  #[cfg(not(test))]
  pub fn get_resource_directory(&self, env: Env) -> Reference<ElementDirectory> {
    self.resource_directory.clone(env).unwrap()
  }

  #[napi]
  pub fn get_content(&self) -> String {
    self.source_code.clone()
  }

  #[napi]
  pub fn parse(&mut self) -> serde_json::Value {
    serde_json5::from_str(&self.source_code).unwrap()
  }


  #[napi]
  pub fn get_reference(&mut self) -> Vec<ElementJsonFileReference> {
    let mut reference = Vec::new();
    let tree = self.parser.parse(self.source_code.clone(), None).unwrap();

    for child in tree.root_node().children(&mut tree.root_node().walk()) {
      if child.kind() != "object" {
        continue;
      }

      for element_type_key in child.children(&mut child.walk()) {
        if element_type_key.kind() != "pair" {
          continue;
        }

        for element_type_value in element_type_key.children(&mut element_type_key.walk()) {
          if element_type_value.kind() != "array" {
            continue;
          }

          for element_name in element_type_value.children(&mut element_type_value.walk()) {
            if element_name.kind() != "object" {
              continue;
            }

            let mut name_start: Option<usize> = None;
            let mut name_end: Option<usize> = None;
            let mut name_text: Option<String> = None;
            let mut value_start: Option<usize> = None;
            let mut value_end: Option<usize> = None;
            let mut value_text: Option<String> = None;

            for element_name_key in element_name.children(&mut element_name.walk()) {
              if element_name_key.kind() != "pair" {
                continue;
              }

              let mut filtered_nodes = Vec::new();
              for element_name_key_item in element_name_key.children(&mut element_name_key.walk()) {
                if element_name_key_item.kind() != "string" {
                  continue;
                }
                filtered_nodes.push(element_name_key_item);
              }
              if filtered_nodes.len() != 2 {
                continue;
              }
              let key_text = match filtered_nodes[0].utf8_text(self.source_code.as_bytes()) {
                Ok(text) => text,
                Err(_) => continue,
              };
              if key_text.contains("\"name\"") {
                name_start = Some(filtered_nodes[1].start_byte());
                name_end = Some(filtered_nodes[1].end_byte());
                name_text = Some(match filtered_nodes[1].utf8_text(self.source_code.as_bytes()) {
                  Ok(text) => text.to_string(),
                  Err(_) => continue,
                });
              } else if key_text.contains("\"value\"") {
                value_start = Some(filtered_nodes[1].start_byte());
                value_end = Some(filtered_nodes[1].end_byte());
                value_text = Some(match filtered_nodes[1].utf8_text(self.source_code.as_bytes()) {
                  Ok(text) => text.to_string(),
                  Err(_) => continue,
                });
              } else {
                continue;
              }
            }

            if let (Some(name_start), Some(name_end), Some(name_text), Some(value_start), Some(value_end), Some(value_text)) =
              (name_start, name_end, name_text, value_start, value_end, value_text)
            {
              reference.push(ElementJsonFileReference::new(
                name_start as u32,
                name_end as u32,
                name_text,
                value_start as u32,
                value_end as u32,
                value_text,
              ))
            }
          }
        }
      }
    }

    reference
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_reference() {
    let mock_str = String::from("{ \"string\": [{ \"name\": \"test1\", \"value\": \"test1-value\" }] }");
    let mut element_json_file = ElementJsonFile::new(&Uri::file("test.json".to_string()), mock_str.clone());
    let references = element_json_file.get_reference();
    assert_eq!(references.len(), 1);
    assert_eq!(references[0].get_name_text(), "\"test1\"");
    assert_eq!(references[0].get_value_text(), "\"test1-value\"");
    assert_eq!(references[0].get_name_start(), 23);
    assert_eq!(references[0].get_name_end(), 30);
    assert_eq!(references[0].get_value_start(), 41);
    assert_eq!(references[0].get_value_end(), 54);
  }
}
