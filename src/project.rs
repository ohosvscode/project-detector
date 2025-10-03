use std::{fs, path::Path};
use napi::Env;
use napi::bindgen_prelude::Reference;
use napi_derive::napi;
use url::Url;
use crate::project_detector::ProjectDetector;
use walkdir::WalkDir;

/**
 * {@linkcode Workspace} represents a `hvigor` project.
 *
 * Hvigor is a build task orchestration tool based on TypeScript, which mainly
 * provides task management mechanisms, including task registration orchestration,
 * project model management, configuration management, and provides specific
 * processes and configurable settings for building and testing applications.
 *
 * @see https://developer.huawei.com/consumer/en/doc/harmonyos-guides/ide-hvigor
 *
 * ---
 *
 * {@linkcode Workspace} 代表一个`hvigor`工程。
 *
 * 编译构建工具 Hvigor 是一款基于TypeScript实现的构建任务编排工具，主要提供任务
 * 管理机制，包括任务注册编排、工程模型管理、配置管理等关键能力，提供专用于构建
 * 和测试应用的流程和可配置设置。
 *
 * @see https://developer.huawei.com/consumer/cn/doc/harmonyos-guides/ide-hvigor
 */
#[napi]
pub struct Project {
  build_profile_content: String,
  parsed_build_profile_content: serde_json::Value,
  project_detector: Reference<ProjectDetector>,
  uri: Url
}

impl Project {
  /**
   * Get the project folder URL.
   *
   * @returns The project folder URL.
   */
  pub fn get_project_folder_url(&self) -> Url {
    self.uri.clone()
  }
}

#[napi]
impl Project {
  /**
   * Get the project uri.
   *
   * @returns The project uri.
   */
  #[napi]
  pub fn get_uri(&self) -> String {
    self.uri.to_string()
  }

  /**
   * Create a new project.
   *
   * @param project_detector - The project detector.
   * @param directory_uri - The directory URI, must be a valid URL like `file://`.
   * @returns The project.
   */
  #[napi(ts_return_type = "Project | undefined")]
  pub fn create(project_detector: Reference<ProjectDetector>, directory_uri: String) -> Option<Project> {
    let workspace_folder = project_detector.get_workspace_folder();
    let uri = match Url::parse(&directory_uri) {
      Ok(url) => url,
      Err(_) => return None
    };
    let directory_file_path = match uri.to_file_path() {
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
        uri,
      }
    )
  }

  /**
   * Get the workspace-level `build-profile.json5` content.
   *
   * @returns The workspace-level `build-profile.json5` content.
   */
  #[napi]
  pub fn get_build_profile_content(&self) -> String {
    self.build_profile_content.clone()
  }

  /**
   * Get the parsed workspace-level `build-profile.json5` content.
   *
   * @returns The parsed workspace-level `build-profile.json5` content.
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
    let workspace_path = match workspace_folder.to_file_path() {
      Ok(path) => path,
      Err(_) => return projects,
    };

    for entry in WalkDir::new(&workspace_path)
      .follow_links(false)
      .into_iter()
      .filter_entry(|e| {
        !e.path().iter().any(|component| {
          if let Some(component_str) = component.to_str() {
            component_str == "node_modules" || component_str == "oh_modules"
          } else {
            false
          }
        })
      })
    {
      let entry = match entry {
        Ok(e) => e,
        Err(_) => continue,
      };

      if !entry.file_type().is_file() || entry.file_name() != "build-profile.json5" {
        continue;
      }

      let directory_path = entry.path().parent();
      if directory_path.is_none() {
        continue;
      }

      let directory_url = match Url::from_directory_path(directory_path.unwrap()) {
        Ok(url) => url.to_string(),
        Err(_) => continue,
      };

      if let Some(project) = Project::create(project_detector.clone(env).unwrap(), directory_url) {
        projects.push(project);
      }
    }

    projects
  }
}