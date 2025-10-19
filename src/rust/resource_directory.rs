use crate::resource::Resource;
use crate::utils::qualifier::utils_impl::QualifierUtils;
use crate::utils::uri::Uri;
use core::str;
use std::fs;
use napi::bindgen_prelude::Reference;
use napi::Env;
use napi_derive::napi;
use serde_json::Value;
use std::path::Path;

#[napi]
pub struct ResourceDirectory {
  uri: Uri,
  resource: Reference<Resource>,
}

#[napi]
impl ResourceDirectory {
  #[napi]
  pub fn find_all(resource: Reference<Resource>, env: Env) -> Vec<ResourceDirectory> {
    let mut resource_directories = Vec::new();
    let resource_directory = resource.get_uri();

    let dirs = match fs::read_dir(resource_directory.fs_path()) {
      Ok(dirs) => dirs,
      Err(_) => return resource_directories,
    };

    for dir in dirs.flatten() {
      if dir.metadata().map(|metadata| metadata.is_dir()).unwrap_or(false) {
        let dir_name = dir.file_name().to_string_lossy().to_string();
        if dir_name != "base" && dir_name != "rawfile" && dir_name != "resfile" && QualifierUtils::analyze_qualifier(dir_name).is_empty() {
          continue;
        }
        resource_directories.push(
          ResourceDirectory {
            uri: Uri::file(dir.path().to_string_lossy().to_string()),
            resource: resource.clone(env).unwrap(),
          }
        )
      }
    }

    resource_directories
  }

  #[napi]
  pub fn create(resource: Reference<Resource>, resource_directory_uri: String) -> Option<ResourceDirectory> {
    let uri = Uri::file(resource_directory_uri);
    if fs::metadata(uri.fs_path()).map(|metadata| metadata.is_dir()).unwrap_or(false) {
      let dir_name = Uri::base_name(&uri);
      if dir_name != "base" && dir_name != "rawfile" && dir_name != "resfile" && QualifierUtils::analyze_qualifier(dir_name).is_empty() {
        return None;
      }
      Some(
        ResourceDirectory {
          uri,
          resource,
        }
      )
    } else {
      None
    }
  }

  #[napi]
  pub fn get_uri(&self) -> Uri {
    self.uri.clone()
  }

  #[napi]
  pub fn get_resource(&self, env: Env) -> Reference<Resource> {
    self.resource.clone(env).unwrap()
  }

  #[napi(ts_return_type = "Array<Qualifier> | 'base' | 'rawfile' | 'resfile'")]
  pub fn get_qualifiers(&self) -> Value {
    let directory_name = Path::new(&self.uri.fs_path()).file_name().unwrap_or_default().to_string_lossy().to_string();
    if directory_name == "base" || directory_name == "rawfile" || directory_name == "resfile" {
      Value::String(directory_name)
    } else {
      Value::Array(
        QualifierUtils::analyze_qualifier(directory_name)
          .into_iter()
          .map(|q| serde_json::to_value(q).unwrap_or(Value::Null))
          .collect(),
      )
    }
  }
}
