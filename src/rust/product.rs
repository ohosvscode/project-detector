use crate::module::Module;
use crate::utils::uri::Uri;
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;
use std::path::Path;

#[napi]
pub struct Product {
  module: Reference<Module>,
  name: String,
}

#[napi]
impl Product {
  #[napi]
  pub fn find_all(module: Reference<Module>, env: Env) -> Vec<Product> {
    let mut products = Vec::new();
    let parsed_build_profile = module.get_parsed_build_profile();
    let targets_array = match parsed_build_profile.get("targets").and_then(|targets| targets.as_array()) {
      Some(array) => array,
      None => return products,
    };

    for target_config in targets_array {
      let target_name = target_config.get("name").and_then(|name| name.as_str()).unwrap_or_default();

      products.push(
        Product {
          module: module.clone(env).unwrap(),
          name: target_name.to_string(),
        }
      )
    }

    products
  }

  #[napi]
  pub fn get_module(&self, env: Env) -> Reference<Module> {
    self.module.clone(env).unwrap()
  }

  #[napi]
  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  #[napi]
  pub fn get_current_target_config(&self) -> serde_json::Value {
    let parsed_build_profile = self.module.get_parsed_build_profile();
    let targets_array = match parsed_build_profile.get("targets").and_then(|targets| targets.as_array()) {
      Some(array) => array,
      None => return serde_json::Value::Null,
    };
    let current_target_config = targets_array
      .iter()
      .find(|target| target.get("name").and_then(|name| name.as_str()).unwrap_or_default() == self.name);
    current_target_config.unwrap_or(&serde_json::Value::Null).clone()
  }

  #[napi]
  pub fn get_source_directories(&self) -> Vec<Uri> {
    let mut target_directories = Vec::new();
    let current_target_config = self.get_current_target_config();
    let name = self.get_name();
    let module_uri = self.module.get_uri();

    if current_target_config.is_null() {
      return target_directories;
    }

    let default_child_path = if name == "default" { "main" } else { &name };
    let default_source_root = Path::new(&module_uri.fs_path()).join("src").join(default_child_path);
    let source_roots = current_target_config
      .get("source")
      .and_then(|source| source.get("sourceRoots"))
      .and_then(|source_roots| source_roots.as_array());

    if let Some(source_roots) = source_roots {
      if source_roots.is_empty() {
        target_directories.push(Uri::file(default_source_root.to_string_lossy().to_string()));
      } else {
        for source_root in source_roots {
          let source_root_path = path_clean::clean(Path::new(&module_uri.fs_path()).join(source_root.as_str().unwrap_or_default()));
          target_directories.push(Uri::file(source_root_path.to_string_lossy().to_string()));
        }
      }
    } else {
      target_directories.push(Uri::file(default_source_root.to_string_lossy().to_string()));
    }

    target_directories
  }

  #[napi]
  pub fn get_resource_directories(&self) -> Vec<Uri> {
    let mut target_directories = Vec::new();
    let current_target_config = self.get_current_target_config();
    let name = self.get_name();
    let module_uri = self.module.get_uri();

    if current_target_config.is_null() {
      return target_directories;
    }

    let default_child_path = if name == "default" { "main" } else { &name };
    let default_resource_root = Path::new(&module_uri.fs_path()).join("src").join(default_child_path).join("resources");
    let resource_roots = current_target_config
      .get("resource")
      .and_then(|resource| resource.get("directories"))
      .and_then(|resource_roots| resource_roots.as_array());

    if let Some(resource_roots) = resource_roots {
      if resource_roots.is_empty() {
        target_directories.push(Uri::file(default_resource_root.to_string_lossy().to_string()));
      } else {
        for resource_root in resource_roots {
          let resource_root_path = path_clean::clean(Path::new(&module_uri.fs_path()).join(resource_root.as_str().unwrap_or_default()));
          target_directories.push(Uri::file(resource_root_path.to_string_lossy().to_string()));
        }
      }
    } else {
      target_directories.push(Uri::file(default_resource_root.to_string_lossy().to_string()));
    }

    target_directories
  }
}
