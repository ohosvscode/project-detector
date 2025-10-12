use crate::utils::uri::Uri;
use napi_derive::napi;

#[napi]
#[derive(Clone)]
pub struct ProjectDetector {
  workspace_folder: Uri,
}

#[napi]
impl ProjectDetector {
  #[napi]
  pub fn create(workspace_folder: String) -> Self {
    Self {
      workspace_folder: Uri::parse(workspace_folder),
    }
  }

  #[napi]
  pub fn get_workspace_folder(&self) -> Uri {
    self.workspace_folder.clone()
  }
}
