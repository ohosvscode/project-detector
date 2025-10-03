use std::sync::Arc;
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
  product_folder: Arc<Url>,
  parsed_module_json5_content: serde_json::Value,
  module_json5_content: String,
}

#[napi]
impl Product {
  #[napi]
  pub fn create(env: Env, module: Reference<Module>, directory_uri: String) -> Option<Product> {
    let parsed_directory_url = match Url::parse(&directory_uri) {
      Ok(url) => url,
      Err(_) => return None
    };
    let module_json5_path = match parsed_directory_url.join("module.json5") {
      Ok(url) => url.to_string(),
      Err(_) => return None,
    };
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
        product_folder: Arc::new(parsed_directory_url),
        parsed_module_json5_content,
        module_json5_content,
      }
    )
  }

  #[napi]
  pub fn get_product_folder(&self) -> String {
    self.product_folder.to_string()
  }

  pub fn get_product_folder_url(&self) -> Arc<Url> {
    self.product_folder.clone()
  }

  #[napi]
  pub fn get_module(&self, env: Env) -> Reference<Module> {
    self.module.clone(env).unwrap()
  }

  #[napi]
  pub fn get_module_json5_content(&self) -> String {
    self.module_json5_content.clone()
  }

  #[napi(ts_return_type = "unknown")]
  pub fn get_parsed_module_json5_content(&self) -> serde_json::Value {
    self.parsed_module_json5_content.clone()
  }
}