extern crate reqwest;
#[macro_use] extern crate error_chain;
extern crate zip;

use std::env;
use std::path::{Path, PathBuf};
use std::io::Read;
use std::fs::{self, File, DirBuilder};


error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
        ZipError(zip::result::ZipError);
    }
}

/*fn main() {
    let bwapic_dir = env::var("BWAPIC_DIR").expect("Environment variable BWAPIC_DIR");
    //.unwrap_or("/home/korvin/work/research/bw/bwapi-sys/");
    println!("cargo:rustc-link-search={}", bwapic_dir);
    println!("cargo:rustc-link-lib=BWAPIC");
}*/

fn run() -> Result<()> {
    let architecture = "i686-pc-windows-gnu"; // TODO read from cargo env
    let zip_filename = "bwapi-c-0.1.0-Release-win32.zip"; // TODO read from config
    let build_mode   = "debug"; // TODO read from env
    
    let mut zip_path = PathBuf::from("./bwapi-c");
    // zip_path.push(architecture);
    // zip_path.push(build_mode);
    // zip_path.push("native");

    if !zip_path.exists() {
        println!("Creating download directory  {:?}", zip_path);
        DirBuilder::new().recursive(true).create(&zip_path)?;
    }

    zip_path.push(zip_filename);

    if zip_path.exists() {
        println!("Archive was already downloaded");
    } else {
        println!("Obtaining BWAPI-C distribution...");
        {
            let download_url  = String::from("https://github.com/RnDome/bwapi-c/releases/download/v0.1.0/") + zip_filename;
            let mut response = reqwest::get(&download_url)?;

            println!("Status: {}", response.status());
            println!("Headers:\n{}", response.headers());

            println!("Downloading to {:?}", &zip_path);
            let mut file = File::create(&zip_path)?;
            std::io::copy(&mut response, &mut file)?;
        }
    }

    println!("Unpacking archive...");
    {
        let file = File::open(&zip_path)?;
        let mut zip = zip::ZipArchive::new(file)?;
        println!("Archive contains {} entries", zip.len());

        for i in 0 .. zip.len()
        {
            let mut archived_file = zip.by_index(i)?;

            let mut unpacked_file = {
                let archived_file_name = PathBuf::from(archived_file.name());
                println!("Unpacking from {} ...", archived_file.name());

                let mut unpacked_path = zip_path.clone();
                unpacked_path.pop();
                let unpacked_path = unpacked_path.join(archived_file_name);

                println!("Unpacking to {:?} ...", unpacked_path);

                if unpacked_path.is_dir() {
                    DirBuilder::new().recursive(true).create(unpacked_path)?;
                    continue;
                } else {
                    File::create(&unpacked_path)?
                }
            };

            std::io::copy(&mut archived_file, &mut unpacked_file)?;
        }
    }

    println!("Done.");
    Ok(())
}

quick_main!(run);
