fn main() {
  napi_build::setup();

  if cfg!(windows) {
    let node_lib_dir = std::path::PathBuf::from(std::env::var("NODE_LIB_DIR").expect(
      "NODE_LIB_DIR must point to the target-arch directory containing
              node.lib",
    ));

    println!("cargo:rustc-link-search=native={}", node_lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=node");
  }
}
