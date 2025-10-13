#[cfg(not(test))]
use napi::bindgen_prelude::Reference;
#[cfg(not(test))]
use napi::Env;
use napi_derive::napi;
use crate::files::element_json_file::ElementJsonFile;

#[napi]
pub struct ElementJsonFileReference {
  element_type: String,
  name_start: u32,
  name_end: u32,
  name_text: String,
  value_start: u32,
  value_end: u32,
  value_text: String,
  #[cfg(not(test))]
  element_json_file: Reference<ElementJsonFile>,
}

#[napi]
impl ElementJsonFileReference {
  #[cfg(not(test))]
  pub fn new(
    name_start: u32,
    name_end: u32,
    name_text: String,
    value_start: u32,
    value_end: u32,
    value_text: String,
    element_json_file: Reference<ElementJsonFile>,
    element_type: String,
  ) -> Self {
    Self {
      name_start,
      name_end,
      name_text,
      value_start,
      value_end,
      value_text,
      element_json_file,
      element_type,
    }
  }

  #[cfg(test)]
  pub fn new(
    name_start: u32,
    name_end: u32,
    name_text: String,
    value_start: u32,
    value_end: u32,
    value_text: String,
    element_type: String,
  ) -> Self {
    Self {
      name_start,
      name_end,
      name_text,
      value_start,
      value_end,
      value_text,
      element_type,
    }
  }

  fn byte_to_char_index(source_code: &String, byte_offset: usize) -> usize {
    source_code[..byte_offset].chars().count()
  }

  #[napi]
  #[cfg(not(test))]
  pub fn find_all(element_json_file: Reference<ElementJsonFile>, env: Env) -> Vec<ElementJsonFileReference> {
    let mut reference = Vec::new();
    let parser = element_json_file.get_parser();
    let source_code = element_json_file.get_content();
    let tree = parser.lock().unwrap().parse(&source_code, None).unwrap();

    for child in tree.root_node().children(&mut tree.root_node().walk()) {
      if child.kind() != "object" {
        continue;
      }

      for element_type_key in child.children(&mut child.walk()) {
        if element_type_key.kind() != "pair" {
          continue;
        }

        let mut current_element_type: String = String::new();
        for element_type_value in element_type_key.children(&mut element_type_key.walk()) {
          if element_type_value.kind() == "string" {
            current_element_type = match element_type_value.utf8_text(source_code.as_bytes()) {
              Ok(text) => text.to_string(),
              Err(_) => continue,
            };
            continue;
          }
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
              let key_text = match filtered_nodes[0].utf8_text(source_code.as_bytes()) {
                Ok(text) => text,
                Err(_) => continue,
              };
              if key_text =="\"name\"" {
                name_start = Some(Self::byte_to_char_index(&source_code, filtered_nodes[1].start_byte()));
                name_end = Some(Self::byte_to_char_index(&source_code, filtered_nodes[1].end_byte()));
                name_text = Some(match filtered_nodes[1].utf8_text(source_code.as_bytes()) {
                  Ok(text) => text.to_string(),
                  Err(_) => continue,
                });
              } else if key_text == "\"value\"" {
                value_start = Some(Self::byte_to_char_index(&source_code, filtered_nodes[1].start_byte()));
                value_end = Some(Self::byte_to_char_index(&source_code, filtered_nodes[1].end_byte()));
                value_text = Some(match filtered_nodes[1].utf8_text(source_code.as_bytes()) {
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
              reference.push(
                ElementJsonFileReference::new(
                  name_start as u32,
                  name_end as u32,
                  name_text,
                  value_start as u32,
                  value_end as u32,
                  value_text,
                  element_json_file.clone(env).unwrap(),
                  current_element_type.clone(),
                )
              )
            }
          }
        }
      }
    }

    reference
  }

  #[cfg(test)]
  pub fn find_all(element_json_file: ElementJsonFile) -> Vec<ElementJsonFileReference> {
    let mut reference = Vec::new();
    let parser = element_json_file.get_parser();
    let source_code = element_json_file.get_content();
    let tree = parser.lock().unwrap().parse(&source_code, None).unwrap();

    for child in tree.root_node().children(&mut tree.root_node().walk()) {
      if child.kind() != "object" {
        continue;
      }

      for element_type_key in child.children(&mut child.walk()) {
        if element_type_key.kind() != "pair" {
          continue;
        }

        let mut current_element_type: String = String::new();
        for element_type_value in element_type_key.children(&mut element_type_key.walk()) {
          if element_type_value.kind() == "string" {
            current_element_type = match element_type_value.utf8_text(source_code.as_bytes()) {
              Ok(text) => text.to_string(),
              Err(_) => continue,
            };
            continue;
          }
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
              let key_text = match filtered_nodes[0].utf8_text(source_code.as_bytes()) {
                Ok(text) => text,
                Err(_) => continue,
              };
              if key_text =="\"name\"" {
                name_start = Some(Self::byte_to_char_index(&source_code, filtered_nodes[1].start_byte()));
                name_end = Some(Self::byte_to_char_index(&source_code, filtered_nodes[1].end_byte()));
                name_text = Some(match filtered_nodes[1].utf8_text(source_code.as_bytes()) {
                  Ok(text) => text.to_string(),
                  Err(_) => continue,
                });
              } else if key_text == "\"value\"" {
                value_start = Some(Self::byte_to_char_index(&source_code, filtered_nodes[1].start_byte()));
                value_end = Some(Self::byte_to_char_index(&source_code, filtered_nodes[1].end_byte()));
                value_text = Some(match filtered_nodes[1].utf8_text(source_code.as_bytes()) {
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
              reference.push(
                ElementJsonFileReference::new(
                  name_start as u32,
                  name_end as u32,
                  name_text,
                  value_start as u32,
                  value_end as u32,
                  value_text,
                  current_element_type.clone(),
                )
              )
            }
          }
        }
      }
    }

    reference
  }

  #[napi]
  #[cfg(not(test))]
  pub fn get_element_json_file(&self, env: Env) -> Reference<ElementJsonFile> {
    self.element_json_file.clone(env).unwrap()
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
    let s = self.name_text.as_str();
    let s = if s.starts_with('"') { &s[1..] } else { s };
    let s = if s.ends_with('"') { &s[..s.len() - 1] } else { s };
    s.to_string()
  }

  #[napi]
  pub fn get_name_full_text(&self) -> String {
    self.name_text.clone()
  }

  #[napi]
  pub fn get_value_text(&self) -> String {
    let s = self.value_text.as_str();
    let s = if s.starts_with('"') { &s[1..] } else { s };
    let s = if s.ends_with('"') { &s[..s.len() - 1] } else { s };
    s.to_string()
  }

  #[napi]
  pub fn get_value_full_text(&self) -> String {
    self.value_text.clone()
  }

  #[napi]
  pub fn get_element_type(&self) -> String {
    let s = self.element_type.as_str();
    let s = if s.starts_with('"') { &s[1..] } else { s };
    let s = if s.ends_with('"') { &s[..s.len() - 1] } else { s };
    s.to_string()
  }

  #[napi]
  pub fn get_full_element_type(&self) -> String {
    self.element_type.clone()
  }

  #[napi]
  pub fn to_ets_format(&self) -> String {
    let text = self.get_name_text();
    return format!("app.{}.{}", self.get_element_type(), text);
  }

  #[napi]
  pub fn to_json_format(&self) -> String {
    let text = self.get_name_text();
    return format!("${}:{}", self.get_element_type(), text);
  }
}
