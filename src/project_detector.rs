use napi_derive::napi;
use url::Url;

#[napi]
#[derive(Clone)]
pub struct ProjectDetector {
  workspace_folder: Url,
}

#[napi]
impl ProjectDetector {
  /**
   * Create a new project detector.
   *
   * @param workspace_folder - The workspace folder path, must be a valid URL like `file://`.
   * @returns The project detector.
   */
  #[napi]
  pub fn create(workspace_folder: String) -> Self {
    Self {
      workspace_folder: Url::parse(&workspace_folder).unwrap(),
    }
  }

  /**
   * Get the workspace folder.
   *
   * @returns The workspace folder.
   */
  #[napi]
  pub fn get_workspace_folder(&self) -> String {
    self.workspace_folder.to_string()
  }

  /**
   * Get the workspace folder URL.
   *
   * @returns The workspace folder URL.
   */
  pub fn get_workspace_folder_url(&self) -> Url {
    self.workspace_folder.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create() {
    let workspace_folder = "mock/workspace";
    let cwd = std::env::current_dir().unwrap();
    let resolved_workspace_folder = std::path::Path::join(&cwd, workspace_folder);
    let resolved_file_url = Url::from_file_path(&resolved_workspace_folder).unwrap();
    let project_detector = ProjectDetector::create(resolved_file_url.to_string());
    assert_eq!(project_detector.get_workspace_folder(), resolved_file_url.to_string());
  }
}
