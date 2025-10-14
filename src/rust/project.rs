use crate::project_detector::ProjectDetector;
use crate::utils::uri::Uri;
#[cfg(not(test))]
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;
#[cfg(not(test))]
use std::fs;
#[cfg(not(test))]
use std::path::Path;
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

  pub fn is_in_exclude_dirs(entry: &walkdir::DirEntry) -> bool {
    entry.path().iter().any(|component| {
      if let Some(component_str) = component.to_str() {
        component_str == "node_modules" || component_str == "oh_modules" || component_str.starts_with('.')
      } else {
        false
      }
    })
  }

  #[napi]
  #[cfg(not(test))]
  pub fn find_all(project_detector: Reference<ProjectDetector>, env: Env) -> Vec<Project> {
    let mut projects = Vec::new();
    let workspace_folder = project_detector.get_workspace_folder().fs_path();
    let dir_entries = WalkDir::new(workspace_folder).into_iter().filter_map(|entry| match entry {
      Ok(entry) => {
        if Self::is_in_exclude_dirs(&entry) {
          None
        } else {
          Some(entry)
        }
      }
      Err(_) => None,
    });

    for entry in dir_entries {
      if let Some(project) = Self::create(
        project_detector.clone(env).unwrap(),
        env,
        entry.path().parent().unwrap_or(Path::new("")).to_string_lossy().to_string(),
      ) {
        projects.push(project);
      }
    }

    projects
  }

  #[napi]
  #[cfg(not(test))]
  pub fn create(project_detector: Reference<ProjectDetector>, env: Env, uri: String) -> Option<Project> {
    let uri = Uri::file(uri);
    let file_path = uri.fs_path();
    let build_profile_path = Path::new(&file_path).join("build-profile.json5");
    let build_profile_uri = Uri::file(build_profile_path.to_string_lossy().to_string());
    let file_content = fs::read_to_string(build_profile_uri.fs_path()).unwrap_or_default();
    let build_profile: serde_json::Value = serde_json5::from_str(&file_content).unwrap_or_default();

    if build_profile.is_object()
      && build_profile
        .get("app")
        .is_some_and(|app| app.is_object() && build_profile.get("modules").is_some_and(|modules| modules.is_array()))
    {
      Some(Project {
        project_detector: project_detector.clone(env).unwrap(),
        uri,
        parsed_build_profile: build_profile,
        build_profile_uri,
        build_profile_content: file_content,
      })
    } else {
      None
    }
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
