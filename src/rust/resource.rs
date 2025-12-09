use crate::utils::uri::Uri;
use crate::{app_scope::AppScope, product::Product};
use napi::{bindgen_prelude::Reference, Env};
use napi_derive::napi;
use std::{fs, path::Path, rc::Rc};

#[napi]
#[derive(Clone)]
pub enum ResourceType {
  Product,
  AppScope,
}

#[napi]
pub struct Resource {
  product: Option<Rc<Reference<Product>>>,
  app_scope: Option<Reference<AppScope>>,
  uri: Uri,
  resource_type: ResourceType,
}

#[napi]
impl Resource {
  #[napi]
  pub fn find_all(product: Reference<Product>, env: Env) -> Vec<Resource> {
    let cloned_product = Rc::new(match product.clone(env) {
      Ok(cloned_product) => cloned_product,
      Err(_) => panic!("Failed to get cloned product, please check your product is valid in Resource.findAll()."),
    });
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
      product: Some(cloned_product),
      app_scope: None,
      uri: Uri::file(default_resource_root.to_string_lossy().to_string()),
      resource_type: ResourceType::Product,
    });

    resources
  }

  #[napi]
  pub fn from_app_scope(app_scope: Reference<AppScope>) -> Option<Resource> {
    let app_scope_config = app_scope.get_uri();
    let app_scope_resource_uri = Path::new(&app_scope_config.fs_path()).join("resources").to_string_lossy().to_string();

    Some(Resource {
      product: None,
      app_scope: Some(app_scope),
      uri: Uri::file(app_scope_resource_uri),
      resource_type: ResourceType::AppScope,
    })
  }

  #[napi]
  pub fn create(product: Rc<Reference<Product>>, resource_uri: String) -> Option<Resource> {
    let uri = Uri::file(resource_uri);
    if fs::metadata(uri.fs_path()).map(|metadata| metadata.is_dir()).unwrap_or(false) {
      Some(Resource {
        product: Some(Rc::clone(&product)),
        app_scope: None,
        uri,
        resource_type: ResourceType::Product,
      })
    } else {
      None
    }
  }

  /// If the resource created by {@linkcode Product}, this method will return the product.
  ///
  /// @throw If the resource is not created by {@linkcode Product}, this method will throw an error.
  #[napi]
  pub fn get_product(&self, env: Env) -> Reference<Product> {
    match &self.product {
      Some(product) => match product.as_ref().clone(env) {
        Ok(cloned_product) => cloned_product,
        Err(_) => panic!("Failed to get cloned product, please check your product is valid in Resource.getProduct()."),
      },
      None => panic!("Resource is not associated with a product, please check your resource is valid in Resource.getProduct()."),
    }
  }

  /// If the resource created by {@linkcode AppScope}, this method will return the app scope.
  ///
  /// @throw If the resource is not created by {@linkcode AppScope}, this method will throw an error.
  #[napi]
  pub fn get_app_scope(&self, env: Env) -> Reference<AppScope> {
    match &self.app_scope {
      Some(app_scope) => match app_scope.clone(env) {
        Ok(cloned_app_scope) => cloned_app_scope,
        Err(_) => panic!("Failed to get cloned app scope, please check your app scope is valid in Resource.getAppScope()."),
      },
      None => panic!("Resource is not associated with an app scope, please check your resource is valid in Resource.getAppScope()."),
    }
  }

  #[napi]
  pub fn get_uri(&self) -> Uri {
    self.uri.clone()
  }

  /// Get the type of current resource.
  ///
  /// - If created by {@link Product}, return {@link ResourceType.Product}.
  /// - If created by {@link AppScope}, return {@link ResourceType.AppScope}.
  #[napi(getter)]
  pub fn resource_type(&self) -> &ResourceType {
    &self.resource_type
  }
}
