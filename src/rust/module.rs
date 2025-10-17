use crate::project::Project;
use crate::utils::uri::Uri;
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;
use std::fs;
use std::path::Path;

#[napi]
pub struct Module {
  module_name: String,
  uri: Uri,
  project: Reference<Project>,
  parsed_build_profile: serde_json::Value,
  build_profile_uri: Uri,
  build_profile_content: String,
}

#[napi]
impl Module {
  #[napi]
  pub fn create(project: Reference<Project>, module_uri: String, env: Env) -> Option<Module> {
    let uri = Uri::file(module_uri);
    let build_profile_path = Path::new(&uri.fs_path()).join("build-profile.json5");
    let build_profile_uri = Uri::file(build_profile_path.to_string_lossy().to_string());
    let build_profile_content = fs::read_to_string(build_profile_path).unwrap_or_default();
    let parsed_build_profile: serde_json::Value = serde_json5::from_str(&build_profile_content).unwrap_or_default();
    if !parsed_build_profile.is_object() || !parsed_build_profile.get("targets").is_some_and(|targets| targets.is_array()) {
      return None;
    }

    Some(Module {
      module_name: Self::extract_module_name(&parsed_build_profile),
      uri,
      project: project.clone(env).unwrap(),
      parsed_build_profile,
      build_profile_uri,
      build_profile_content,
    })
  }

  #[napi]
  pub fn find_all(project: Reference<Project>, env: Env) -> Vec<Module> {
    let parsed_build_profile = project.get_parsed_build_profile();
    let mut modules = Vec::new();

    let modules_array = match parsed_build_profile.get("modules").and_then(|modules| modules.as_array()) {
      Some(array) => array,
      None => return modules,
    };

    for module_config in modules_array {
      let project_ref = project.clone(env).unwrap();
      let uri = Self::build_module_uri(project_ref, module_config);
      if let Some(module) = Self::create(project.clone(env).unwrap(), uri.to_string(), env) {
        modules.push(module);
      }
    }

    modules
  }

  #[napi]
  pub fn get_uri(&self) -> Uri {
    self.uri.clone()
  }

  #[napi]
  pub fn get_module_name(&self) -> String {
    self.module_name.clone()
  }

  #[napi]
  pub fn get_project(&self, env: Env) -> Reference<Project> {
    self.project.clone(env).unwrap()
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

impl Module {
  fn extract_module_name(module_config: &serde_json::Value) -> String {
    module_config.get("name").and_then(|name| name.as_str()).unwrap_or("").to_string()
  }

  fn build_module_uri(project: Reference<Project>, module_config: &serde_json::Value) -> Uri {
    let project_path = project.get_uri().fs_path();
    let src_path = module_config.get("srcPath").and_then(|path| path.as_str()).unwrap_or("");
    let full_path = path_clean::clean(Path::new(&project_path).join(src_path).to_string_lossy().to_string());

    Uri::file(full_path.to_string_lossy().to_string())
  }
}
