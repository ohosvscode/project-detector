use std::{sync::Arc};
use crate::project::Project;
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;
use url::Url;

#[napi]
pub struct Module {
  module_folder: Arc<Url>,
  project: Reference<Project>
}

#[napi]
impl Module {
  /**
   * Create a new module.
   *
   * @param project - The project.
   * @param directory_uri - The directory URI.
   * @returns The module.
   */
  #[napi]
  pub fn create(env: Env, project: Reference<Project>, directory_uri: String) -> Option<Module> {
    Some(
      Module {
        module_folder: Arc::new(Url::parse(&directory_uri).unwrap()),
        project: project.clone(env).unwrap(),
      }
    )
  }

  /**
   * Find all modules.
   *
   * @param project - The project.
   * @returns The modules.
   */
  #[napi]
  pub fn find_all(env: Env, project: Reference<Project>) -> Vec<Module> {
    let mut modules = Vec::new();
    let parsed_build_profile_content = project.get_parsed_build_profile_content();
    let parsed_modules = parsed_build_profile_content.get("modules").unwrap().as_array().unwrap();
    let project_folder = project.get_project_folder_url();

    for module in parsed_modules {
      if module.is_object() != true {
        continue;
      }

      let src_path = match module.get("srcPath") {
        Some(value) => match value.as_str() {
          Some(path) => path.to_string(),
          None => continue,
        },
        None => continue,
      };

      let module_folder = match project_folder.join(&src_path) {
        Ok(url) => url.to_string(),
        Err(_) => continue,
      };

      if let Some(module) = Module::create(env, project.clone(env).unwrap(), module_folder) {
        modules.push(module);
      }
    }

    modules
  }

  /**
   * Get the module folder.
   *
   * @returns The module folder.
   */
  #[napi]
  pub fn get_module_folder(&self) -> String {
    self.module_folder.to_string()
  }

  /**
   * Get the project.
   *
   * @returns The project.
   */
  #[napi]
  pub fn get_project(&self, env: Env) -> Reference<Project> {
    self.project.clone(env).unwrap()
  }
}