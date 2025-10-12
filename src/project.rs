use crate::project_detector::ProjectDetector;
use crate::utils::uri::Uri;
#[cfg(not(test))]
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;
#[cfg(not(test))]
use std::fs;
#[cfg(not(test))]
use walkdir::WalkDir;

#[napi]
pub struct Project {
  #[cfg(not(test))]
  project_detector: Reference<ProjectDetector>,
  #[cfg(test)]
  project_detector: ProjectDetector,
  uri: Uri,
  parsed_build_profile: serde_json::Value,
  build_profile_uri: Uri,
  build_profile_content: String,
}

#[napi]
impl Project {
  #[napi]
  #[cfg(not(test))]
  pub fn get_project_detector(&self, env: Env) -> Reference<ProjectDetector> {
    self.project_detector.clone(env).unwrap()
  }

  #[cfg(test)]
  pub fn get_project_detector(&self) -> ProjectDetector {
    self.project_detector.clone()
  }

  #[napi]
  #[cfg(not(test))]
  pub fn find_all(project_detector: Reference<ProjectDetector>, env: Env) -> Vec<Project> {
    let mut projects = Vec::new();
    // 扫描workspace folder下的所有build-profile.json5文件
    let workspace_folder = project_detector.get_workspace_folder().fs_path();
    let dir_entries = WalkDir::new(workspace_folder).into_iter().filter_map(|entry| match entry {
      Ok(entry) => match entry.path().is_file() && entry.path().file_name().unwrap_or_default() == "build-profile.json5" {
        true => Some(entry),
        false => None,
      },
      Err(_) => None,
    });

    for entry in dir_entries {
      let file_path = entry.path().to_string_lossy().to_string();
      let file_content = fs::read_to_string(file_path.clone()).unwrap_or_default();
      let build_profile: serde_json::Value = serde_json5::from_str(&file_content).unwrap_or_default();

      if build_profile.is_object()
        && build_profile
          .get("app")
          .is_some_and(|app| app.is_object() && build_profile.get("modules").is_some_and(|modules| modules.is_array()))
      {
        projects.push(Project {
          project_detector: project_detector.clone(env).unwrap(),
          uri: Uri::file(entry.path().parent().unwrap().to_string_lossy().to_string()),
          parsed_build_profile: build_profile,
          build_profile_uri: Uri::file(file_path),
          build_profile_content: file_content,
        })
      }
    }

    projects
  }

  #[napi]
  pub fn get_uri(&self) -> Uri {
    self.uri.clone()
  }

  #[napi]
  pub fn get_parsed_build_profile(&self) -> serde_json::Value {
    self.parsed_build_profile.clone()
  }

  #[napi]
  pub fn get_build_profile_uri(&self) -> Uri {
    self.build_profile_uri.clone()
  }

  #[napi]
  pub fn get_build_profile_content(&self) -> String {
    self.build_profile_content.clone()
  }
}
