extern crate nes_rom;

use std::fs::File;
use nes_rom::*;

fn main() {
    // The roms are parsed with very little manipulation to not make assumptions about client code use.
    // Error checking is done to make sure data is correct size and header flag conflicts (e.g Ines/ Nes2.0)

    let ines_file = File::open("example_roms\\Super Mario Bros. + Duck Hunt (USA).nes").unwrap();
    let ines = ines::Ines::from_rom(ines_file);

    match ines {
        Ok(ref ines) => {
            println!(" ines rom -------------------------------------------------------");
            println!("header ver. {:?}", ines.header_version);
            println!("mapper: {:?} submapper: {:?}", ines.mapper, ines.submapper);
            println!("mirroring: {:?}", ines.nametable_mirroring);
            println!("prg rom size: {:?} chr rom size: {:?}", ines.prg_rom_size, ines.chr_rom_size);
            println!("prg rom crc: {:X} chr rom crc: {:X}", ines.prg_crc, ines.chr_crc);
            // Nes 2.0 allows more detailed handling of ram types
            match ines.ram {
                ines::Ram::Ines(prg_ram) => println!("prg ram: {:?}", prg_ram),
                ines::Ram::Nes2{prg_ram, prg_nvram, chr_ram, chr_nvram} =>
                    println!("prg ram: {:?} chr ram: {:?} prg nvram: {:?} chr nvram: {:?}", prg_ram, chr_ram, prg_nvram, chr_nvram),
            }
        }
        Err(ref e) => println!("{:?}", e),
    }

    let unif_file = File::open("example_roms\\EarthWorm Jim 2 (Unl) [U][!].unf").unwrap();
    let unif = unif::Unif::from_rom(unif_file);

    match unif {
        Ok(ref unif) => {
            println!(" unif rom -------------------------------------------------------");
            println!("name: {:?}", unif.name);
            println!("read: {:?}", unif.read);
            println!("mapr: {:?}", unif.mapr);
            println!("mirr: {:?}", unif.mirr);
            println!("prg rom size: {:?} chr rom size: {:?}", unif.prg_data.len(), unif.chr_data.len());
            println!("prg rom crc: {:X} chr rom crc: {:X}", unif.prg_crc, unif.chr_crc);
            println!("batr: {:?}", unif.batr);
        }
        Err(ref e) => println!("{:?}", e),
    }

}