use std::{fs, path::Path};
use crate::{resource_directory::ResourceDirectory, utils::uri::Uri};
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;

#[napi]
pub struct ProfileDirectory {
  uri: Uri,
  resource_directory: Reference<ResourceDirectory>,
}

#[napi]
impl ProfileDirectory {
  #[napi]
  pub fn from(resource_directory: Reference<ResourceDirectory>) -> Option<ProfileDirectory> {
    let uri = Uri::file(Path::new(&resource_directory.get_uri().fs_path()).join("profile").to_string_lossy().to_string());
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
    let mut profile_directories = Vec::new();
    let profiles = match fs::read_dir(self.uri.fs_path()) {
      Ok(profiles) => profiles,
      Err(_) => return profile_directories,
    };

    for profile in profiles.flatten() {
      if profile.metadata().map(|metadata| metadata.is_dir()).unwrap_or(false) {
        profile_directories.push(Uri::file(profile.path().to_string_lossy().to_string()));
      }
    }

    profile_directories
  }
}