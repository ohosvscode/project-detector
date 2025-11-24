use std::{fs, path::Path, rc::Rc};
use crate::product::Product;
use crate::utils::uri::Uri;
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;

#[napi]
pub struct Resource {
  product: Rc<Reference<Product>>,
  uri: Uri,
}

#[napi]
impl Resource {
  #[napi]
  pub fn find_all(product: Reference<Product>, env: Env) -> Vec<Resource> {
    let cloned_product = Rc::new(
      match product.clone(env) {
        Ok(cloned_product) => cloned_product,
        Err(_) => panic!("Failed to get cloned product, please check your product is valid in Resource.findAll()."),
      }
    );
    let mut resources = Vec::new();
    let current_target_config = product.get_current_target_config();
    let name = product.get_name();
    let module_uri = product.get_module(env).get_uri();

    if current_target_config.is_null() {
      return resources;
    }

    let default_child_path = if name == "default" { "main" } else { &name };
    let default_resource_root = Path::new(&module_uri.fs_path()).join("src").join(default_child_path).join("resources");
    let resource_roots = current_target_config
      .get("resource")
      .and_then(|resource| resource.get("directories"))
      .and_then(|resource_roots| resource_roots.as_array());

    if let Some(resource_roots) = resource_roots {
      if !resource_roots.is_empty() {
        for resource_root in resource_roots {
          let resource_root_path = path_clean::clean(Path::new(&module_uri.fs_path()).join(resource_root.as_str().unwrap_or_default()));
          if let Some(resource) = Self::create(Rc::clone(&cloned_product), resource_root_path.to_string_lossy().to_string()) {
            resources.push(resource);
          }
        }
        return resources;
      }
    }

    resources.push(Resource {
      product: cloned_product,
      uri: Uri::file(default_resource_root.to_string_lossy().to_string()),
    });

    resources
  }

  #[napi]
  pub fn create(product: Rc<Reference<Product>>, resource_uri: String) -> Option<Resource> {
    let uri = Uri::file(resource_uri);
    if fs::metadata(uri.fs_path()).map(|metadata| metadata.is_dir()).unwrap_or(false) {
      Some(Resource {
        product: Rc::clone(&product),
        uri,
      })
    } else {
      None
    }
  }

  #[napi]
  pub fn get_product(&self, env: Env) -> Reference<Product> {
    match self.product.as_ref().clone(env) {
      Ok(cloned_product) => cloned_product,
      Err(_) => panic!("Failed to get cloned product, please check your product is valid in Resource.getProduct()."),
    }
  }

  #[napi]
  pub fn get_uri(&self) -> Uri {
    self.uri.clone()
  }
}
