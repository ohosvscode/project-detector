use crate::resource_directory::ResourceDirectory;
use crate::utils::uri::Uri;
use napi::bindgen_prelude::Reference;
use napi::Env;
use napi_derive::napi;
use std::fs;
use std::path::Path;

#[napi]
pub struct ElementDirectory {
  uri: Uri,
  resource_directory: Reference<ResourceDirectory>,
}

#[napi]
impl ElementDirectory {
  #[napi]
  pub fn from(resource_directory: Reference<ResourceDirectory>) -> Option<ElementDirectory> {
    let uri = Uri::file(Path::new(&resource_directory.get_uri().fs_path()).join("element").to_string_lossy().to_string());
    if !fs::metadata(uri.fs_path()).map(|metadata| metadata.is_dir()).unwrap_or(false) {
      return None;
    }

    Some(Self { uri, resource_directory })
  }

  #[napi]
  pub fn get_uri(&self) -> Uri {
    self.uri.clone()
  }

  #[napi]
  pub fn get_resource_directory(&self, env: Env) -> Reference<ResourceDirectory> {
    self.resource_directory.clone(env).unwrap()
  }
}
