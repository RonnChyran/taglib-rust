#[cfg(feature="pkg-config")]
extern crate pkg_config;

#[cfg(feature="vcpkg")]
extern crate vcpkg;

fn main() {
  if !build_pkgconfig() && !build_vcpkg() {
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

#[cfg(not(feature="vcpkg"))]
fn build_vcpkg() -> bool {
  false
}

#[cfg(feature="vcpkg")]
fn build_vcpkg() -> bool {
  let config = vcpkg::Config::new()
      .lib_names("tag_c","tag")
      .probe("taglib");
  match config {
    Ok(library) => {
      for metadata in library.cargo_metadata {
        println!("{}", metadata);
      }
      true
    },
    Err(err) => {
      panic!("{}", err);
    }
  }
}
