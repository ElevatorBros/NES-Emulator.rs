// Basicly mapper 0 right now
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::utils;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::error::Error;

pub struct NesHeader {
    /// Raw Data
    data: [u8; 16],
}

pub struct Cart {
    /// The Header data for the Cartridge
    pub header: NesHeader,
    /// Contains the chr data
    pub chr: Vec<u8>,
    /// Contains the prg data
    pub prg: Vec<u8>,
    /// The Trainer Area follows the 16-byte Header and precedes the PRG-ROM area if bit 2 of Header byte 6 is set. It is always 512 bytes in size if present, and contains data to be loaded into CPU memory at $7000. It is only used by some games that were modified to run on different hardware from the original cartridges, such as early RAM cartridges and emulators, and which put some additional compatibility code into those address ranges. 
    pub trainer: Option<[u8; 512]>,
}

impl NesHeader {
    /// Mirroring: 
        /// 0: horizontal (vertical arrangement) (CIRAM A10 = PPU A11)
        /// 1: vertical (horizontal arrangement) (CIRAM A10 = PPU A10)
    fn mirror(&self) -> bool { (self.data[6] & 1) != 0 }
    /// Returns true Cartridge contains battery-backed PRG RAM ($6000-7FFF) or other persistent memory
    fn battery(&self) -> bool { (self.data[6] & (1 << 1)) != 0 }
    /// If trainer data exists
    fn trainer(&self) -> bool { (self.data[6] & (1 << 2)) != 0 }
    /// Ignore mirror control; instead four screen vram is provided
    fn four_screen(&self) -> bool { (self.data[6] & (1 << 3)) != 0 }
    /// Gets the mapper number
    fn mapper(&self) -> u8 {
        (self.data[7] & 0b1111000) | ((self.data[6] & 0b11110000) >> 4)
    }
    fn unisystem(&self) -> bool { (self.data[7] & 1) != 0 }
    /// PlayChoice-10 (8 KB of Hint Screen data stored after CHR data)
        /// In the context of this emulator, this data is simply ignored
    fn playchoice(&self) -> bool { (self.data[7] & (1 << 1)) != 0 }
    /// Determines if this is in the nes2.0 format
        /// This is simply for bookeeping
    fn nes2(&self) -> bool {((self.data[7] & 0b00001100) >> 2)  == 2 }
    /// Determines size of the prg ram
    fn prg_ram(&self) -> u8 {
        if self.data[8] == 0 {
            return 8192
        } else {
            self.data[8] * 8192
        }
    }
    /// NTSC vs PAL
        /// 0: NTSC
        /// 1: PAL
        /// Note: No ROM images in circulation make use of this bit
    fn tv_system(&self) -> bool { (self.data[9] & 1) != 0 }
    /// Usually zeroed out data but sometimes may contain ripper's name or something
    fn ripper_name(&self) -> [u8; 7] {
        // Rust, my beloved, why
        self.data[7..=15].try_into().expect("Invalid length")
    }
    /// Gets the size of the prg rom
    fn prg_size(&self) -> u8 { self.data[4] * 16384 }
    /// Gets the size of the chr rom
        /// A value of 0 indicates that the board uses chr-ram
    fn chr_size(&self) -> u8 { self.data[5] * 8192 }
}

impl Cart {
    pub fn new(filename: &str) -> Result<Self, Box<dyn Error>> {
        // Loads the file
        let mut file = File::open(filename)?;
        let size = file.metadata()?.len() as usize;
        // Reads the data into the buffer
        let mut buffer = vec![0; size];
        file.read(&mut buffer)?;
        let mut header = NesHeader {
            prg_size   : 0,
            chr_size   : 0,
            data       : buffer[0..=15].try_into().expect("Invalid length")
        };
        let prg_msb = buffer[9] & 0b00001111;
        let prg_lsb = buffer[4];
        let chr_msb = (buffer[9] & 0b11110000) >> 4;
        let chr_lsb = buffer[5];

        // Determines rom type
        let rtype: RomType;
        if header.data[0..=3] == *b"NES\x1a" {
            if buffer[7] & 0x0C == 0x08 {
                rtype = RomType::NES20;
            } else {
                rtype = RomType::INES;
            }
        } else {
            return Err(format!("{filename} is not a valid rom. File does not have magic 'NES<EOF>' bytes"))?;
        }

        // If MSB nibble is $F, then prg and chr size is calculated like so:
        if prg_msb == 0xFu8 {
            // 2 ** prg_lsb >> 2
            let mul = 1 << (prg_lsb >> 2);
            if prg_lsb > 128 {
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
            if chr_lsb > 128 {
                return Err(format!("{chr_lsb} is too large. Maybe there's an error with bitwise math or something here or cringe rust stuff, idk."))?;
            }
            let can = (chr_lsb & 0b00000011) * 2 + 1;
            header.chr_size = mul as u16 * can as u16;
        } else {
            header.chr_size = (prg_msb as u16) | (prg_lsb as u16);
        }

        /*
        let trainer = vec![0; 512];
        let prg = vec![0; header.prg_size as usize];
        let chr = vec![0; header.chr_size as usize];

        let pointer: usize = 0;
        if header.trainer() {
            utils::readbuf(&trainer, &buffer, &pointer, 512);
        }
        utils::readbuf(&prg, &buffer, &pointer, header.prg_size as usize);
        utils::readbuf(&chr, &buffer, &pointer, header.chr_size as usize);
       */
        let mut trainer = vec![0; 512];
        let mut ptr: usize = 0;
        if header.trainer() {
            utils::readbuf(&mut trainer, &mut buffer, &mut ptr, 512);
        }

        // Ensure the data is valid
        return Ok(Self { 
            trainer,
            header, 
            rom: buffer,
            rtype 
        });
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
}
