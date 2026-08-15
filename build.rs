fn main() {
  napi_build::setup();

  if cfg!(windows) {
    println!(
      "cargo:rustc-link-search=native={}",
      std::env::var("NODE_DIR").unwrap()
    );
    println!("cargo:rustc-link-lib=dylib=node");
  }
}
