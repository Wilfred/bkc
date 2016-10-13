extern crate elf;
extern crate byteorder;

use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use byteorder::{LittleEndian, WriteBytesExt};

use elf::types::*;

fn write_elf_file(file: elf::File, filename: &str) -> Result<(), Error> {
    let mut buffer = try!(File::create(filename));

    // Write the ELF magic number.
    try!(buffer.write(&[ELFMAG0, ELFMAG1, ELFMAG2, ELFMAG3]));

    try!(buffer.write(&[file.ehdr.class.0]));
    try!(buffer.write(&[file.ehdr.data.0]));

    // TODO: add this to the elf::File struct.
    try!(buffer.write(&[1]));

    try!(buffer.write(&[file.ehdr.osabi.0]));
    try!(buffer.write(&[file.ehdr.abiversion]));

    // Currently unused in ELF.
    try!(buffer.write(&[0; 7]));

    println!("wrote copy {}", filename);
    Ok(())
}

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

    // Program header immediately follows elf header, so offset 0.
    try!(buffer.write_u64::<LittleEndian>(0));

    // TODO: calculate the correct section header offset.
    // This value is just blindly copy-pasted from exit.o.
    try!(buffer.write_u64::<LittleEndian>(264));

    // Flags can be zero on x86-64.
    try!(buffer.write_u32::<LittleEndian>(0));

    // Header size, which is 64 bytes.
    try!(buffer.write_u16::<LittleEndian>(64));

    // Program header table entry size.
    try!(buffer.write_u16::<LittleEndian>(0));

    // Number of entries in program header entry table.
    try!(buffer.write_u16::<LittleEndian>(0));

    // Size of a section header table entry.
    try!(buffer.write_u16::<LittleEndian>(64));

    // The number of sections we will write (which is the number of
    // entries in the section header table).
    try!(buffer.write_u16::<LittleEndian>(7));

    // The index in the section header table of section names.
    try!(buffer.write_u16::<LittleEndian>(4));

    println!("wrote {}", name);
    Ok(())
}

fn main() {
    let path = PathBuf::from("/home/wilfred/projects/tiddles/exit.o");
    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    write_elf64("tiddles_out.o").unwrap();
    write_elf_file(file, "copy.o").unwrap();
}
