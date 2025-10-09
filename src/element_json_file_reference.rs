use crate::element_json_file::ElementJsonFile;
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;
use tree_sitter::{Parser, TreeCursor};

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

  #[napi]
  pub fn find_all(env: Env, element_json_file: Reference<ElementJsonFile>) -> Vec<ElementJsonFileNameReference> {
    Self::extract_name_references(&element_json_file.content, &element_json_file, env)
  }

  /// 提取 JSON 中所有 "name" 字段值的精确位置
  fn extract_name_references(content: &str, element_json_file: &Reference<ElementJsonFile>, env: Env) -> Vec<ElementJsonFileNameReference> {
    let mut references = Vec::new();

    // 初始化 parser
    let mut parser = Parser::new();
    let language = tree_sitter_json::LANGUAGE;

    if parser.set_language(&language.into()).is_err() {
      return references;
    }

    // 解析 JSON
    let tree = match parser.parse(content, None) {
      Some(tree) => tree,
      None => return references,
    };

    // 遍历语法树查找所有 "name" 字段
    let mut cursor = tree.root_node().walk();
    Self::find_name_fields(&mut cursor, content, &mut references, element_json_file, &env);

    references
  }

  /// 递归查找 JSON 中所有 "name" 字段
  fn find_name_fields(cursor: &mut TreeCursor, content: &str, references: &mut Vec<ElementJsonFileNameReference>, element_json_file: &Reference<ElementJsonFile>, env: &Env) {
    loop {
      let node = cursor.node();

      // 检查是否是键值对节点
      if node.kind() == "pair" {
        // 获取 key 和 value 节点
        if let Some(key_node) = node.child_by_field_name("key") {
          if let Ok(key_text) = key_node.utf8_text(content.as_bytes()) {
            // 检查键是否为 "name"
            if key_text == "\"name\"" {
              if let Some(value_node) = node.child_by_field_name("value") {
                // 确保值是字符串类型
                if value_node.kind() == "string" {
                  let text = &content[value_node.start_byte()..value_node.end_byte()];

                  // 去掉引号
                  let actual_text = text.trim_matches('"');
                  let start = value_node.start_byte() as u32 + 1; // 跳过开头引号
                  let end = value_node.end_byte() as u32 - 1; // 跳过结尾引号

                  references.push(ElementJsonFileNameReference::create(
                    start,
                    end,
                    actual_text.to_string(),
                    element_json_file.clone(*env).unwrap(),
                  ));
                }
              }
            }
          }
        }
      }

      // 递归遍历子节点
      if cursor.goto_first_child() {
        Self::find_name_fields(cursor, content, references, element_json_file, env);
        cursor.goto_parent();
      }

      // 移动到下一个兄弟节点
      if !cursor.goto_next_sibling() {
        break;
      }
    }
  }
}