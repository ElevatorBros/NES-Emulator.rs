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
    fn prg_ram(&self) -> usize {
        if self.data[8] == 0 {
            return 8192
        } else {
            (self.data[8] as usize) * 8192
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
    fn prg_size(&self) -> usize { self.data[4] as usize * 16384 }
    /// Gets the size of the chr rom
        /// A value of 0 indicates that the board uses chr-ram
    fn chr_size(&self) -> usize { self.data[5] as usize * 8192 }
    /// Checks to see if the magic bytes of the rom are accurrate
    fn magic(&self) -> bool {
        self.data[0..=3] == *b"NES\x1a"
    }
}

impl Cart {
    pub fn new(filename: &str) -> Result<Self, Box<dyn Error>> {
        // Loads the file
        let mut file = File::open(filename)?;
        let size = file.metadata()?.len() as usize;
        let mut buffer = vec![0u8; size];
        file.read(&mut buffer)?;

        // Reads the header data
        let data = buffer[0..=15].try_into()?;
        let mut ptr = 16;
        let header = NesHeader { data };
        if !header.magic() {
            return Err("ROM does not contain magic bytes")?;
        }

        let trainer = None;
        if header.trainer() {
            let trainer_data: [u8; 512] = buffer[ptr..=(ptr + 512)].try_into()?;
            let trainer = Some(trainer_data);
            ptr += 512;
        }

        let mut prg = vec![0u8; header.prg_size()];
        let mut chr = vec![0u8; header.chr_size()];
        utils::readbuf_vec(&mut prg, &mut buffer, &mut ptr, header.prg_size());
        utils::readbuf_vec(&mut chr, &mut buffer, &mut ptr, header.chr_size());

        return Ok(Cart{
            header,
            trainer,
            prg,
            chr
        });
    }

    pub fn read(&self, addr: u16) -> u8 {
        // mapper 0
        if addr < 0x8000 { // not dealt with
             return 0; 
        } else if addr < 0xC000 { // CHR ROM
            let mut raw_addr:u16 = addr;
            raw_addr -= 0x8000;
            raw_addr %= self.chr.len() as u16;
            return self.chr[raw_addr as usize];
        } else { // PRG ROM
            let mut raw_addr:u16 = addr;
            raw_addr -= 0xC000;
            raw_addr %= self.prg.len() as u16;
            //raw_addr %= 0x2000; // Mirrored every 8kb
            return self.prg[raw_addr as usize]
        }
    }
}
