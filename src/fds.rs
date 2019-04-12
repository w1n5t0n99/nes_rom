use std::io;
use std::io::prelude::*;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use super::RomError;

const HEADER_SIZE: u64 = 16;
const DISK_SIZE: u64 = 65500;
const BLOCK_1_SIZE: u64 = 56;
const BLOCK_2_SIZE: u64 = 2;
const BLOCK_3_SIZE: u64 = 17;

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum KindOfFile {
    PrgRam = 0,
    ChrRam = 1,
    Nametable = 2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileData {
    pub boot_read_file_code: u8,
    pub disk_number: u8, 
    pub emu_disk_number: u8,
    pub side_number: u8, 
    pub emu_side_number: u8,
    pub file_number: u8,
    pub file_id: u8,
    pub file_name: [u8; 8],
    pub file_address: u16,
    pub file_size: u16,
    pub file_type: KindOfFile,
    pub file_data: Vec<u8>,
}

impl FileData {
    pub fn new() -> FileData {
        FileData {
            boot_read_file_code: 0,
            disk_number: 0,
            emu_disk_number: 0,
            side_number: 0,
            emu_side_number: 0,
            file_number: 0,
            file_id: 0,
            file_name: [0; 8],
            file_address: 0,
            file_size: 0,
            file_type: KindOfFile::PrgRam,
            file_data: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fds {
    pub sides_amount: u8,
    pub disk_files: Vec<FileData>,
}

impl Fds {
    pub fn new() -> Fds {
        Fds {
            sides_amount: 0,
            disk_files: Vec::new(),
        }
    }

     pub fn from_rom<R: Read + Seek>(mut file: R) -> Result<Fds, RomError> {
        let mut buf: [u8; 16] = [0; 16];
        file.read_exact(&mut buf)?;
        // check header
        if &buf[0..4] != super::FDS_GUARD {
            return Err(RomError::InvalidFormat);
        }  

        let mut fds = Fds::new();
        fds.sides_amount = buf[4];

        let cur_pos = file.seek(io::SeekFrom::Current(0))?;
        let file_size = file.seek(io::SeekFrom::End(0))?;
        file.seek(io::SeekFrom::Start(cur_pos))?;

        if file_size < (fds.sides_amount as u64 * DISK_SIZE) + 16 {
            return Err(RomError::InvalidRom);
        }

        let mut disk_num = -1;
        let mut side_num;
        for i in 0..fds.sides_amount {
            file.seek(io::SeekFrom::Start(HEADER_SIZE + (i as u64 * DISK_SIZE)))?;

            side_num = i % 2;
            if i%2 == 0 {
                disk_num += 1;
            }

            Fds::read_disk_side(&mut file, &mut fds.disk_files, side_num, disk_num as u8)?;
        }
        
        Ok(fds)
     }

     fn read_disk_side<R: Read>(file: &mut R, fv: &mut Vec<FileData>, eside_num: u8, edisk_num: u8) -> Result<(), RomError> {
         // block 1
        let mut buf1: [u8; BLOCK_1_SIZE as usize] = [0; BLOCK_1_SIZE as usize];
        file.read_exact(&mut buf1)?;
        // check block code
        if buf1[0] != 1 {
            return Err(RomError::InvalidRom);
        }

        // If the FDS is started with a disk whose side number and disk number aren't both $00, it will be prompted to insert the first disk side. 
        // However, some games make this number $00, even for the second disk to make it bootable too.
        let side_num = buf1[21];
        let disk_num = buf1[22];
        // All files with IDs smaller or equals to the boot read file code will be loaded when the game is booting.
        let boot_read_file_code = buf1[25];
        // block 2
        let mut buf2: [u8; BLOCK_2_SIZE as usize] = [0; BLOCK_2_SIZE as usize];
        file.read_exact(&mut buf2)?;
        // check block code
        if buf2[0] != 2 {
            return Err(RomError::InvalidRom);
        }

        let mut last_file_good: bool = true;
        while last_file_good {
            match Fds::read_disk_files(file) {
                Some(mut fd) => {
                    fd.disk_number = disk_num;
                    fd.side_number = side_num;
                    fd.emu_disk_number = edisk_num;
                    fd.emu_side_number = eside_num;
                    fd.boot_read_file_code = boot_read_file_code;
                    fv.push(fd)
                },
                None => last_file_good = false,
            }
        }

        Ok(())
     }

    fn read_disk_files<R: Read>(file: &mut R) -> Option<FileData> {
        // read file header
        let mut buf: [u8; BLOCK_3_SIZE as usize] = [0; BLOCK_3_SIZE as usize];
        match file.read_exact(&mut buf) {
            Ok(_) => (),
            Err(_) => return None,
        }

        // disk will be zero filled after last file
        if buf[0] != 0x03 || buf[16] != 0x04 {
            return None;
        }

        let mut fd = FileData::new();
        fd.file_number = buf[1];
        fd.file_id = buf[2];
        fd.file_name.copy_from_slice(&buf[3..11]);
        fd.file_address = ((buf[12] as u16) << 8) | buf[11] as u16;
        fd.file_size = ((buf[14] as u16) << 8) | buf[13] as u16;
        fd.file_type = match KindOfFile::from_u8(buf[15]) {
            Some(kof) => kof,
            None => return None,
        };

        fd.file_data = vec![0; (fd.file_size) as usize];
        match file.read_exact(fd.file_data.as_mut_slice()) {
            Ok(_) => (),
            Err(_) => return None,
        }

        Some(fd)
    }

}