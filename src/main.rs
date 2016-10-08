extern crate elf;

use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

fn write_elf(name: &str) {
    let mut buffer = File::create(name).unwrap();
    buffer.write(b"some bytes").unwrap();
    println!("wrote {}", name);
}

fn main() {
    let path = PathBuf::from("/home/wilfred/projects/tiddles/exit.o");
    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    let text_scn = match file.get_section(".text") {
        Some(s) => s,
        None => panic!("Failed to look up .text section"),
    };

    println!("{:?}", text_scn.data);

    write_elf("tiddles_out.o");
}
