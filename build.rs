use std::env;

static KEY: &str = "SWIFT_RUNTIME_LIB_DIR";

fn main() {
    let runtime_lib_dir = match env::var(KEY) {
        Ok(val) => val,
        Err(_) => {
            println!("Environment variable {} not found", KEY);
            "/usr/lib/swift".to_string()
        }
    };
    println!("cargo:rerun-if-env-changed={}", KEY);
    println!("cargo:rustc-link-search=native={}", runtime_lib_dir);
    println!("cargo:rustc-link-lib=dylib=swiftCore");
}
