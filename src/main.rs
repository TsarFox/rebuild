#[macro_use]
extern crate simple_error;

use std::fs::File; // Temporary, used for 'extract'.
use std::io::Write; // Temporary, used for 'extract'.
use std::process;

mod fmt;
mod path;

// This function is strictly temporary and exists only to help me get samples of
// binary files to inspect in radare.
fn extract(name: &str, group_manager: &fmt::GroupManager) {
    let output_path = format!("/tmp/{}", name);
    let mut file = File::create(output_path).unwrap();
    file.write_all(group_manager.get(name).unwrap()).ok();
}

fn main() {
    let path_manager = path::PathManager::new();

    let filename = "DUKE3D.GRP";
    let mut group_manager = fmt::GroupManager::new(path_manager);

    if let Err(e) = group_manager.load_file(filename) {
        println!("Couldn't open {}: {}", filename, e);
        process::exit(1);
    }

    extract("PALETTE.DAT", &group_manager);
    // println!("DOGWHINE.VOC: {} bytes", group_manager.get("DOGWHINE.VOC").unwrap().len());
}
