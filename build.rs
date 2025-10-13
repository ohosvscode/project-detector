extern crate napi_build;

use std::env;

fn main() {
  napi_build::setup();

  // 在 Linux GNU 的 arm64/armv7 目标上链接 libbsd，提供 le16toh 等符号
  let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
  let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
  let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

  if target_os == "linux" && target_env == "gnu" && (target_arch == "aarch64" || target_arch == "arm") {
    println!("cargo:rustc-link-lib=bsd");
  }
}
