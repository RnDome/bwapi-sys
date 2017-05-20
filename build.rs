use std::env;

fn main() {
    let bwapic_dir = env::var("BWAPIC_DIR").expect("Environment variable BWAPIC_DIR");
    println!("cargo:rustc-link-search={}", bwapic_dir);
    println!("cargo:rustc-link-lib=BWAPIC");
}
