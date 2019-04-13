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
            println!(" ines rom =========================================================");
            println!("\theader ver. {:?}", ines.header_version);
            println!("\tmapper: {:?} submapper: {:?}", ines.mapper, ines.submapper);
            println!("\tmirroring: {:?}", ines.nametable_mirroring);
            println!("\tprg rom size: {:?} chr rom size: {:?}", ines.prg_rom_size, ines.chr_rom_size);
            println!("\tprg rom crc: {:X} chr rom crc: {:X}", ines.prg_crc, ines.chr_crc);
            // Nes 2.0 allows more detailed handling of ram types
            match ines.ram {
                ines::Ram::Ines(prg_ram) => println!("\tprg ram: {:?}", prg_ram),
                ines::Ram::Nes2{prg_ram, prg_nvram, chr_ram, chr_nvram} =>
                    println!("\tprg ram: {:?} chr ram: {:?} prg nvram: {:?} chr nvram: {:?}", prg_ram, chr_ram, prg_nvram, chr_nvram),
            }
        }
        Err(ref e) => println!("{:?}", e),
    }

    let unif_file = File::open("example_roms\\EarthWorm Jim 2 (Unl) [U][!].unf").unwrap();
    let unif = unif::Unif::from_rom(unif_file);

    match unif {
        Ok(ref unif) => {
            println!(" unif rom =========================================================");
            println!("\tname: {:?}", unif.name);
            println!("\tread: {:?}", unif.read);
            println!("\tmapr: {:?}", unif.mapr);
            println!("\tmirr: {:?}", unif.mirr);
            println!("\tprg rom size: {:?} chr rom size: {:?}", unif.prg_data.len(), unif.chr_data.len());
            println!("\tprg rom crc: {:X} chr rom crc: {:X}", unif.prg_crc, unif.chr_crc);
            println!("\tbatr: {:?}", unif.batr);
        }
        Err(ref e) => println!("{:?}", e),
    }

    let fds_file = File::open("example_roms\\Time Twist - Rekishi no Katasumi de (1991)(Nintendo).fds").unwrap();
    let fds = fds::Fds::from_rom(fds_file);

     match fds {
        Ok(ref fds) => {
            println!(" fds rom =========================================================");
            println!(" \tfile -----------------------------------------------");

            for f in &fds.disk_files {
                let fname = unsafe { 
                    String::from_utf8_unchecked(f.file_name.to_vec())
                };
                println!("\t\tdisk num: {:?} actual disk num: {:?}", f.disk_number, f.actual_disk_number);
                println!("\t\tside num: {:?}", f.side_number);
                println!("\t\tfile number: {:?}", f.file_number);
                println!("\t\tfile id: {:?}", f.file_id);
                println!("\t\tfile type: {:?}", f.file_type);
                println!("\t\tfile address: {:X}", f.file_address);
                println!("\t\tboot read file code: {:?}", f.boot_read_file_code);
                println!("\t\tfile size: {:?}", f.file_size);
                println!("\t\tfile name: {:?}", fname);
                println!(" \tfile -----------------------------------------------");
            }
        },
        Err(e) => println!("fds ERROR: {:?}", e),
    }

}