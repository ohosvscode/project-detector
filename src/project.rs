use std::{fs, path::Path, sync::Arc};
use napi::Env;
use napi::bindgen_prelude::Reference;
use napi_derive::napi;
use url::Url;
use crate::project_detector::ProjectDetector;
use glob::glob;

#[napi]
pub struct Project {
  build_profile_content: String,
  parsed_build_profile_content: serde_json::Value,
  project_detector: Reference<ProjectDetector>,
  project_folder: Arc<Url>
}

#[napi]
impl Project {
  /**
   * Create a new project.
   *
   * @param project_detector - The project detector.
   * @param directory_uri - The directory URI.
   * @returns The project.
   */
  #[napi]
  pub fn create(project_detector: Reference<ProjectDetector>, directory_uri: String) -> Option<Project> {
    let workspace_folder = project_detector.get_workspace_folder();
    let parsed_directory_url = Url::parse(&directory_uri);
    if parsed_directory_url.is_err() {
      return None;
    }
    let unwrapped_directory_url = match parsed_directory_url {
      Ok(url) => url,
      Err(_) => return None,
    };
    let directory_file_path = match unwrapped_directory_url.to_file_path() {
      Ok(path) => match path.to_str() {
        Some(path) => path.to_string(),
        None => return None,
      },
      Err(_) => return None,
    };
    let full_path = match Path::join(Path::new(&workspace_folder), Path::new(&directory_file_path)).to_str() {
      Some(path) => path.to_string(),
      None => return None,
    };
    
    // build-profile.json5 file
    let build_profile_path = match Path::join(Path::new(&full_path), Path::new("build-profile.json5")).to_str() {
      Some(path) => path.to_string(),
      None => return None,
    };
    let build_profile_content = match fs::read_to_string(build_profile_path) {
      Ok(content) => content,
      Err(_) => return None,
    };
    let parsed_build_profile_content = match serde_json5::from_str::<serde_json::Value>(&build_profile_content) {
      Ok(value) => value,
      Err(_) => return None,
    };

    let app_field_is_object = match parsed_build_profile_content.get("app") {
      Some(value) => value.is_object(),
      None => false,
    };
    let modules_field_is_array = match parsed_build_profile_content.get("modules") {
      Some(value) => value.is_array(),
      None => false,
    };

    // workspace build-profile.json5 file must have `app` field and `modules` field
    if app_field_is_object != true || modules_field_is_array != true {
      return None;
    }

    Some(
      Project {
        build_profile_content,
        parsed_build_profile_content,
        project_detector,
        project_folder: Arc::new(unwrapped_directory_url),
      }
    )
  }

  /**
   * Get the build profile content.
   *
   * @returns The build profile content.
   */
  #[napi]
  pub fn get_build_profile_content(&self) -> String {
    self.build_profile_content.clone()
  }

  /**
   * Get the parsed build profile content.
   *
   * @returns The parsed build profile content.
   */
  #[napi(ts_return_type = "unknown")]
  pub fn get_parsed_build_profile_content(&self) -> serde_json::Value {
    self.parsed_build_profile_content.clone()
  }

  /**
   * Get the project detector.
   *
   * @param env - The environment.
   * @returns The project detector.
   */
  #[napi]
  pub fn get_project_detector(&self, env: Env) -> Reference<ProjectDetector> {
    self.project_detector.clone(env).unwrap()
  }

  /**
   * Get the project folder URL.
   *
   * @returns The project folder URL.
   */
  pub fn get_project_folder_url(&self) -> Arc<Url> {
    self.project_folder.clone()
  }

  /**
   * Find all projects.
   *
   * @param env - The environment.
   * @param project_detector - The project detector.
   * @returns The projects.
   */
  #[napi]
  pub fn find_all(env: Env, project_detector: Reference<ProjectDetector>) -> Vec<Project> {
    let mut projects = Vec::new();
    let workspace_folder = project_detector.get_workspace_folder_url();
    let glob_pattern = match Path::join(Path::new(&workspace_folder.to_file_path().unwrap()), Path::new("**/build-profile.json5")).to_str() {
      Some(path) => path.to_string(),
      None => return projects,
    };
    let glob_result = match glob(&glob_pattern) {
      Ok(result) => result,
      Err(_) => return projects,
    };

    for entry in glob_result {
      let path = match entry {
        Ok(p) => p,
        Err(_) => continue,
      };

      let metadata = match path.metadata() {
        Ok(m) => m,
        Err(_) => continue,
      };
      if metadata.is_file() != true {
        continue;
      }

      let directory_path = match path.parent() {
        Some(p) => p,
        None => continue,
      };

      let directory_url = match Url::from_directory_path(directory_path) {
        Ok(url) => url.to_string(),
        Err(_) => continue,
      };

      if directory_url.contains("oh_modules") || directory_url.contains("node_modules") {
        continue;
      }

      if let Some(project) = Project::create(project_detector.clone(env).unwrap(), directory_url) {
        projects.push(project);
      }
    }

    projects
  }
}