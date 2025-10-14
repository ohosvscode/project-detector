use std::fs;

use crate::{
  product::Product,
  utils::{qualifier::utils_impl::QualifierUtils, uri::Uri},
};
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;

#[napi]
pub struct Resource {
  product: Reference<Product>,
  uri: Uri,
}

#[napi]
impl Resource {
  #[napi]
  pub fn find_all(product: Reference<Product>, env: Env) -> Vec<Resource> {
    let mut resources = Vec::new();
    let resource_directories = product.get_resource_directories();

    for resource_directory in resource_directories {
      resources.push(Resource {
        product: product.clone(env).unwrap(),
        uri: resource_directory,
      })
    }

    resources
  }

  #[napi]
  pub fn get_product(&self, env: Env) -> Reference<Product> {
    self.product.clone(env).unwrap()
  }

  #[napi]
  pub fn get_uri(&self) -> Uri {
    self.uri.clone()
  }

  #[napi]
  pub fn get_qualified_directories(&self) -> Vec<Uri> {
    let mut qualified_directories = Vec::new();
    let resource_directory = self.get_uri();

    let dirs = match fs::read_dir(resource_directory.fs_path()) {
      Ok(dirs) => dirs,
      Err(_) => return qualified_directories,
    };

    for dir in dirs.flatten() {
      if dir.metadata().map(|metadata| metadata.is_dir()).unwrap_or(false) {
        let dir_name = dir.file_name().to_string_lossy().to_string();
        if dir_name != "base" && dir_name != "rawfile" && dir_name != "resfile" && QualifierUtils::analyze_qualifier(dir_name).is_empty() {
          continue;
        }
        qualified_directories.push(Uri::file(dir.path().to_string_lossy().to_string()))
      }
    }

    qualified_directories
  }
}
