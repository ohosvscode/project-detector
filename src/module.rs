use std::{path::Path, sync::Arc};
use std::fs;
use crate::project::Project;
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;
use url::Url;

/**
 * Single {@linkcode Workspace} contains multiple modules.
 *
 * As a basic functional unit of apps/atomic services, a module contains source
 * code, resource files, third-party libraries, and configuration files.
 *
 * It must contain the `build-profile.json5` and `oh-package.json5` files at the
 * project level, so the current module implementation provides
 * {@linkcode getProjectBuildProfile} and {@linkcode getProjectOhPackage} methods
 * to get their related information.
 *
 * @see https://developer.huawei.com/consumer/en/doc/harmonyos-guides/ide-hvigor-multi-module
 *
 * ---
 * 一个工作空间包含多个模块。
 *
 * 作为应用/原子服务的最小功能单元，模块包含源代码、资源文件、第三方库和配置文件。
 *
 * 它首先必须包含有工程级的`build-profile.json5` 和`oh-package.json5`文件，因
 * 此在当前模块实现中提供{@linkcode getProjectBuildProfile} 和 {@linkcode getProjectOhPackage}
 * 方法来获取它们的相关信息。
 *
 *
 * @see https://developer.huawei.com/consumer/cn/doc/harmonyos-guides/ide-hvigor-multi-module
 */
#[napi]
pub struct Module {
  module_folder: Arc<Url>,
  project: Reference<Project>,
  module_level_build_profile_content: String,
  module_level_build_profile_parsed_content: serde_json::Value,
  module_level_build_profile_path: String,
}

#[napi]
impl Module {
  /**
   * Create a new module.
   *
   * @param project - The project.
   * @param directory_uri - The directory URI, must be a valid URL like `file://`.
   * @returns The module.
   */
  #[napi]
  pub fn create(env: Env, project: Reference<Project>, directory_uri: String) -> Option<Module> {
    let parsed_directory_url = match Url::parse(&directory_uri) {
      Ok(url) => url,
      Err(_) => return None,
    };
    let join_module_level_build_profile_path = match parsed_directory_url.clone().to_file_path() {
      Ok(path) => match path.clone().to_str() {
        Some(path) => Path::join(Path::new(path), "build-profile.json5"),
        None => return None,
      },
      Err(_) => return None,
    };
    let module_level_build_profile_content = match fs::read_to_string(join_module_level_build_profile_path.clone()) {
      Ok(content) => content,
      Err(_) => return None,
    };
    let module_level_build_profile_parsed_content = match serde_json5::from_str::<serde_json::Value>(&module_level_build_profile_content) {
      Ok(value) => value,
      Err(_) => return None,
    };

    let app_field_is_object = match module_level_build_profile_parsed_content.get("app") {
      Some(value) => value.is_object(),
      None => false,
    };
    let modules_field_is_array = match module_level_build_profile_parsed_content.get("modules") {
      Some(value) => value.is_array(),
      None => false,
    };

    if app_field_is_object == true || modules_field_is_array == true {
      return None;
    }

    let module_level_build_profile_path = match join_module_level_build_profile_path.to_str() {
      Some(path) => path.to_string(),
      None => return None,
    };

    Some(
      Module {
        module_folder: Arc::new(Url::parse(&directory_uri).unwrap()),
        project: project.clone(env).unwrap(),
        module_level_build_profile_content,
        module_level_build_profile_parsed_content,
        module_level_build_profile_path,
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
    let parsed_modules = match parsed_build_profile_content.get("modules") {
      Some(value) => match value.as_array() {
        Some(vec) => vec,
        None => return modules,
      },
      None => return modules,
    };
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

  pub fn get_module_folder_url(&self) -> Arc<Url> {
    self.module_folder.clone()
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

  #[napi]
  pub fn get_module_level_build_profile_path(&self) -> String {
    self.module_level_build_profile_path.clone()
  }

  #[napi(ts_return_type = "unknown")]
  pub fn get_parsed_build_profile_content(&self) -> serde_json::Value {
    self.module_level_build_profile_parsed_content.clone()
  }

  #[napi]
  pub fn get_build_profile_content(&self) -> String {
    self.module_level_build_profile_content.clone()
  }
}