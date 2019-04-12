extern crate nes_rom;

use std::fs::File;
use nes_rom::{ines, unif, fds};

fn main() {

    let ines_file = File::open("example_roms\\Super Mario Bros. + Duck Hunt (USA).nes").unwrap();
    let rom = ines::Ines::from_rom(ines_file);

    match rom {
            Ok(ref rom) => println!("mapper: {:?} prg: {:?} chr: {:?}", rom.mapper, rom.prg_rom_size, rom.chr_rom_size),
            Err(ref e) => println!("{:?}", e),
    }
}