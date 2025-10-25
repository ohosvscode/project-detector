use crate::{resource_directory::ResourceDirectory, utils::uri::Uri};
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;
use std::{fs, path::Path};

#[napi]
pub struct MediaDirectory {
  uri: Uri,
  resource_directory: Reference<ResourceDirectory>,
}

#[napi]
impl MediaDirectory {
  #[napi]
  pub fn from(resource_directory: Reference<ResourceDirectory>) -> Option<MediaDirectory> {
    let uri = Uri::file(Path::new(&resource_directory.get_uri().fs_path()).join("media").to_string_lossy().to_string());
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

  #[napi]
  pub fn find_all(&self) -> Vec<Uri> {
    let mut media_files = Vec::new();
    let media_directory = self.get_uri();
    let dirs = match fs::read_dir(media_directory.fs_path()) {
      Ok(dirs) => dirs,
      Err(_) => return media_files,
    };

    for dir in dirs.flatten() {
      if dir.metadata().map(|metadata| metadata.is_file()).unwrap_or(false) {
        media_files.push(Uri::file(dir.path().to_string_lossy().to_string()));
      }
    }

    media_files
  }
}
