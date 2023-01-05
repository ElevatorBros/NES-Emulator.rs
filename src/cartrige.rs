// Basicly mapper 0 right now
use crate::utils;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::error::Error;

enum RomType {
    INES,
    NES20,
    Invalid
}

pub struct NesHeader {
    /// Program size
    prg_size: u16,
    /// Character data size
    chr_size: u16,
    /// Raw Data
    data: [u8; 16],
}

pub struct Cart {
    /// The Header data for the Cartridge
    pub header: NesHeader,
    /// Contains all the data in the program area
    pub prg: Vec<u8>,
    /// Contains all the data in the character area
    pub chr: Vec<u8>,
    // TODO: Make trainer an Option<[u8; 512]>
    /// The Trainer Area follows the 16-byte Header and precedes the PRG-ROM area if bit 2 of Header byte 6 is set. It is always 512 bytes in size if present, and contains data to be loaded into CPU memory at $7000. It is only used by some games that were modified to run on different hardware from the original cartridges, such as early RAM cartridges and emulators, and which put some additional compatibility code into those address ranges. 
    pub trainer: Vec<u8>,
    /// Either INES or NES2.0
    pub rtype: RomType,
}

impl NesHeader {
    /// Bool if 512 byte trainer header exists between header and PRG-ROM
    fn trainer(&self) -> bool { self.data[6] & (1 << 2) != 0 } 
    /// Vertical or horizontal mirroring
    ///     0: Horizontal/Mapper
    ///     1: Vertical
    fn mirror(&self) -> bool { self.data[6] & 1 != 0 }
    /// Battery and other non volatile memory
    fn battery(&self) -> bool { self.data[6] & 2 != 0}
    /// Console type
    ///     0: Nes/Famicom 
    ///     1: Vs 
    ///     2: Playchoice
    ///     3: Extended
    fn console(&self) -> u8 { self.data[7] & 3 }
    fn prg_ram(&self) -> u8 { self.data[10] & 0b00001111 }
    fn prg_nvram(&self) -> u8 { self.data[10] & 0b11110000 }
    fn chr_ram(&self) -> u8 { self.data[11] & 0b00001111 }
    fn chr_nvram(&self) -> u8 { self.data[11] & 0b11110000 }
    /// CPU/PPU timing mode
    ///      0: RP2C02 ("NTSC NES")
    ///      1: RP2C07 ("Licensed PAL NES")
    ///      2: Multiple-region
    ///      3: UMC 6527P ("Dendy")
    fn timing(&self) -> u8 { self.data[12] & 3 }
    /// VS PPU
    fn vs_ppu(&self) -> u8 { 
        if self.console() == 1 {
            self.data[13] & 0b00001111 
        } else {
            0
        }
    }
    /// Extended Console Type
    fn ext_console_type(&self) -> u8 { 
        if self.console() == 3 {
            self.data[13] & 0b00001111 
        } else {
            0
        }
    }
    /// VS Hardware
    fn vs_hw(&self) -> u8 { self.data[13] & 0b11110000 }
    /// Number of miscellaneous roms present
    fn misc_roms(&self) -> u8 { self.data[14] & 3 }
    /// Default expansion device
    fn def_expansion_device(&self) -> u8 { self.data[15] & 0b00111111}
}

impl Cart {
    pub fn new(filename: &str) -> Result<Self, Box<dyn Error>> {
        // Loads the file
        let mut file = File::open(filename)?;
        let size = file.metadata()?.len() as usize;
        // Reads the data into the buffer
        let mut buffer = vec![0; size];
        file.read(&mut buffer)?;
        let header = NesHeader {
            prg_size   : 0,
            chr_size   : 0,
            data       : buffer[0..=15].try_into().expect("Invalid length")
        };
        let prg_msb = buffer[9] & 0b00001111;
        let prg_lsb = buffer[4];
        let chr_msb = (buffer[9] & 0b11110000) >> 4;
        let chr_lsb = buffer[5];

        // Determines rom type
        let rtype = RomType::Invalid;
        if header.data[0..=3] == *b"NES\x1a" {
            if buffer[7] & 0x0C == 0x08 {
                let rtype = RomType::NES20;
            } else {
                let rtype = RomType::INES;
            }
        } else {
            return Err(format!("{filename} is not a valid rom. File does not have magic 'NES<EOF>' bytes"))?;
        }

        // If MSB nibble is $F, then prg and chr size is calculated like so:
        if prg_msb == 0xFu8 {
            // 2 ** prg_lsb >> 2
            let mul = 1 << (prg_lsb >> 2);
            if prg_lsb >= 128 {
                return Err(format!("{prg_lsb} is too large. Maybe there's an error with bitwise math or something here or cringe rust stuff, idk."))?;
            }
            let can = (prg_lsb & 0b00000011) * 2 + 1;
            header.prg_size = mul as u16 * can as u16;
        } else {
            header.prg_size = (prg_msb as u16) | (prg_lsb as u16);
        }
        if chr_msb == 0xFu8 {
            // 2 ** prg_lsb >> 2
            let mul = 1 << (chr_lsb >> 2);
            if chr_lsb >= 64 {
                return Err(format!("{chr_lsb} is too large. Maybe there's an error with bitwise math or something here or cringe rust stuff, idk."))?;
            }
            let can = (chr_lsb & 0b00000011) * 2 + 1;
            header.chr_size = mul as u16 * can as u16;
        } else {
            header.chr_size = (prg_msb as u16) | (prg_lsb as u16);
        }

        let trainer = vec![0; 512];
        let prg = vec![0; header.prg_size as usize];
        let chr = vec![0; header.chr_size as usize];
        if header.trainer() {
            utils::readbuf(&trainer, &buffer, 512);
        }
        utils::readbuf(&prg, &buffer, header.prg_size as usize);
        utils::readbuf(&chr, &buffer, header.chr_size as usize);

        // Ensure the data is valid
        return Ok(Self { 
            trainer,
            header, 
            prg,
            chr,
            rtype 
        });
    }

    // TODO: Maybe this should be an indexer. It will check to see where the address is and will
    // index either the prg or chr
    pub fn read(&mut self, addr: u16) -> u16 {
        unimplemented!()
    }
}
