use crate::utils::uri::Uri;
#[cfg(not(test))]
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;

#[napi]
pub struct ProjectDetector {
  #[cfg(not(test))]
  workspace_folder: Uri,
  #[cfg(test)]
  workspace_folder: Uri,
}

#[napi]
impl ProjectDetector {
  #[napi]
  #[cfg(not(test))]
  pub fn create(workspace_folder: Reference<String>, env: Env) -> Self {
    Self {
      workspace_folder: Uri::parse(workspace_folder.clone(env).unwrap().as_str().to_string()),
    }
  }

  #[napi]
  #[cfg(test)]
  pub fn create(workspace_folder: String) -> Self {
    Self {
      workspace_folder: Uri::parse(workspace_folder),
    }
  }

  #[napi]
  #[cfg(not(test))]
  pub fn get_workspace_folder(&self) -> Uri {
    self.workspace_folder.clone()
  }

  #[cfg(test)]
  pub fn get_workspace_folder(&self) -> Uri {
    self.workspace_folder.clone()
  }
}
