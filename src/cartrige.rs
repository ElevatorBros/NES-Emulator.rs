// Basicly mapper 0 right now
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::error::Error;
use std::fmt::format;

enum RomType {
    INES,
    NES20,
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
    pub fn load(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        // Loads the file
        let mut file = File::open(filename)?;
        let size = file.metadata()?.len() as usize;
        // Reads the data into the buffer
        let mut buffer = vec![0; size];
        file.read(&mut buffer)?;
        self.header = NesHeader {
            magic      :  [buffer[0], buffer[1], buffer[2], buffer[3]],
                          // MSB                            LSB
            prg_size   :  (buffer[9] & 0b00001111) as u16 | (buffer[4] as u16),
            chr_size   :  (buffer[9] & 0b11110000) as u16 | (buffer[5] as u16),
            mirror     :  (buffer[6] & 0b00000001) != 0,
            battery    :  (buffer[6] & 0b00000010) != 0,
            trainer    :  (buffer[6] & 0b00000100) != 0,
            four_screen:  (buffer[6] & 0b00001000) != 0,
            console    :  (buffer[6] & 0b00000011),
            flag13     :  buffer[13]
        };

        // Determines rom type
        if self.header.magic == *b"NES\x1a" {
            if buffer[7] & 0x0C == 0x08 {
                self.rtype = RomType::NES20;
            } else {
                self.rtype = RomType::INES;
            }
        } else {
            return Err(format!("{filename} is not a valid rom. File does not have magic 'NES<EOF>' bytes"))?;
        }

        return Err("what")?;
    }
}
