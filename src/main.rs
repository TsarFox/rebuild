mod fmt;

use std::fs::File;
use std::io::Read;
use std::io::Write; // Temporary

fn main() {
    let filename = "/tmp/DUKE3D.GRP";

    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            panic!("Couldn't open {}", filename);
        }
    };

    let mut bytes: Vec<u8> = Vec::new();

    match file.read_to_end(&mut bytes) {
        Ok(count) => println!("Read {} bytes", count),
        Err(err) => panic!("Couldn't read."),
    }

    let grp = fmt::Group::new(&bytes).unwrap();

    println!("Files: {}", grp.file_count);

    for entry in grp {
        println!("{}", entry.name);
        if entry.name.contains("MID") {
            let mut filename = String::from("/tmp/");
            filename.push_str(&entry.name);
            filename = filename[..filename.find('\x00').unwrap()].to_string();
            
            let mut file = File::create(filename).unwrap();
            file.write_all(&entry.data).unwrap();
        }
    }
}
