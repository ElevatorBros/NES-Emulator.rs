// Basicly mapper 0 right now
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
    /// Magic bytes 'NES<EOF>'
    magic: [u8; 4],
    /// Program size
    prg_size: u16,
    /// Character data size
    chr_size: u16,
    // Miscellaneous stuff
    /// Vertical or horizontal mirroring
    ///     0: Horizontal/Mapper
    ///     1: Vertical
    mirror: bool,
    /// Battery and other non volatile memory
    battery: bool,
    /// Bool if 512 byte trainer header exists between header and PRG-ROM
    trainer: bool,
    /// Hard wired four screen mode
    four_screen: bool,
    /// Console type
    ///     0: Nes/Famicom 
    ///     1: Vs 
    ///     2: Playchoice
    ///     3: Extended
    console: u8,
    flag13: u8,
}

pub struct Cart {
    pub header: NesHeader,
    pub prg: Vec<u8>,
    pub chr: Vec<u8>,
    pub rtype: RomType,
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
            magic      :  [buffer[0], buffer[1], buffer[2], buffer[3]],
            prg_size   :  0,
            chr_size   :  0,
            mirror     :  (buffer[6] & 0b00000001) != 0,
            battery    :  (buffer[6] & 0b00000010) != 0,
            trainer    :  (buffer[6] & 0b00000100) != 0,
            four_screen:  (buffer[6] & 0b00001000) != 0,
            console    :  (buffer[6] & 0b00000011),
            flag13     :  buffer[13]
        };
        let prg_msb = buffer[9] & 0b00001111;
        let prg_lsb = buffer[4];
        let chr_msb = (buffer[9] & 0b11110000) >> 4;
        let chr_lsb = buffer[5];

        // Determines rom type
        let rtype = RomType::Invalid;
        if header.magic == *b"NES\x1a" {
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
            if mul >= 64 {
                return Err(format!("{mul} is too large. Maybe there's an error with bitwise math or something here or cringe rust stuff, idk."))?;
            }
            let can = (prg_lsb & 0b00000011) * 2 + 1;
            header.prg_size = mul as u16 * can as u16;
        } else {
            // FIXME: For some reason `(buffer[9] as u16 << 8) | (buffer[4] as u16)` yields incorrect
            // results
            header.prg_size = (buffer[9] as u16) << 8;
            header.prg_size |= buffer[4] as u16;
        }
        if chr_msb == 0xFu8 {
            // 2 ** prg_lsb >> 2
            let mul = 1 << (chr_lsb >> 2);
            if mul >= 64 {
                return Err(format!("{mul} is too large. Maybe there's an error with bitwise math or something here or cringe rust stuff, idk."))?;
            }
            let can = (chr_lsb & 0b00000011) * 2 + 1;
            header.chr_size = mul as u16 * can as u16;
        } else {
            // FIXME: For some reason `(buffer[9] as u16 << 8) | (buffer[4] as u16)` yields incorrect
            // results
            header.chr_size = (buffer[9] as u16) << 8;
            header.chr_size |= buffer[5] as u16;
        }

        // Ensure the data is valid
        return Ok(Self { 
            header, 
            prg: vec![0; header.prg_size as usize], 
            chr: vec![0; header.chr_size as usize], 
            rtype 
        });
    }

    pub fn read(&mut self, addr: u16) -> u16 {
        unimplemented!()
    }
}
