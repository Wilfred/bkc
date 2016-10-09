extern crate elf;
extern crate byteorder;

use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use byteorder::{LittleEndian, WriteBytesExt};

use elf::types::*;

// Based on https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
// and reading the excellent Rust elf library.
fn write_elf64(name: &str) -> Result<(), Error> {
    let mut buffer = try!(File::create(name));
    // Write the ELF magic number.
    try!(buffer.write(&[ELFMAG0, ELFMAG1, ELFMAG2, ELFMAG3]));

    // We want a 64 bit ELF file.
    try!(buffer.write(&[ELFCLASS64.0]));

    // Little-endian.
    try!(buffer.write(&[ELFDATA2LSB.0]));

    // Zero-pad to 16 bytes, so 'System V' and default ABI.
    try!(buffer.write(&[0u8; 10]));

    // Relocatable.
    try!(buffer.write_u16::<LittleEndian>(ET_REL.0));

    // x86-64 architecture.
    try!(buffer.write_u16::<LittleEndian>(EM_X86_64.0));

    // ELF v1
    try!(buffer.write_u32::<LittleEndian>(EV_CURRENT.0));

    // Entry point for executing the process.
    try!(buffer.write_u64::<LittleEndian>(0));

    println!("wrote {}", name);
    Ok(())
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

    write_elf64("tiddles_out.o").unwrap();
}
