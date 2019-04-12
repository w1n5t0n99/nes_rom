
#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate lazy_static;

pub mod unif;
pub mod ines;
pub mod fds;
mod crc32;

use std::io;
use std::error;
use std::fmt;
use std::convert::From;

static INES_GUARD: [u8; 4] = [0x4e, 0x45, 0x53, 0x1a];
static UNIF_GUARD: [u8; 4] = [0x55, 0x4e, 0x49, 0x46];
static FDS_GUARD: [u8; 4] = [0x46, 0x44, 0x53, 0x1a];
//static NSF_GUARD: [u8; 5] = [0x4e, 0x45, 0x53, 0x4d, 0x1a];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RomError {
    InvalidFormat,
    InvalidRom,
    InvalidConversion,
    IOError,
}

impl error::Error for RomError {
    fn description(&self) -> &str {
        match *self {
            RomError::InvalidFormat => "invalid rom file format",
            RomError::InvalidRom => "rom file contained invalid or corrupted data",
            RomError::InvalidConversion => "unable to process conversion",
            RomError::IOError => "rom file io error",
        }
    }
}

impl fmt::Display for RomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self)
    }
}

impl From<io::Error> for RomError {
    fn from(_err: io::Error) -> Self {
        RomError::IOError
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

