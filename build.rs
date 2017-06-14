extern crate reqwest;
extern crate flate2;
extern crate tar;

use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;

use flate2::read::GzDecoder;
use tar::Archive;

macro_rules! get(($name:expr) => (ok!(env::var($name))));
macro_rules! ok(($expression:expr) => ($expression.unwrap()));
macro_rules! log {
    ($fmt:expr) => (println!(concat!("bwapi-sys/build.rs:{}: ", $fmt), line!()));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("bwapi-sys/build.rs:{}: ", $fmt),
    line!(), $($arg)*));
}
macro_rules! log_var(($var:ident) => (log!(concat!(stringify!($var), " = {:?}"), $var)));

fn download(url: &str, into: &Path) {
    if into.exists() {
        log!("File {} was already downloaded", into.display());
    } else {
        log!("Downloading to {:?}", &into);

        let mut response = reqwest::get(url).unwrap();

        if !response.status().is_success() {
            log!("Status: {}", response.status());
            log!("Headers:\n{}", response.headers());
            panic!("Could not download file");
        }

        let mut file = File::create(&into).unwrap();
        std::io::copy(&mut response, &mut file).unwrap();
    }
}

fn extract(archive_path: &Path, extract_to: &Path) {
    let file = File::open(archive_path).unwrap();
    let unzipped = GzDecoder::new(file).unwrap();
    let mut archive = Archive::new(unzipped);
    archive.unpack(extract_to).unwrap();
}

fn main() {
    let output_dir   = PathBuf::from(&get!("OUT_DIR"));
    let target       = &get!("TARGET");
    let build_mode   = &get!("PROFILE");

    log_var!(output_dir);
    log_var!(target);
    log_var!(build_mode);

    if !target.contains("i686-pc-windows-gnu") {
        panic!("Only --target=i686-pc-windows-gnu is supported");
    }

    let binary_url = format!(
        "https://github.com/RnDome/bwapi-c/releases/download/v{version}/bwapi-c-{mode}-win32.tar.gz",
        version="0.3.0", mode=&build_mode);
    log_var!(binary_url);
    let short_file_name = binary_url.split("/").last().unwrap();

    let zip_path = output_dir.join(short_file_name);
    log_var!(zip_path);

    log!("Obtaining BWAPI-C distribution...");
    download(&binary_url, &zip_path);

    log!("Unpacking archive...");
    extract(&zip_path, &output_dir);

    println!("cargo:rerun-if-changed={}", output_dir.display());
    println!("cargo:rustc-link-search=native={}", output_dir.join("lib").display());
    println!("cargo:rustc-link-lib=static=BWAPIC");
}
