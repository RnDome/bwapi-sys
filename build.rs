extern crate reqwest;
extern crate flate2;
extern crate tar;

use std::env;
use std::path::{Path, PathBuf};
use std::fs::{File, DirBuilder};

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
    let architecture = "i686-pc-windows-gnu"; // TODO read from cargo env
    let build_mode   = "debug"; // TODO read from env

    let binary_url = format!(
        "https://github.com/RnDome/bwapi-c/releases/download/v{}/bwapi-c-{}-win32.tar.gz",
        "0.3.0", "Debug");
    log_var!(binary_url);
    let short_file_name = binary_url.split("/").last().unwrap();

    let download_dir = PathBuf::from(&get!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(&architecture)
        .join(&build_mode)
        .join("native");

    if !download_dir.exists() {
        DirBuilder::new().recursive(true).create(&download_dir).unwrap();
    }

    let zip_path = download_dir.join(short_file_name);
    log_var!(zip_path);

    let unpacked_dir = download_dir.join(format!("bwapi-c-{}-win32", "Debug"));
    log_var!(unpacked_dir);

    log!("Obtaining BWAPI-C distribution...");
    download(&binary_url, &zip_path);
    println!("cargo:rerun-if-changed={}", zip_path.display());

    log!("Unpacking archive...");
    extract(&zip_path, &unpacked_dir);

    println!("cargo:rustc-link-search={}", unpacked_dir.join("lib").display());
    println!("cargo:rustc-link-lib=BWAPIC");
}
