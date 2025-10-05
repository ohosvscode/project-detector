use std::{fs, path::Path};

use napi_derive::napi;
use url::Url;
use crate::{element_json_file::ElementJsonFile, resource::{Resource, ResourceQualifiedDirectoryType}};

#[napi]
#[derive(Debug)]
pub struct ResourceGroup {
  uri: Url,
  is_base: bool,
}

#[napi]
impl ResourceGroup {
  #[napi]
  pub fn get_uri(&self) -> String {
    path_clean::clean(&self.uri.to_string())
  }

  #[napi]
  pub fn find_all(resources: Vec<&Resource>) -> Vec<ResourceGroup> {
    let mut resource_groups = Vec::new();

    for resource_directory in resources {
      let qualified_directories = resource_directory.get_qualified_directories();

      for qualified_directory in qualified_directories {
        if qualified_directory.directory_type == ResourceQualifiedDirectoryType::Qualified || qualified_directory.directory_type == ResourceQualifiedDirectoryType::Base {
          let uri = match Url::parse(&qualified_directory.uri) {
            Ok(url) => url,
            Err(_) => continue,
          };

          resource_groups.push(
            ResourceGroup {
              uri,
              is_base: qualified_directory.directory_type == ResourceQualifiedDirectoryType::Base,
            }
          )
        }
      }
    }

    resource_groups
  }

  #[napi]
  pub fn is_base(&self) -> bool {
    self.is_base
  }

  #[napi]
  pub fn get_element_json_files(&self) -> Vec<ElementJsonFile> {
    let mut element_json_files = Vec::new();
    let element_dir_path = match self.uri.to_file_path() {
      Ok(path) => Path::new(&path).join("element"),
      Err(_) => return element_json_files,
    };
    let files = match fs::read_dir(element_dir_path) {
      Ok(files) => files,
      Err(_) => return element_json_files,
    };

    for file in files {
      let entry = match file {
        Ok(file) => match file.metadata() {
          Ok(metadata) => match metadata.is_file() {
            true => file,
            false => continue,
          },
          Err(_) => continue,
        },
        Err(_) => continue,
      };

      let file_name = entry.file_name().to_string_lossy().to_string();
      if !file_name.ends_with(".json") {
        continue;
      }

      let file_uri = match Url::from_directory_path(entry.path().to_string_lossy().to_string()) {
        Ok(url) => url,
        Err(_) => continue,
      };

      if let Some(element_json_file) = ElementJsonFile::create(file_uri.to_string()) {
        element_json_files.push(element_json_file);
      }
    }

    element_json_files
  }
}