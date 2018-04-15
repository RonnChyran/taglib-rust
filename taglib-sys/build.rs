#[cfg(feature="pkg-config")]
extern crate pkg_config;

extern crate cmake;

use cmake::Config;
use std::path::Path;
use std::env;

fn main() {
  if !build_pkgconfig() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("lib").display());
    println!("cargo:rustc-flags=-l tag_c -l tag");
  }
}

#[cfg(not(feature="pkg-config"))]
fn build_pkgconfig() -> bool {
  false
}

#[cfg(feature="pkg-config")]
fn build_pkgconfig() -> bool {
  if pkg_config::find_library("taglib_c").is_err() {
    panic!("Could not find taglib_c via pkgconfig");
  }
  true
}

