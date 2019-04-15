use std::io;
use std::io::prelude::*;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use super::{RomError,crc32};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NametableMirroring {
	Horizontal,
	Vertical,
	FourScreens,
    Other, // other types of mirroring not specified in ines header e.g. one-screen or diagnol
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum TvSystem {
    NTSC = 0,
    PAL = 1,
    MultRegion = 2,
    Dendy = 3,
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum GameInputType
{
	Unspecified = 0,
	StandardControllers = 1,
	FourScore = 2,
	FourPlayerAdapter = 3,
	VsSystem = 4,
	VsSystemSwapped = 5,
	VsSystemSwapAB = 6,
	VsZapper = 7,
	Zapper = 8,
	TwoZappers = 9,
	BandaiHypershot = 0x0A,
	PowerPadSideA = 0x0B,
	PowerPadSideB = 0x0C,
	FamilyTrainerSideA = 0x0D,
	FamilyTrainerSideB = 0x0E,
	ArkanoidControllerNes = 0x0F,
	ArkanoidControllerFamicom = 0x10,
	DoubleArkanoidController = 0x11,
	KonamiHyperShot = 0x12,
	PachinkoController = 0x13,
	ExcitingBoxing = 0x14,
	JissenMahjong = 0x15,
	PartyTap = 0x16,
	OekaKidsTablet = 0x17,
	BarcodeBattler = 0x18,
	MiraclePiano = 0x19, 
	PokkunMoguraa = 0x1A, 
	TopRider = 0x1B, 
	DoubleFisted = 0x1C, 
	Famicom3dSystem = 0x1D, 
	DoremikkoKeyboard = 0x1E, 
	RobGyroSet = 0x1F, 
	FamicomDataRecorder = 0x20,
	TurboFile = 0x21,
	BattleBox = 0x22,
	FamilyBasicKeyboard = 0x23,
	Pec586Keyboard = 0x24, 
	Bit79Keyboard = 0x25, 
	SuborKeyboard = 0x26,
	SuborKeyboardMouse1 = 0x27,
	SuborKeyboardMouse2 = 0x28,
	SnesMouse = 0x29,
	GenericMulticart = 0x2A, 
	SnesControllers = 0x2B,
	RacermateBicycle = 0x2C, 
	UForce = 0x2D, 
	RobStackUp = 0x2E,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BusConflictType
{
	Default,
	Yes,
	No,
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ConsoleType
{
	Regular = 0,
	VsSystem = 1,
	Playchoice = 2,
	FamicloneWithDecimalMode = 3,
    VT01Mono = 4,
    VT01Stn = 5,
    VT02 = 6,
    VT03 = 7,
    VT09 = 8,
    VT32 = 9,
    VT369 = 0xA, // 0xB-0xF reserved
    FDS = 0x10,
    Unknown = 0x11,
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum VsHardwareType
{
	Default = 0,
	RbiBaseballProtection = 1,
	TkoBoxingProtection = 2,
	SuperXeviousProtection = 3,
	IceClimberProtection = 4,
	VsDualSystem = 5,
	RaidOnBungelingBayProtection = 6,
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum VsPpuType
{
	RP2C03B = 0x0,
    RP2C03G = 0x1,
    RP2C04_0001 = 0x2,
    RP2C04_0002 = 0x3,
    RP2C04_0003 = 0x4,
    RP2C04_0004 = 0x5,
    RC2C03B = 0x6,
    RC2C03C = 0x7,
    RC2C05_01 = 0x8,
    RC2C05_02 = 0x9,
    RC2C05_03 = 0xA,
    RC2C05_04 = 0xB,
    RC2C05_05 = 0xC,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Ram {
    Ines(u32),
    Nes2{prg_ram: u32, prg_nvram: u32, chr_ram: u32, chr_nvram: u32,},
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HeaderVersion
{
	Ines,
	Nes2,
	ArchaiciNes,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ines {
    pub header_version: HeaderVersion,
    pub mapper: u32,
    pub submapper: u8,
    pub prg_rom_size: u32,
    pub chr_rom_size: u32,
    pub ram: Ram,
    pub nametable_mirroring: NametableMirroring,
    pub console_type: ConsoleType,
    pub tv_system: TvSystem,
    pub bus_conflict: BusConflictType,
    pub game_input: GameInputType,
    pub contains_nvram: bool,
    pub contains_trainer: bool,
    pub misc_rom_chips: u8,
    pub vs_system: Option<(VsHardwareType, VsPpuType)>,
    pub prg_crc: u32,
    pub chr_crc: u32,
    pub trainer_data: Vec<u8>,
    pub prg_data: Vec<u8>,
    pub chr_data: Vec<u8>,
    pub misc_data: Vec<u8>,
}

impl Ines {

    pub fn new() -> Ines {
        Ines {
            header_version: HeaderVersion::Ines,
            mapper: 0,
            submapper: 0,
            prg_rom_size: 0,
            chr_rom_size: 0,
            ram: Ram::Ines(0),
            nametable_mirroring: NametableMirroring::Horizontal,
            console_type: ConsoleType::Regular,
            tv_system: TvSystem::NTSC,
            bus_conflict: BusConflictType::Default,
            game_input: GameInputType::Unspecified,
            contains_nvram: false,
            contains_trainer: false,
            misc_rom_chips: 0,
            vs_system: None,
            prg_crc: 0,
            chr_crc: 0,
            trainer_data: Vec::new(),
            prg_data: Vec::new(),
            chr_data: Vec::new(),
            misc_data: Vec::new(),
        }
    }

    /// Load and parse .ines file
    /// 
    /// # Examples
    /// 
    ///  ```
    /// let ines = nes_rom::ines::Ines::from_rom(arg);
    ///  ```
    pub fn from_rom<R: Read + Seek>(mut file: R) -> Result<Ines, RomError> {
        let mut buf: [u8; 16] = [0; 16];
        file.read_exact(&mut buf)?;

        let mut ines;
        if &buf[0..4] != super::INES_GUARD {
            return Err(RomError::InvalidFormat);
        }  

         // spec: if anything other than 0b00001000 than rom is ines
		// will still need to be checked if dirty or corrupted rom header
		if (buf[7] & 0x0C) == 0x08 {
        ines = Ines::create_as_ines2(&mut buf)?;
        }
        else if (buf[7] & 0x0C) == 0x00 && buf[15] == 0 && buf[14] == 0 && buf[13] == 0 && buf[12] == 0 {
            ines = Ines::create_as_ines(&mut buf, false)?;
        }
        else {
            ines = Ines::create_as_ines(&mut buf, true)?;
        }

        // copy rom data
        ines.prg_data = vec![0u8; ines.prg_rom_size as usize];  
        ines.chr_data = vec![0u8; ines.chr_rom_size as usize];
        if ines.contains_trainer == true {
            ines.trainer_data = vec![0u8; 512];
        }

        file.read_exact(&mut ines.trainer_data)?;
        file.read_exact(&mut ines.prg_data)?;
        file.read_exact(&mut ines.chr_data)?;

        let curr = file.seek(io::SeekFrom::Current(0))?;
        let end = file.seek(io::SeekFrom::End(0))?;
        file.seek(io::SeekFrom::Start(curr))?;

        let len = (end - curr) as usize;
        if len > 0 {
            ines.misc_data = vec![0u8; len];
            file.read_exact(&mut ines.misc_data)?;
        }

        ines.prg_crc = crc32::crc32_ieee_from_slice(&ines.prg_data);     
        ines.chr_crc = crc32::crc32_ieee_from_slice(&ines.chr_data);           

        Ok(ines)
    }

    fn create_as_ines(header: &mut [u8], is_archaic: bool) -> Result<Ines, RomError> {
        let mut ines = Ines::new();

        ines.header_version = HeaderVersion::Ines;
        ines.prg_rom_size = header[4] as u32;
        ines.chr_rom_size = header[5] as u32;

        // flag 6
        if (header[6] & 0b00000010) == 0b00000010 {
            ines.contains_nvram = true;
        }

        if (header[6] & 0b00000100) == 0b00000100 {
            ines.contains_trainer = true;
        }
    
        if (header[6] & 0b00001000) == 0b00001000 {
            ines.nametable_mirroring = NametableMirroring::FourScreens;
        }
        else if (header[6] & 0b00000001) == 1{
            ines.nametable_mirroring = NametableMirroring::Vertical;
        }
        else {
            ines.nametable_mirroring = NametableMirroring::Horizontal;
        }

        let low_nibble = (header[6] & 0b11110000) >> 4;
        ines.mapper |= low_nibble as u32;

        ines.prg_rom_size = (ines.prg_rom_size * 16) * 1024;
        ines.chr_rom_size = (ines.chr_rom_size * 8) * 1024;

        if is_archaic == true {
            // "dirty rom" clear last 9 bytes
            header[7] = 0;
            header[8] = 0;
            header[9] = 0;
            header[10] = 0;
            header[11] = 0;
            header[12] = 0;
            header[13] = 0;
            header[14] = 0;
            header[15] = 0;

            ines.header_version = HeaderVersion::ArchaiciNes;
            return Ok(ines);
        }

        // flag 7
        // rom cannot be vs and playchoice
        if (header[7] & 0b00000011) == 0b00000011 {
            return Err(RomError::InvalidRom);
        }
        else if (header[7] & 0b00000001) == 0b00000001 {
            ines.console_type = ConsoleType::Playchoice;
        }
        else if (header[7] & 0b00000010) == 0b00000010 {
            ines.console_type = ConsoleType::VsSystem;
        }
        else {
            ines.console_type = ConsoleType::Regular
        }

        let high_nibble = header[7] & 0xF0;
        ines.mapper |= high_nibble as u32;

        // flag 8
        ines.ram = Ram::Ines(((header[8] as u32) * 8) * 1024);

        // flag 9 - usually 0
        // TODO check rom file name or database as alternative to flag 9
        if (header[9] & 0b00000001) == 0b00000001 {
            ines.tv_system = TvSystem::PAL;
        }
        else {
            ines.tv_system = TvSystem::NTSC;
        }

        Ok(ines)
    }

    fn create_as_ines2(header: &mut [u8]) -> Result<Ines, RomError> {
        let mut ines = Ines::new();

        ines.header_version = HeaderVersion::Nes2;
        ines.prg_rom_size = header[4] as u32;
        ines.chr_rom_size = header[5] as u32;

        // flag 6
        if (header[6] & 0b00000010) == 0b00000010 {
            ines.contains_nvram = true;
        }

        if (header[6] & 0b00000100) == 0b00000100 {
            ines.contains_trainer = true;
        }
    
        let low_nibble = (header[6] & 0b11110000) >> 4;
        ines.mapper |= low_nibble as u32;

         // following Mesen source - http://wiki.nesdev.com/w/index.php/Talk:NES_2.0
        // TODO possibly change mirroring
        if (header[6] & 0b00001001) == 0b00001001 {
            ines.nametable_mirroring = NametableMirroring::Other;     
        }
        else if (header[6] & 0b00001000) == 0b00001000 {
            ines.nametable_mirroring = NametableMirroring::FourScreens;
        }
        else if (header[6] & 0b00000001) == 1 {
            ines.nametable_mirroring = NametableMirroring::Vertical;
        }
        else {
            ines.nametable_mirroring = NametableMirroring::Horizontal;
        }

        //flag 8
        let higher_nibble = ((header[8] & 0b00001111) as u32) << 8;
        ines.mapper |= higher_nibble;
        ines.submapper = ((header[8] & 0b11110000) >> 4) as u8;

        //flag 9
        let higher_chr_rom_size = ((header[9] & 0b11110000) as u32) << 4;
        let higher_prg_rom_size = ((header[9] & 0b00001111) as u32) << 8;
        ines.chr_rom_size |= higher_chr_rom_size;
        ines.prg_rom_size |= higher_prg_rom_size;

        // flag 10
        // flag 11
        ines.ram = Ram::Nes2 {
            prg_nvram: 64 << (((header[10] & 0b11110000) >> 4) as u32),
            prg_ram: 64 << ((header[10] & 0b00001111) as u32),
            chr_nvram: 64 << (((header[11] & 0b11110000) >> 4) as u32),
            chr_ram: 64 << ((header[11] & 0b00001111) as u32),
        };

        match ines.ram {
            Ram::Nes2{prg_nvram: p, chr_nvram: c, ..} => {
                //For backward compatibility, the battery bit in the original iNES header (byte 6, bit 1) MUST be true if the upper nibble of byte 10 or 11 is nonzero or false otherwise
                if ines.contains_nvram == false && (p > 0 || c > 0)  {
                    return Err(RomError::InvalidRom);
                }
            },
            _ => (),
        }

        // flag 12
        let timing_mode = header[12] & 0b00000011;
        ines.tv_system = match TvSystem::from_u8(timing_mode) {
            Some(tv) => tv,
            None => return Err(RomError::InvalidRom),
        };

        // flag 13
        if (header[7] & 0b00000011) == 1 {
            let ppu_type = match VsPpuType::from_u8(header[13] & 0b00001111) {
                Some(ppu) => ppu,
                None => return Err(RomError::InvalidRom),
            };

            let ppu_hw = match VsHardwareType::from_u8(header[13] & 0b11110000) {
                Some(ppu) => ppu,
                None => return Err(RomError::InvalidRom),
            };

            ines.vs_system = Some((ppu_hw, ppu_type));     
        }
        else if (header[7] & 0b00000011) == 3 {
            ines.console_type = match ConsoleType::from_u8(header[13] & 0b00001111) {
                Some(gs) => gs,
                None => return Err(RomError::InvalidRom),
            };
        }

        //  flag 14
        // misc. roms
        ines.misc_rom_chips = header[14] & 0b00000011;

        // flag 15
        ines.game_input = match GameInputType::from_u8(header[15] * 0b00111111) {
            Some(gi) => gi,
            None => return Err(RomError::InvalidRom),
        };

        ines.prg_rom_size = (ines.prg_rom_size * 16) * 1024;
        ines.chr_rom_size = (ines.chr_rom_size * 8) * 1024;

        Ok(ines)
    }
}




