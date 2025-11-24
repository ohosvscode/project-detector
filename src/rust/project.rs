use crate::project_detector::ProjectDetector;
use crate::utils::uri::Uri;
#[cfg(not(test))]
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;
#[cfg(not(test))]
use std::{fs, rc::Rc};
use std::path::Path;
#[cfg(not(test))]
use walkdir::WalkDir;

#[napi]
pub struct Project {
  #[cfg(not(test))]
  project_detector: Rc<Reference<ProjectDetector>>,
  #[cfg(test)]
  project_detector: ProjectDetector,
  uri: Uri,
  parsed_build_profile: serde_json::Value,
  build_profile_uri: Uri,
  build_profile_content: String,
  app_scope_uri: Uri,
  app_scope_config_uri: Uri,
  app_scope_config_content: String,
}

#[napi]
impl Project {
  #[napi]
  #[cfg(not(test))]
  pub fn get_project_detector(&self, env: Env) -> Reference<ProjectDetector> {
    match self.project_detector.as_ref().clone(env) {
      Ok(cloned_project_detector) => cloned_project_detector,
      Err(_) => panic!("Failed to get cloned project detector, please check your project detector is valid in Project.getProjectDetector()."),
    }
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
    let cloned_project_detector = Rc::new(
      match project_detector.clone(env) {
        Ok(cloned_project_detector) => cloned_project_detector,
        Err(_) => panic!("Failed to get cloned project detector, please check your project detector is valid in Project.findAll()."),
      }
    );
    let mut projects = Vec::new();
    let workspace_folder = cloned_project_detector.get_workspace_folder().fs_path();
    let entries: Vec<_> = WalkDir::new(workspace_folder)
      .into_iter()
      .filter_entry(|entry| !Self::is_in_exclude_dirs(entry))
      .filter_map(|res| res.ok())
      .collect();

    for entry in entries {
      let path = entry.path();
      if path.file_name().and_then(|name| name.to_str()).is_some_and(|name| name == "build-profile.json5") && entry.file_type().is_file() {
        let project_dir = path.parent().unwrap_or(Path::new("")).to_string_lossy().to_string();
        if let Some(project) = Self::create(Rc::clone(&cloned_project_detector), project_dir) {
          projects.push(project);
        }
      }
    }

    projects
  }

  #[napi]
  #[cfg(not(test))]
  pub fn create(project_detector: Rc<Reference<ProjectDetector>>, project_uri: String) -> Option<Project> {
    let uri = Uri::file(project_uri);
    if !fs::metadata(uri.fs_path()).map(|m| m.is_dir()).unwrap_or(false) {
      return None;
    }
    let build_profile_path = Path::new(&uri.fs_path()).join("build-profile.json5");
    let build_profile_uri = Uri::file(build_profile_path.to_string_lossy().to_string());
    let build_profile_content = fs::read_to_string(build_profile_uri.fs_path()).unwrap_or_default();
    let parsed_build_profile: serde_json::Value = serde_json5::from_str(&build_profile_content).unwrap_or_default();
    let app_scope_uri = Uri::file(Path::new(&uri.fs_path()).join("AppScope").to_string_lossy().to_string());
    let app_scope_config_uri = Uri::file(Path::new(&app_scope_uri.fs_path()).join("config.json5").to_string_lossy().to_string());
    let app_scope_config_content = fs::read_to_string(app_scope_config_uri.fs_path()).unwrap_or_default();
    

    if parsed_build_profile.is_object()
      && parsed_build_profile
        .get("app")
        .is_some_and(|app| app.is_object() && parsed_build_profile.get("modules").and_then(|modules| modules.as_array()).is_some())
    {
      Some(Project {
        project_detector: Rc::clone(&project_detector),
        uri,
        parsed_build_profile,
        build_profile_uri,
        build_profile_content,
        app_scope_uri,
        app_scope_config_uri,
        app_scope_config_content,
      })
    } else {
      None
    }
  }

  #[napi]
  #[cfg(not(test))]
  pub fn reload(project: &mut Project) {
    let project_uri = project.get_uri();
    let build_profile_path = Path::new(&project_uri.fs_path()).join("build-profile.json5");
    let build_profile_content = fs::read_to_string(build_profile_path).unwrap_or_default();
    let parsed_build_profile: serde_json::Value = serde_json5::from_str(&build_profile_content).unwrap_or_default();
    if !parsed_build_profile.is_object()
      || !parsed_build_profile
        .get("app")
        .is_some_and(|app| app.is_object() && parsed_build_profile.get("modules").and_then(|modules| modules.as_array()).is_some())
    {
      return;
    }

    project.update_parsed_build_profile(parsed_build_profile);
    project.update_build_profile_content(build_profile_content);
  }

  #[napi]
  pub fn get_uri(&self) -> Uri {
    self.uri.clone()
  }

  #[napi]
  pub fn get_app_scope_uri(&self) -> Uri {
    self.app_scope_uri.clone()
  }

  #[napi]
  pub fn get_app_scope_resource_uri(&self) -> Uri {
    Uri::file(Path::new(&self.app_scope_uri.fs_path()).join("resources").to_string_lossy().to_string())
  }

  #[napi]
  pub fn get_app_scope_resource_config_uri(&self) -> Uri {
    self.app_scope_config_uri.clone()
  }

  #[napi(ts_return_type = "unknown")]
  pub fn get_app_scope_config(&self) -> serde_json::Value {
    serde_json5::from_str(&self.app_scope_config_content).unwrap_or_default()
  }

  #[napi]
  pub fn get_parsed_build_profile(&self) -> serde_json::Value {
    self.parsed_build_profile.clone()
  }

  pub fn update_parsed_build_profile(&mut self, parsed_build_profile: serde_json::Value) {
    self.parsed_build_profile = parsed_build_profile;
  }

  #[napi]
  pub fn get_build_profile_uri(&self) -> Uri {
    self.build_profile_uri.clone()
  }

  #[napi]
  pub fn get_build_profile_content(&self) -> String {
    self.build_profile_content.clone()
  }

  pub fn update_build_profile_content(&mut self, build_profile_content: String) {
    self.build_profile_content = build_profile_content;
  }
}
