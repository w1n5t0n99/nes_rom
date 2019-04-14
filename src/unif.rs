use std::str;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use super::{RomError,ines,crc32};

lazy_static! {
    // based on https://wiki.nesdev.com/w/index.php/UNIF_to_NES_2.0_Mapping
    // mapper, submapper, prg ram, chr ram
    static ref UNIF_BOARD_MAPPINGS: HashMap<&'static str, (u32, u8, u32, u32)> = {
        let mut map = HashMap::new();
        // nintendo-made boards
        map.insert("B4", (4,0,0,0));
        map.insert("AMROM", (7,2,0,8));
        map.insert("ANROM", (7,1,0,8));
        map.insert("AN1ROM", (7,1,0,8));
        map.insert("AOROM", (7,0,0,8));
        map.insert("BNROM", (34,2,0,8));
        map.insert("CNROM", (3,2,0,0));
        map.insert("CNROM+SECURITY", (185,0,0,0));
        map.insert("CPROM", (13,0,0,16));
        map.insert("FAMILYBASIC", (0,0,2,0));
        map.insert("EKROM", (5,0,8,0));
        map.insert("ELROM", (5,0,0,0));
        map.insert("ETROM", (5,0,16,8)); //8kb prg-ram+8kb prg-nvram
        map.insert("EWROM", (5,0,32,0));
        map.insert("FJROM", (10,0,8,0));
        map.insert("FKROM", (10,0,8,0));
        map.insert("HROM", (0,0,0,0));
        map.insert("HKROM", (4,1,1,0));
        map.insert("NROM", (0,0,0,0));
        map.insert("NROM-128", (0,0,0,0));
        map.insert("NROM-256", (0,0,0,0));
        map.insert("PEEOROM", (9,0,0,0));
        map.insert("PNROM", (9,0,0,0));
        map.insert("RROM", (0,0,0,0));
        map.insert("RTROM", (0,0,0,0));
        map.insert("SROM", (0,0,0,0));
        map.insert("SAROM", (1,0,0,0));
        map.insert("SBROM", (1,0,0,0));
        map.insert("SCROM", (1,0,0,0));
        map.insert("SC1ROM", (1,0,0,0));
        map.insert("SEROM", (1,5,0,0));
        map.insert("SFROM", (1,0,0,0));
        map.insert("SF1ROM", (1,0,0,0));
        map.insert("SFEXPROM", (1,0,0,0));
        map.insert("SGROM", (1,0,0,0));
        map.insert("SHROM", (1,5,0,0));
        map.insert("SH1ROM", (1,5,0,0));
        map.insert("SIROM", (1,0,8,0));
        map.insert("SJROM", (1,0,8,0));
        map.insert("SKROM", (1,0,8,0));
        map.insert("SLROM", (1,0,0,0));
        map.insert("SL1ROM", (1,0,0,0));
        map.insert("SL2ROM", (1,0,0,0));
        map.insert("SL3ROM", (1,0,0,0));
        map.insert("SLRROM", (1,0,0,0));
        map.insert("SMROM", (1,0,0,8));
        map.insert("SNROM", (1,0,8,8));
        map.insert("SNWEPROM", (1,0,8,8));
        map.insert("SOROM", (1,0,16,8)); //8kb prg-ram+8kb prg-nvram
        map.insert("SUROM", (1,0,8,8));
        map.insert("SXROM", (1,0,32,8));
        map.insert("TBROM", (4,0,0,0));
        map.insert("TEROM", (4,0,0,0));
        map.insert("TFROM", (4,0,0,0));
        map.insert("TGROM", (4,0,0,8));
        map.insert("TKROM", (4,0,8,0));
        map.insert("TK1ROM", (4,0,8,0));
        map.insert("TKEPROM", (4,0,8,0));
        map.insert("TKSROM", (118,0,8,0));
        map.insert("TLROM", (4,0,0,0));
        map.insert("TL1ROM", (4,0,0,0));
        map.insert("TL2ROM", (4,0,0,0));
        map.insert("TLSROM", (118,0,0,0));
        map.insert("TNROM", (4,0,8,8));
        map.insert("TQROM", (119,0,0,8));
        map.insert("TR1ROM", (4,0,0,0));
        map.insert("TSROM", (4,0,8,0));
        map.insert("TVROM", (4,0,0,0));
        map.insert("STROM", (0,0,0,0));
        map.insert("UNROM", (2,2,0,8));
        map.insert("UOROM", (2,2,0,8));
        // boards made by third-party licensees
        map.insert("ACCLAIM-MC-ACC", (4,3,0,0));
        map.insert("BANDAI-FCG-1", (16,0,0,0));
        map.insert("BANDAI-FCG-2", (16,0,0,0));
        map.insert("BANDAI-LZ93D50", (16,0,0,0));
        map.insert("BANDAI-LZ93D50+24C01", (159,0,128,0));
        map.insert("BANDAI-LZ93D50+24C02", (16,0,256,0));
        map.insert("BANDAI-PT-554", (3,2,0,0));
        map.insert("IREM-FCG-1", (16,0,0,0));
        map.insert("JALECO-JF01", (0,0,0,0));
        map.insert("JALECO-JF02", (0,0,0,0));
        map.insert("JALECO-JF03", (0,0,0,0));
        map.insert("JALECO-JF04", (0,0,0,0));
        map.insert("JALECO-JF15", (2,2,0,8));
        map.insert("JALECO-JF18", (2,2,0,8));
        map.insert("JALECO-JF23", (18,0,0,0));
        map.insert("JALECO-JF24", (18,0,0,0));
        map.insert("JALECO-JF25", (18,0,0,0));
        map.insert("JALECO-JF27", (18,0,8,0));
        map.insert("JALECO-JF29", (18,0,0,0));
        map.insert("JALECO-JF37", (18,0,0,0));
        map.insert("JALECO-JF40", (18,0,8,0));
        map.insert("NAMCOT-129", (19,0,0,0));
        map.insert("NAMCOT-163", (19,0,8,0));
        map.insert("NAMCOT-3301", (0,0,0,0));
        map.insert("NAMCOT-3302", (0,0,0,0));
        map.insert("NAMCOT-3303", (0,0,0,0));
        map.insert("NAMCOT-3304", (0,0,0,0));
        map.insert("NAMCOT-3305", (0,0,0,0));
        map.insert("NAMCOT-3311", (0,0,0,0));
        map.insert("NAMCOT-3312", (0,0,0,0));
        map.insert("NAMCOT-CNROM+WRAM", (3,2,2,0));
        map.insert("NES-NTBROM", (68,1,8,0));
        map.insert("SUNSOFT_UNROM", (93,0,0,8));
        // boards made by unlicensed and bootleg publishers
        map.insert("SL1632", (14,0,0,0));
        map.insert("AC-08", (42,0,0,8));
        map.insert("LH-09", (42,0,0,0));
        map.insert("SUPERVISION16IN1", (53,0,0,8));
        map.insert("SUPERHIK8IN1", (45,0,0,0));
        map.insert("STREETFIGTER-GAME4IN1", (49,0,0,0));
        map.insert("MARIO1-MALEE2", (42,0,2,0));
        map.insert("D1038", (59,0,0,0));
        map.insert("T3H53", (59,0,0,0));
        map.insert("SA-016-1M", (79,0,0,0));
        map.insert("VRC7", (85,0,0,0));
        map.insert("SC-127", (90,0,8,0));
        map.insert("BB", (108,0,0,0));
        map.insert("SL12", (108,0,0,0));
        map.insert("H2288", (123,0,0,0));
        map.insert("22211", (132,0,0,0));
        map.insert("SA-72008", (133,0,0,0));
        map.insert("T4A54A", (134,0,0,0));
        map.insert("SACHEN-8259D", (137,0,0,0));
        map.insert("SACHEN-8259B", (138,0,0,0));
        map.insert("SACHEN-8259C", (139,0,0,0));
        map.insert("SACHEN-8259A", (141,0,0,0));
        map.insert("KS7032", (142,0,0,0));
        map.insert("SA-NROM", (143,0,0,0));
        map.insert("SA-72007", (145,0,0,0));
        map.insert("TC-U01-1.5M", (147,0,0,0));
        map.insert("SA-0037", (148,0,0,0));
        map.insert("SA-0036", (149,0,0,0));
        map.insert("SACHEN-74LS374N", (150,0,0,0));
        map.insert("FS304", (162,0,8,8));
        map.insert("SUPER24IN1SC03", (176,0,0,8));
        map.insert("FK23C", (176,0,0,256));
        map.insert("FK23CA", (176,0,0,256));
        map.insert("WAIXING-FS005", (176,0,32,8));
        map.insert("NOVELDIAMOND9999999IN1", (201,0,0,0));
        map.insert("JC-016-2", (205,0,0,0));
        map.insert("8237", (215,0,0,0));
        map.insert("8237A", (215,1,0,0));
        map.insert("N625092", (221,0,0,0));
        map.insert("GHOSTBUSTERS63IN1", (226,0,0,0));
        map.insert("42IN1RESETSWITCH", (233,0,0,0));
        map.insert("150IN1A", (235,0,0,0));
        map.insert("212-HONG-KONG", (235,0,0,0));
        map.insert("70IN1", (236,0,0,0));
        map.insert("70IN1B", (236,0,0,0));
        map.insert("603-5052", (238,0,0,0));
        map.insert("WAIXING-FW01", (227,0,8,0));
        map.insert("43272", (227,0,8,0));
        map.insert("ONEBUS", (256,0,8,0));
        map.insert("DANCE", (256,0,0,0));
        map.insert("PEC-586", (257,0,8,0));
        map.insert("158B", (258,0,0,0));
        map.insert("F-15", (259,0,0,0));
        map.insert("HPXX", (260,0,8,0));
        map.insert("HP2018-A", (260,0,8,0));
        map.insert("810544-C-A1", (261,0,0,0));
        map.insert("SHERO", (262,0,0,8));
        map.insert("KOF97", (263,0,0,0));
        map.insert("YOKO", (264,0,0,0));
        map.insert("T-262", (265,0,0,8));
        map.insert("CITYFIGHT", (266,0,0,0));
        map.insert("COOLBOY", (268,0,0,256));
        map.insert("MINDKIDS", (268,0,8,256));
        map.insert("22026", (271,0,0,0));
        map.insert("80013-B", (274,0,0,8));
        map.insert("GKCXIN1", (288,0,0,0));
        map.insert("GS-2004", (283,0,0,8));
        map.insert("GS-2004", (283,0,0,8));
        map.insert("A65AS", (285,0,0,8));
        map.insert("BS-5", (286,0,0,0));
        map.insert("411120-C", (287,0,0,0));
        map.insert("K-3088", (287,0,0,0));
        map.insert("60311C", (289,0,0,8));
        map.insert("NTD-03", (290,0,0,0));
        map.insert("DRAGONFIGHTER", (292,0,0,0));
        map.insert("13IN1JY110", (295,0,0,8));
        map.insert("TF1201", (298,0,0,0));
        map.insert("11160", (299,0,0,0));
        map.insert("190in1", (300,0,0,0));
        map.insert("8157", (301,0,0,8));
        map.insert("KS7057", (302,0,0,8));
        map.insert("KS7017", (303,0,8,8));
        map.insert("SMB2J", (304,0,0,0));
        map.insert("KS7031", (305,0,0,8));
        map.insert("KS7016", (306,0,0,8));
        map.insert("KS7037", (307,0,8,8));
        map.insert("TH2131-1", (308,0,0,0));
        map.insert("LH51", (309,0,8,8));
        map.insert("LH32", (125,0,8,8));
        map.insert("KS7013B", (312,0,0,8));
        map.insert("RESET-TXROM", (313,0,0,0));
        map.insert("64IN1NOREPEAT", (314,0,0,0));
        map.insert("830134C", (315,0,0,0));
        map.insert("HP898F", (319,0,0,0));
        map.insert("830425C-4391T", (320,0,0,8));
        map.insert("K-3033", (322,0,0,0));
        map.insert("MALISB", (325,0,0,0));
        map.insert("10-24-C-A1", (327,0,8,8));
        map.insert("RT-01", (328,0,0,0));
        map.insert("EDU2000", (329,0,32,8));
        map.insert("12-IN-1", (331,0,0,0));
        map.insert("WS", (332,0,0,0));
        map.insert("NEWSTAR-GRM070-8IN1", (333,0,0,0));
        map.insert("8-IN-1", (333,0,0,0));
        map.insert("CTC-09", (335,0,0,0));
        map.insert("K-3046", (336,0,0,8));
        map.insert("CTC-12IN1", (337,0,0,8));
        map.insert("SA005-A", (338,0,0,0));
        map.insert("K-3006", (339,0,0,0));
        map.insert("K-3036", (340,0,0,8));
        map.insert("TJ-03", (341,0,0,0));
        map.insert("GN-26", (344,0,0,0));
        map.insert("L6IN1", (345,0,0,0));
        map.insert("KS7012\"", (346,0,8,8));
        map.insert("KS7030", (347,0,8,8));
        map.insert("830118C", (348,0,0,0));
        map.insert("G-146", (349,0,0,8));
        map.insert("891227", (350,0,0,8));
        map.insert("3D-BLOCK", (355,0,0,8));
        map.insert("SA-9602B", (513,0,0,32));
        map.insert("DANCE2000", (518,0,8,8));
        map.insert("EH8813A", (519,0,0,0));
        map.insert("DREAMTECH01", (521,0,0,8));
        map.insert("LH10", (522,0,8,8));
        map.insert("900218", (524,0,0,0));
        map.insert("KS7021A", (525,0,0,0));
        map.insert("BJ-56", (526,0,8,0));
        map.insert("AX-40G", (527,0,0,0));
        map.insert("831128C", (528,0,8,0));
        map.insert("T-230", (529,0,0,0));
        map.insert("AX5705", (530,0,0,0));
        // homebrew boards
        map.insert("COOLGIRL", (342,0,32,256));
        map.insert("DRIPGAME", (284,0,8,0));
        map.insert("FARID_SLROM_8-IN-1", (323,0,0,0));
        map.insert("FARID_UNROM_8-IN-1", (324,0,0,8));
        map.insert("RET-CUFROM", (29,0,0,32));

        map
    };
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum NametableMirroring {
    Horizontal = 0,
    Vertical = 1,
    ScreenAOnly = 2,
    ScreenBOnly = 3,
    FourScreens = 4,
    MapperControlled = 5,
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum TvSystem {
    NTSC = 0,
    PAL = 1,
    MultRegion = 2,
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum GameInputType {
    Unspecified = 0,
    StandardController = 1,
    Zapper = 2,
    ROB = 4,
    Arkanoid = 8,
    PowerPad = 16,
    FourScore = 32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unif {
    pub name: Option<String>,
    pub read: Option<String>,
    pub tvci: Option<TvSystem>,
    pub ctrl: Option<GameInputType>,
    pub batr: Option<bool>,
    pub vror: Option<u8>,
    pub mirr: Option<NametableMirroring>,
    pub mapr: String,
    pub prg_crc: u32,
    pub chr_crc: u32,
    pub prg_data: Vec<u8>,
    pub chr_data: Vec<u8>,
}

impl Unif {
    pub fn new() -> Unif {
        Unif {
            name: None,
            read: None,
            tvci: None,
            ctrl: None,
            batr: None,
            vror: None,
            mirr: None,
            prg_crc: 0,
            chr_crc: 0,
            mapr: String::new(),
            prg_data: Vec::new(),
            chr_data: Vec::new(),
        }
    }

    pub fn from_rom<R: Read + Seek>(mut file: R) -> Result<Unif, RomError> {
        let mut buf: [u8; 32] = [0; 32];
        file.read_exact(&mut buf)?;
        // check header
        if &buf[0..4] != super::UNIF_GUARD {
            return Err(RomError::InvalidFormat);
        }  

        let file_size = file.seek(io::SeekFrom::End(0))?;
        file.seek(io::SeekFrom::Start(32))?;

        // used to make rom chunks are appended in correct order
        let mut prg_chunks: [Vec<u8>; 16] = Default::default();
        let mut chr_chunks: [Vec<u8>; 16] = Default::default();
        let mut unif = Unif::new();
        
        while read_chunk(&mut file, &mut unif, &mut prg_chunks, &mut chr_chunks, file_size)? {

        }

        // copy finalized rom data
        for chunk in &mut prg_chunks {
            unif.prg_data.append(&mut *chunk);
        }

        for chunk in &mut chr_chunks {
            unif.chr_data.append(&mut *chunk);
        }

        unif.prg_crc = crc32::crc32_ieee_from_slice(&unif.prg_data);     
        unif.chr_crc = crc32::crc32_ieee_from_slice(&unif.chr_data); 

        if unif.mapr.is_empty() || unif.prg_data.is_empty() || unif.chr_data.is_empty() {
            return Err(RomError::InvalidRom);
        }
        Ok(unif)
    }

    // tranfers ownership, maybe useful since rom files can be very large
    pub fn into_ines(self) ->Result<ines::Ines, RomError> {
        let mut i = ines::Ines::new();
        i.header_version = ines::HeaderVersion::Nes2;

        let (mapper, submapper, prg_ram, chr_ram) = get_mapper_id(self.mapr.as_str())?;
        i.mapper = mapper;
        i.submapper = submapper;
        i.ram = ines::Ram::Nes2{prg_ram: prg_ram, prg_nvram: 0, chr_ram: chr_ram, chr_nvram: 0};

        match self.batr {
            Some(contains_nvram) => i.contains_nvram = contains_nvram,
            None => (),
        }

        let tv: TvSystem = self.tvci.unwrap_or(TvSystem::NTSC);
        match tv {
            TvSystem::NTSC => i.tv_system = ines::TvSystem::NTSC,
            TvSystem::PAL => i.tv_system = ines::TvSystem::PAL,
            TvSystem::MultRegion => i.tv_system = ines::TvSystem::MultRegion,
        }

        let ctrl: GameInputType = self.ctrl.unwrap_or(GameInputType::Unspecified);
        match ctrl {
            GameInputType::Unspecified => i.game_input = ines::GameInputType::Unspecified,
            GameInputType::StandardController => i.game_input = ines::GameInputType::StandardControllers,
            GameInputType::Zapper => i.game_input = ines::GameInputType::Zapper,
            GameInputType::ROB => i.game_input = ines::GameInputType::RobGyroSet,
            GameInputType::Arkanoid => {
                if i.tv_system == ines::TvSystem::NTSC {
                    i.game_input = ines::GameInputType::ArkanoidControllerNes;
                }
                else {
                     i.game_input = ines::GameInputType::ArkanoidControllerFamicom;
                }
            },
            GameInputType::PowerPad => i.game_input = ines::GameInputType::PowerPadSideA,
            GameInputType::FourScore => i.game_input = ines::GameInputType::FourScore,
        }

        let mirr: NametableMirroring = self.mirr.unwrap_or(NametableMirroring::MapperControlled);
        match mirr {
            NametableMirroring::Horizontal => i.nametable_mirroring = ines::NametableMirroring::Horizontal,
            NametableMirroring::Vertical => i.nametable_mirroring = ines::NametableMirroring::Vertical,
            NametableMirroring::FourScreens => i.nametable_mirroring = ines::NametableMirroring::FourScreens,
            _ => i.nametable_mirroring = ines::NametableMirroring::Other,
        }

        i.prg_crc = self.prg_crc;
        i.chr_crc = self.chr_crc;
        i.prg_rom_size = self.prg_data.len() as u32;
        i.chr_rom_size = self.chr_data.len() as u32;
        i.prg_data = self.prg_data;
        i.chr_data = self.chr_data;

        return Ok(i);
    }

    // creates clone, leaves original format intact
     pub fn to_ines(&self) ->Result<ines::Ines, RomError> {
        let mut i = ines::Ines::new();
        i.header_version = ines::HeaderVersion::Nes2;

        let (mapper, submapper, prg_ram, chr_ram) = get_mapper_id(self.mapr.as_str())?;
        i.mapper = mapper;
        i.submapper = submapper;
        i.ram = ines::Ram::Nes2{prg_ram: prg_ram, prg_nvram: 0, chr_ram: chr_ram, chr_nvram: 0};

        match self.batr {
            Some(contains_nvram) => i.contains_nvram = contains_nvram,
            None => (),
        }

        let tv: TvSystem = self.tvci.unwrap_or(TvSystem::NTSC);
        match tv {
            TvSystem::NTSC => i.tv_system = ines::TvSystem::NTSC,
            TvSystem::PAL => i.tv_system = ines::TvSystem::PAL,
            TvSystem::MultRegion => i.tv_system = ines::TvSystem::MultRegion,
        }

        let ctrl: GameInputType = self.ctrl.unwrap_or(GameInputType::Unspecified);
        match ctrl {
            GameInputType::Unspecified => i.game_input = ines::GameInputType::Unspecified,
            GameInputType::StandardController => i.game_input = ines::GameInputType::StandardControllers,
            GameInputType::Zapper => i.game_input = ines::GameInputType::Zapper,
            GameInputType::ROB => i.game_input = ines::GameInputType::RobGyroSet,
            GameInputType::Arkanoid => {
                if i.tv_system == ines::TvSystem::NTSC {
                    i.game_input = ines::GameInputType::ArkanoidControllerNes;
                }
                else {
                     i.game_input = ines::GameInputType::ArkanoidControllerFamicom;
                }
            },
            GameInputType::PowerPad => i.game_input = ines::GameInputType::PowerPadSideA,
            GameInputType::FourScore => i.game_input = ines::GameInputType::FourScore,
        }

        let mirr: NametableMirroring = self.mirr.unwrap_or(NametableMirroring::MapperControlled);
        match mirr {
            NametableMirroring::Horizontal => i.nametable_mirroring = ines::NametableMirroring::Horizontal,
            NametableMirroring::Vertical => i.nametable_mirroring = ines::NametableMirroring::Vertical,
            NametableMirroring::FourScreens => i.nametable_mirroring = ines::NametableMirroring::FourScreens,
            _ => i.nametable_mirroring = ines::NametableMirroring::Other,
        }

        i.prg_crc = self.prg_crc;
        i.chr_crc = self.chr_crc;
        i.prg_rom_size = self.prg_data.len() as u32;
        i.chr_rom_size = self.chr_data.len() as u32;
        i.prg_data = self.prg_data.clone();
        i.chr_data = self.chr_data.clone();

        return Ok(i);
    }
}

fn read_chunk<R: Read + Seek>(file: &mut R, unif: &mut Unif, prg_chunks: &mut [Vec<u8>], chr_chunks: &mut [Vec<u8>], end_of_file: u64) -> Result<bool, RomError> {
    let mut type_buf =  [0u8; 4];
    let mut len_buf = [0u8; 4];
    file.read_exact(&mut type_buf)?;
    file.read_exact(&mut len_buf)?;

    let chunk_type = unsafe {
        str::from_utf8_unchecked(&type_buf)
    };

    let len = get_chunk_len(&len_buf);
    let mut chunk_buf = vec![0u8; len as usize];    

    if chunk_type == "MAPR" {
        file.read_exact(&mut chunk_buf)?;
        unif.mapr = unsafe {
            // unif uses null terminated utf-8 strings
            String::from_utf8_unchecked(chunk_buf).trim_end_matches(char::from(0)).to_string()
        };
    }
    else if chunk_type == "NAME" {
        file.read_exact(&mut chunk_buf)?;
        unif.name = Some(unsafe {
            String::from_utf8_unchecked(chunk_buf).trim_end_matches(char::from(0)).to_string()
        });
    }
    else if chunk_type == "READ" {
        file.read_exact(&mut chunk_buf)?;
        unif.read = Some(unsafe {
            String::from_utf8_unchecked(chunk_buf).trim_end_matches(char::from(0)).to_string()
        });
    }
    else if chunk_type.contains("PRG") {
        file.read_exact(&mut chunk_buf)?;
        // the type identifier fixed 4 bytes with last byte representing hex value
        let index = get_index_from_hex_ascii(type_buf[3])?;
        prg_chunks[index] = chunk_buf.to_vec();

    }
    else if chunk_type.contains("CHR") {
        file.read_exact(&mut chunk_buf)?;
        // the type identifier fixed 4 bytes with last byte representing hex value
        let index = get_index_from_hex_ascii(type_buf[3])?;
        chr_chunks[index] = chunk_buf.to_vec();
    }
    else if chunk_type == "TVCI" {
        file.read_exact(&mut chunk_buf)?;
        unif.tvci = TvSystem::from_u8(chunk_buf[0]);
    }
    else if chunk_type == "CTRL" {
        file.read_exact(&mut chunk_buf)?;
        unif.ctrl = GameInputType::from_u8(chunk_buf[0]);
    }
    else if chunk_type == "MIRR" {
        file.read_exact(&mut chunk_buf)?;
        unif.mirr = NametableMirroring::from_u8(chunk_buf[0]);
    }
    else if chunk_type == "BATR" {
        file.read_exact(&mut chunk_buf)?;
        unif.batr = match chunk_buf[0] {
            0 => Some(false),
            _ => Some(true),
        };
    }
    else if chunk_type == "VROR" {
        file.read_exact(&mut chunk_buf)?;
        unif.vror = Some(chunk_buf[0]);
    }
    else {
        file.read_exact(&mut chunk_buf)?;
    }

    let cur_pos = file.seek(io::SeekFrom::Current(0))?;

    if cur_pos < end_of_file {
        Ok(true)
    }
    else {
        Ok(false)
    }
}

fn get_chunk_len(len_buf: &[u8]) -> u32 {
    ((len_buf[0] as u32) | ((len_buf[1] as u32) << 8) | ((len_buf[2] as u32) << 16) | ((len_buf[3] as u32) << 24))
}

fn get_index_from_hex_ascii(byte: u8) -> Result<usize, RomError> {
    match byte {
        48 => Ok(0),
        49 => Ok(1),
        50 => Ok(2),
        51 => Ok(3),
        52 => Ok(4),
        53 => Ok(5),
        54 => Ok(6),
        55 => Ok(7),
        56 => Ok(8),
        57 => Ok(9),
        97 | 65 => Ok(10),
        98 | 66 => Ok(11),
        99 | 67 => Ok(12),
        100 | 68 => Ok(13),
        101 | 69 => Ok(14),
        102 | 70 => Ok(15),
        _ => Err(RomError::InvalidRom),
    }
}

fn get_mapper_id(mapr: &str) -> Result<(u32, u8, u32, u32), RomError> {
    let mapr_str = mapr.trim_start_matches("NES-").trim_start_matches("HVC-").trim_start_matches("UNL-").
        trim_start_matches("BTL-").trim_start_matches("HVC-").trim_start_matches("BMC-").trim_start_matches("IREM-");
    
    match UNIF_BOARD_MAPPINGS.get(mapr_str) {
            Some(info) => Ok(*info),
            None => return Err(RomError::InvalidConversion),
        }
}