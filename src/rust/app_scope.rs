use std::{fs, path::Path};
use napi::bindgen_prelude::Reference;
use napi_derive::napi;
use crate::{project::Project, utils::uri::Uri};

#[napi]
pub struct AppScope {
  app_scope_uri: Uri,
  app_scope_config_uri: Uri,
  app_scope_config_content: String,
  parsed_app_scope_config: serde_json::Value,
}

#[napi]
impl AppScope {
  #[napi]
  pub fn from(project: Reference<Project>) -> Option<AppScope> {
    let app_scope_uri = Path::new(&project.get_uri().fs_path()).join("AppScope").to_string_lossy().to_string();
    let app_scope_config_uri = Path::new(&app_scope_uri).join("app.json5").to_string_lossy().to_string();
    let app_scope_config_content = match fs::read_to_string(&app_scope_config_uri) {
      Ok(app_scope_config_content) => app_scope_config_content,
      Err(_) => return None,
    };
    let parsed_app_scope_config: serde_json::Value = match serde_json5::from_str(&app_scope_config_content) {
      Ok(parsed_app_scope_config) => parsed_app_scope_config,
      Err(_) => return None,
    };

    Some(AppScope {
      app_scope_uri: Uri::file(app_scope_uri),
      app_scope_config_uri: Uri::file(app_scope_config_uri),
      app_scope_config_content,
      parsed_app_scope_config,
    })
  }

  #[napi]
  pub fn get_uri(&self) -> Uri {
    self.app_scope_uri.clone()
  }

  #[napi]
  pub fn get_config_uri(&self) -> Uri {
    self.app_scope_config_uri.clone()
  }

  #[napi]
  pub fn get_config_content(&self) -> String {
    self.app_scope_config_content.clone()
  }

  #[napi]
  pub fn get_parsed_config_content(&self) -> serde_json::Value {
    self.parsed_app_scope_config.clone()
  }

  pub fn update_app_scope_config_content(&mut self, app_scope_config_content: String) {
    self.app_scope_config_content = app_scope_config_content;
  }

  pub fn update_parsed_app_scope_config(&mut self, parsed_app_scope_config: serde_json::Value) {
    self.parsed_app_scope_config = parsed_app_scope_config;
  }

  #[napi]
  pub fn reload(&mut self) {
    let app_scope_config_content = fs::read_to_string(self.app_scope_config_uri.fs_path()).unwrap_or_default();
    let parsed_app_scope_config: serde_json::Value = serde_json5::from_str(&app_scope_config_content).unwrap_or_default();

    self.update_app_scope_config_content(app_scope_config_content);
    self.update_parsed_app_scope_config(parsed_app_scope_config);
  }
}