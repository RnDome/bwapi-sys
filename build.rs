extern crate reqwest;
extern crate flate2;
extern crate tar;
extern crate cmake;

use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;

use flate2::read::GzDecoder;
use tar::Archive;
use std::process::Command;
use cmake::Config;

macro_rules! get(($name:expr) => (ok!(env::var($name))));
macro_rules! ok(($expression:expr) => ($expression.unwrap()));
macro_rules! log {
    ($fmt:expr) => (println!(concat!("bwapi-sys/build.rs:{}: ", $fmt), line!()));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("bwapi-sys/build.rs:{}: ", $fmt),
    line!(), $($arg)*));
}
macro_rules! log_var (($var:ident) => (log!(concat!(stringify!($var), " = {:?}"), $var)));

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

fn download_bwapic_library(platform: String, output_dir: &PathBuf, bwapic_version: &str, build_mode: &str) {
    // download windows or linux bwapic binary and extract it
    let binary_url = format!(
        "https://github.com/RnDome/bwapi-c/releases/download/v{version}/bwapi-c-{mode}-{platform}.tar.gz",
        version = &bwapic_version, mode = &build_mode, platform = &platform);
    let short_file_name = binary_url.split("/").last().unwrap();
    let zip_path = output_dir.join(short_file_name);
    log_var!(binary_url);
    log_var!(short_file_name);
    log_var!(zip_path);
    log!("Obtaining BWAPI-C distribution (for {})...", platform);
    download(&binary_url, &zip_path);
    log!("Unpacking archive (for {})...", platform);
    extract(&zip_path, &output_dir);
}

fn main() {
    let output_dir = PathBuf::from(&get!("OUT_DIR"));
    let target = &get!("TARGET");
    let build_mode = &get!("PROFILE");
    let supported = vec!("x86_64-unknown-linux-gnu", "i686-pc-windows-gnu");
    let cur_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let bwapic_version = "1.0.0";

    log_var!(output_dir);
    log_var!(target);
    log_var!(build_mode);
    log_var!(cur_dir);

    if !supported.contains(&&target[..]) {
        panic!("Only --target from {:?} is supported", supported);
    } else if target.contains("x86_64-unknown-linux-gnu") {

        // OpenBW/openbw and OpenBW/bwapi are added as the submodules, download them in case of absence
        if  !Path::new("openbw/bwapi/.git").exists() ||
            !Path::new("openbw/openbw/.git").exists() ||
            !Path::new("bwapi-c/.git").exists() {
            // download the sources of all the submodules
            let _ = Command::new("git").args(&["submodule", "update", "--init"])
                .status();
        }
        // OpenBW/bwapi is being built by cmake
        let openbw_dir = format!("{}/openbw/openbw", cur_dir);
        log_var!(openbw_dir);
        let dst = Config::new("openbw/bwapi")
            .define("OPENBW_DIR", &openbw_dir)
            .define("OPENBW_ENABLE_UI", "1")
            .build();
        log_var!(dst);
        // previous variant: download_bwapic_library("linux".to_owned(), &output_dir, bwapic_version, build_mode);
        let bwapic_dir = format!("{}/bwapi-c", cur_dir);
        log_var!(bwapic_dir);
        let dst = Config::new("bwapi-c")
            .define("BWAPI_PATH", &openbw_dir)
            .build();
        log_var!(dst);

    } else if target.contains("i686-pc-windows-gnu") {

        // for windows download bwapic library release
        download_bwapic_library("win32".to_owned(), &output_dir, bwapic_version, build_mode);

    }
    println!("cargo:rerun-if-changed={}", output_dir.display());
    println!("cargo:rustc-link-search=native={}", output_dir.join("lib").display());
    println!("cargo:rustc-link-lib=static=BWAPIC");
}
