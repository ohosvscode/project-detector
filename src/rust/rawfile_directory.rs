use crate::{resource::Resource, utils::uri::Uri};
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;
use std::{fs, path::Path};
use walkdir::WalkDir;

#[napi]
pub struct RawfileDirectory {
  uri: Uri,
  resource: Reference<Resource>,
}

#[napi]
impl RawfileDirectory {
  #[napi]
  pub fn from(resource: Reference<Resource>) -> Option<RawfileDirectory> {
    let uri = Uri::file(Path::new(&resource.get_uri().fs_path()).join("rawfile").to_string_lossy().to_string());
    if !fs::metadata(uri.fs_path()).map(|metadata| metadata.is_dir()).unwrap_or(false) {
      return None;
    }

    Some(RawfileDirectory { uri, resource })
  }

  #[napi]
  pub fn get_uri(&self) -> Uri {
    self.uri.clone()
  }

  #[napi]
  pub fn get_resource(&self, env: Env) -> Reference<Resource> {
    self.resource.clone(env).unwrap()
  }

  #[napi]
  pub fn find_all(&self) -> Vec<Uri> {
    let rawfile_directory = self.get_uri();
    WalkDir::new(rawfile_directory.fs_path())
      .into_iter()
      .filter_map(|res| res.ok())
      .filter(|entry| entry.file_type().is_file())
      .map(|entry| Uri::file(entry.path().to_string_lossy().to_string()))
      .collect()
  }
}
