use std::path::Path;
use std::fs;
use napi_derive::napi;
use napi::bindgen_prelude::Reference;
use napi::Env;
use url::Url;
use crate::module::Module;

/**
 * Single {@linkcode Module} contain multiple products.
 *
 * Usually, application vendors
 * will customize the same application for different versions based on different
 * deployment environments, different target audiences, different runtime environments, etc.,
 * such as domestic version, international version, normal version, VIP version,
 * free version, paid version, etc. For the above scenarios supports the use of a small
 * amount of code configuration to instantiate different versions of differences,
 * and achieve the high reuse of source code and resource files through different target
 * product versions during the compilation and build process.
 *
 * @see https://developer.huawei.com/consumer/en/doc/harmonyos-guides/ide-customized-multi-targets-and-products-guides#section1011341611469
 *
 * ---
 * 一个 {@linkcode Module} 包含多个product。
 *
 * 通常情况下，应用厂商会根据不同的部署环境，不同的目标人群，不同的运行环境等，
 * 将同一个应用定制为不同的版本，如国内版、国际版、普通版、VIP版、免费版、付费版等。
 * 针对以上场景支持通过少量的代码配置以实例化不同的差异版本，在编译构建过程中实现
 * 一个应用构建出不同的目标产物版本，从而实现源代码、资源文件等的高效复用。
 *
 * @see https://developer.huawei.com/consumer/cn/doc/harmonyos-guides/ide-customized-multi-targets-and-products-guides#section1011341611469
 */
#[napi]
pub struct Product {
  module: Reference<Module>,
  product_folder: Url,
  parsed_module_json5_content: serde_json::Value,
  module_json5_content: String,
  target_name: String,
}

#[napi]
impl Product {
  /**
   * Get the product directory uri.
   *
   * @returns The product directory uri.
   */
  #[napi]
  pub fn get_uri(&self) -> String {
    path_clean::clean(&self.product_folder.to_string())
  }

  #[napi(ts_return_type = "Product | undefined")]
  pub fn create(env: Env, module: Reference<Module>, directory_uri: String, target_name: String) -> Option<Product> {
    let parsed_directory_url = match Url::parse(&directory_uri) {
      Ok(url) => url,
      Err(_) => return None
    };
    let fs_parsed_directory = match parsed_directory_url.to_file_path() {
      Ok(path) => path,
      Err(_) => return None,
    };
    let module_json5_path = Path::join(Path::new(&fs_parsed_directory), Path::new("module.json5"));
    let module_json5_content = match fs::read_to_string(module_json5_path) {
      Ok(content) => content,
      Err(_) => return None,
    };
    let parsed_module_json5_content = match serde_json5::from_str::<serde_json::Value>(&module_json5_content) {
      Ok(value) => value,
      Err(_) => return None,
    };

    Some(
      Product {
        module: module.clone(env).unwrap(),
        product_folder: match Url::from_directory_path(fs_parsed_directory) {
          Ok(url) => url,
          Err(_) => return None,
        },
        parsed_module_json5_content,
        module_json5_content,
        target_name,
      }
    )
  }

  pub fn get_product_url(&self) -> Url {
    self.product_folder.clone()
  }

  /**
   * Get parent module.
   */
  #[napi]
  pub fn get_module(&self, env: Env) -> Reference<Module> {
    self.module.clone(env).unwrap()
  }

  /**
   * Get `module.json5` content.
   */
  #[napi]
  pub fn get_module_json5_content(&self) -> String {
    self.module_json5_content.clone()
  }

  /**
   * Get parsed `module.json5` content.
   */
  #[napi(ts_return_type = "unknown")]
  pub fn get_parsed_module_json5_content(&self) -> serde_json::Value {
    self.parsed_module_json5_content.clone()
  }

  /**
   * Get current product directory name.
   */
  #[napi]
  pub fn get_directory_name(&self) -> String {
    Path::new(&self.product_folder.to_string()).file_name().unwrap().to_string_lossy().to_string()
  }

  /**
   * Find all products.
   */
  #[napi]
  pub fn find_all(env: Env, module: Reference<Module>) -> Vec<Product> {
    let mut products = Vec::new();
    let module_folder = module.get_module_folder_url();
    let module_folder_path = match module_folder.to_file_path() {
      Ok(path) => path,
      Err(_) => return products,
    };
    let parsed_module_json5_content = module.get_parsed_build_profile_content();
    let targets_directory_names: Vec<String> = match parsed_module_json5_content
      .get("targets")
      .and_then(|v| v.as_array())
    {
      Some(vec) => vec
        .iter()
        .filter_map(|item| {
          item
            .get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
        })
        .collect(),
      None => return products,
    };

    let src_directory_path = Path::join(&module_folder_path, "src");
    for target_directory_name in targets_directory_names {
      let target_name = if target_directory_name == "default" { 
        "main".to_string()
      } else {
        target_directory_name.clone()
      };
      let target_directory_path = match Path::join(&src_directory_path, &target_name).to_str() {
        Some(path) => path.to_string(),
        None => continue,
      };
      
      let product_directory_path = match Url::from_directory_path(target_directory_path) {
        Ok(url) => url.to_string(),
        Err(_) => continue,
      };
      if let Some(product) = Product::create(env, module.clone(env).unwrap(), product_directory_path.to_string(), target_directory_name) {
        products.push(product);
      }
    }

    products
  }

  #[napi]
  pub fn get_target_name(&self) -> String {
    self.target_name.clone()
  }

  /**
   * Get resource directories from model level `build-profile.json5` file.
   * 
   * If the resource directories is not set will return the default resource directories like:
   * 
   * - If the target is `default`, the result path is `src/main/resources`
   * - If the target is `foo`, the result path is `src/foo/resources`
   */
  #[napi]
  pub fn get_resource_directories(&self, env: Env) -> Vec<String> {
    let mut resource_directories = Vec::new();
    let target_name = self.get_target_name();
    let parsed_module_json5_content = self.get_module(env).get_parsed_build_profile_content();
    let maybe_target = parsed_module_json5_content
      .get("targets")
      .and_then(|v| v.as_array())
      .and_then(|vec| vec.iter().find(|item| {
        match item.get("name") {
          Some(value) => match value.as_str() {
            Some(name) => name == target_name,
            None => false,
          },
          None => false,
        }
      }));

    let directories = maybe_target
      .and_then(|target| target.get("resource"))
      .and_then(|resource| resource.as_object())
      .and_then(|resource_obj| resource_obj.get("directories"))
      .and_then(|dirs| dirs.as_array());

    if let Some(directories) = directories {
      let current_dir = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => return resource_directories,
      };

      let product_path = match self.get_product_url().to_file_path() {
        Ok(path_buf) => path_buf,
        Err(_) => return resource_directories,
      };

      for directory in directories {
        if let Some(dir) = directory.as_str() {
          let resource_path = product_path.join(Path::new(dir));
          let joined_path = current_dir.join(&resource_path);
          let cleaned_path = path_clean::clean(&joined_path.to_string_lossy());
          match Url::parse(&cleaned_path) {
            Ok(url) => resource_directories.push(url.to_string()),
            Err(_) => (),
          }
        }
      }
    }

    if resource_directories.len() == 0 {
      let default_resource_directory = Path::new(&self.get_uri()).join("resources").to_str().unwrap().to_string();
      resource_directories.push(Url::parse(&default_resource_directory).unwrap().to_string());
    }

    resource_directories
  }
}