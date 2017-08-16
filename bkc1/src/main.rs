extern crate elf;
extern crate byteorder;

use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use byteorder::{LittleEndian, BigEndian, WriteBytesExt};

use elf::types::*;

// TODO: Make a write_u16! macro to be consistent with the elf
// library.
fn write_u16(mut out: &File, value: u16, elf_header: FileHeader) -> Result<(), Error> {
    match elf_header.data {
        ELFDATA2LSB => out.write_u16::<LittleEndian>(value),
        ELFDATA2MSB => out.write_u16::<BigEndian>(value),
        ELFDATANONE => {
            panic!("Unable to resolve file endianness");
        }
        _ => {
            panic!("Unable to resolve file endianness");
        }
    }
}

fn write_u32(mut out: &File, value: u32, elf_header: FileHeader) -> Result<(), Error> {
    match elf_header.data {
        ELFDATA2LSB => out.write_u32::<LittleEndian>(value),
        ELFDATA2MSB => out.write_u32::<BigEndian>(value),
        ELFDATANONE => {
            panic!("Unable to resolve file endianness");
        }
        _ => {
            panic!("Unable to resolve file endianness");
        }
    }
}

fn write_u64(mut out: &File, value: u64, elf_header: FileHeader) -> Result<(), Error> {
    match elf_header.data {
        ELFDATA2LSB => out.write_u64::<LittleEndian>(value),
        ELFDATA2MSB => out.write_u64::<BigEndian>(value),
        ELFDATANONE => {
            panic!("Unable to resolve file endianness");
        }
        _ => {
            panic!("Unable to resolve file endianness");
 }
    }
}

// Based on https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
// and reading the excellent Rust elf library.
fn write_elf_file(elf_file: elf::File, filename: &str) -> Result<(), Error> {
    let mut buffer = try!(File::create(filename));

    // Write the ELF magic number.
    try!(buffer.write(&[ELFMAG0, ELFMAG1, ELFMAG2, ELFMAG3]));

    try!(buffer.write(&[elf_file.ehdr.class.0]));
    try!(buffer.write(&[elf_file.ehdr.data.0]));

    // TODO: add this to the elf::File struct.
    try!(buffer.write(&[1]));

    try!(buffer.write(&[elf_file.ehdr.osabi.0]));
    try!(buffer.write(&[elf_file.ehdr.abiversion]));

    // Currently unused in ELF.
    try!(buffer.write(&[0; 7]));

    try!(write_u16(&mut buffer, elf_file.ehdr.elftype.0, elf_file.ehdr));
    try!(write_u16(&mut buffer, elf_file.ehdr.machine.0, elf_file.ehdr));
    try!(write_u32(&mut buffer, elf_file.ehdr.version.0, elf_file.ehdr));

    if elf_file.ehdr.class == ELFCLASS32 {
        unimplemented!();
    } else {
        try!(write_u64(&mut buffer, elf_file.ehdr.entry, elf_file.ehdr));

        // TODO: phoff isn't in the File struct, fix that.
        let phoff = 0;
        try!(write_u64(&mut buffer, phoff, elf_file.ehdr));

        // TODO: calculate this value somehow.
        let shoff = 264;
        try!(write_u64(&mut buffer, shoff, elf_file.ehdr));

        // TODO: these values aren't in the file header struct either.
        let flags = 0;
        try!(write_u32(&mut buffer, flags, elf_file.ehdr));
    }

    // Header size.
    if elf_file.ehdr.class == ELFCLASS32 {
        try!(write_u16(&mut buffer, 52, elf_file.ehdr));
    } else {
        try!(write_u16(&mut buffer, 64, elf_file.ehdr));
    }

    // TODO: add to struct?
    let phentsize = 0;
    try!(write_u16(&mut buffer, phentsize, elf_file.ehdr));

    let phnum = elf_file.phdrs.len() as u16;
    try!(write_u16(&mut buffer, phnum, elf_file.ehdr));

    // TODO: calculate this.
    let shentsize = 64;
    try!(write_u16(&mut buffer, shentsize, elf_file.ehdr));

    let shnum = elf_file.sections.len() as u16;
    try!(write_u16(&mut buffer, shnum, elf_file.ehdr));

    // TODO: iterate through sections to find this value.
    let shstrndx = 4;
    try!(write_u16(&mut buffer, shstrndx, elf_file.ehdr));

    println!("wrote copy {}", filename);
    Ok(())
}

fn main() {
    let path = PathBuf::from("/home/wilfred/projects/tiddles/exit.o");
    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    write_elf_file(file, "copy.o").unwrap();
}
