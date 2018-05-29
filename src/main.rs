#[macro_use]
extern crate simple_error;

use std::process;

mod fmt;

fn main() {
    let filename = "DUKE3D.GRP";
    let mut group_manager = fmt::GroupManager::new();

    if let Err(e) = group_manager.load_file(filename) {
        println!("Couldn't open {}: {}", filename, e);
        process::exit(1);
    }

    println!("DOGWHINE.VOC: {} bytes", group_manager.get("DOGWHINE.VOC").unwrap().len());
}
