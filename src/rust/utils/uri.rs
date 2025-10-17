use napi_derive::napi;
use std::{fmt, path};
use url::Url;

#[napi]
#[derive(Clone, PartialEq, Eq)]
pub struct Uri {
  fs_path: path::PathBuf,
  url: Url,
}

#[napi]
impl Uri {
  #[napi]
  pub fn file(path: String) -> Self {
    let url = Url::from_file_path(&path).unwrap_or(Url::parse(&path).unwrap_or(Url::parse("file://").unwrap()));

    Self {
      fs_path: url.to_file_path().unwrap_or_default(),
      url,
    }
  }

  #[napi]
  pub fn parse(url: String) -> Self {
    let url = Url::parse(&url).unwrap_or(Url::parse("file://").unwrap());

    Self {
      fs_path: url.to_file_path().unwrap_or_default(),
      url,
    }
  }

  #[napi(js_name = "basename")]
  pub fn base_name(uri: &Uri) -> String {
    uri.url.path_segments()
      .and_then(|segments| segments.last())
      .unwrap_or_default()
      .to_string()
  }

  #[napi(js_name = "dirname")]
  pub fn dir_name(uri: &Uri) -> Uri {
    let path = path::Path::new(&uri.fs_path);
    Uri::file(
      path.parent()
      .map(|p| p.to_string_lossy().to_string())
      .unwrap_or_else(|| ".".to_string())
    )
  }

  #[napi]
  pub fn is_equal(&self, other: &Uri) -> bool {
    self.fs_path == other.fs_path || self.to_string() == other.to_string()
  }

  #[napi(getter)]
  pub fn fs_path(&self) -> String {
    self.fs_path.to_string_lossy().to_string()
  }

  #[napi(getter)]
  pub fn path(&self) -> String {
    self.url.path().to_string()
  }

  #[napi(getter)]
  pub fn scheme(&self) -> String {
    self.url.scheme().to_string()
  }

  #[napi(getter)]
  pub fn host(&self) -> String {
    self.url.host().map(|h| h.to_string()).unwrap_or_default()
  }

  #[napi(getter)]
  pub fn query(&self) -> String {
    self.url.query().map(|q| q.to_string()).unwrap_or_default()
  }

  #[napi(getter)]
  pub fn fragment(&self) -> String {
    self.url.fragment().map(|f| f.to_string()).unwrap_or_default()
  }

  #[napi(getter)]
  pub fn username(&self) -> String {
    self.url.username().to_string()
  }

  #[napi(getter)]
  pub fn password(&self) -> String {
    self.url.password().map(|p| p.to_string()).unwrap_or_default()
  }

  #[napi(getter)]
  pub fn port(&self) -> u16 {
    self.url.port().unwrap_or_default()
  }

  pub fn get_url(&self) -> Url {
    self.url.clone()
  }

  #[napi]
  #[allow(clippy::inherent_to_string_shadow_display)]
  pub fn to_string(&self) -> String {
    self.url.to_string()
  }
}

impl fmt::Display for Uri {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.url)
  }
}
