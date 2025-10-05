use std::fs;
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;
use crate::{product::Product, utils::utils::{Qualifier, Utils}};
use url::Url;

#[napi]
pub struct Resource {
  uri: Url,
  product: Reference<Product>,
}

#[napi]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceQualifiedDirectoryType {
  RawFile,
  ResFile,
  Base,
  Qualified,
}

#[napi(object)]
pub struct ResourceQualifiedDirectory {
  pub uri: String,
  pub qualifiers: Vec<Qualifier>,
  pub directory_type: ResourceQualifiedDirectoryType,
}

#[napi]
impl Resource {
  #[napi]
  pub fn get_uri(&self) -> String {
    path_clean::clean(&self.uri.to_string())
  }

  #[napi]
  pub fn create(product: Reference<Product>, directory_uri: String) -> Option<Resource> {
    let uri = match Url::parse(&directory_uri) {
      Ok(url) => url,
      Err(_) => return None,
    };

    Some(
      Resource {
        uri,
        product,
      }
    )
  }

  #[napi]
  pub fn get_product(&self, env: Env) -> Reference<Product> {
    self.product.clone(env).unwrap()
  }

  #[napi]
  pub fn find_all(env: Env, product: Reference<Product>) -> Vec<Resource> {
    let mut resources = Vec::new();

    for resource_directory in product.get_resource_directories(env) {
      if let Some(resource) = Resource::create(product.clone(env).unwrap(), resource_directory) {
        resources.push(resource);
      }
    }

    resources
  }

  /**
   * Get qualified directories.
   *
   * @returns The qualified directories uris.
   */
  #[napi]
  pub fn get_qualified_directories(&self) -> Vec<ResourceQualifiedDirectory> {
    let mut qualified_directories = Vec::new();
    let directory = match self.uri.to_file_path() {
      Ok(path) => path,
      Err(_) => return qualified_directories,
    };

    let dir = match fs::read_dir(directory) {
      Ok(dir) => dir,
      Err(_) => return qualified_directories,
    };

    for entry in dir {
      let entry = match entry {
        Ok(entry) => entry,
        Err(_) => continue,
      };

      let metadata = match entry.metadata() {
        Ok(metadata) => metadata,
        Err(_) => continue,
      };

      if !metadata.is_dir() {
        continue;
      }

      let file_name = entry.file_name();
      let qualifiers = Utils::analyze_qualifier(file_name.to_string_lossy().to_string());
      let file_name_str = match file_name.to_str() {
        Some(file_name) => file_name,
        None => continue,
      };

      if file_name_str != "base" && file_name_str != "rawfile" && file_name_str != "resfile" && qualifiers.len() == 0 {
        continue;
      }
      
      let directory_uri = match Url::from_directory_path(entry.path().to_string_lossy().to_string()) {
        Ok(url) => url,
        Err(_) => continue,
      };

      qualified_directories.push(
        ResourceQualifiedDirectory {
          uri: directory_uri.to_string(),
          qualifiers,
          directory_type: match file_name_str {
            "base" => ResourceQualifiedDirectoryType::Base,
            "rawfile" => ResourceQualifiedDirectoryType::RawFile,
            "resfile" => ResourceQualifiedDirectoryType::ResFile,
            _ => ResourceQualifiedDirectoryType::Qualified,
          },
        }
      );
    }

    qualified_directories
  }
}