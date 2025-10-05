use url::Url;
use napi_derive::napi;
use std::fs;

#[napi]
#[derive(Clone)]
pub struct ElementJsonFile {
  uri: Url,
  pub content: String,
}

#[napi]
impl ElementJsonFile {
  #[napi]
  pub fn create(uri: String) -> Option<ElementJsonFile> {
    let uri = match Url::parse(&uri) {
      Ok(url) => url,
      Err(_) => return None,
    };
    let file_path = match uri.to_file_path() {
      Ok(path) => path_clean::clean(&path.to_string_lossy()),
      Err(_) => return None,
    };
    let content = match fs::read_to_string(file_path) {
      Ok(content) => content,
      Err(_) => return None,
    };

    let element_json_file = ElementJsonFile { 
      uri,
      content,
    };

    Some(element_json_file)
  }

  #[napi]
  pub fn get_uri(&self) -> String {
    path_clean::clean(&self.uri.to_string())
  }
}
