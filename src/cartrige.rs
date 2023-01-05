// Basicly mapper 0 right now
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::error::Error;
use std::fmt::format;

enum RomType {
    INES,
    NES20,
    Invalid
}

pub struct Cart {
    pub rom: [u8; 0x8000], // 32 KB of ROM
    r_type: RomType,
}

impl Cart {
    pub fn new() -> Self {
        Self {
            rom: [0u8; 0x8000],
            r_type: RomType::Invalid,
        }
    }

    fn load_file(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        // Loads the file
        let mut file = File::open(filename)?;
        let size = file.metadata()?.len() as usize;
        // Reads the data into the buffer
        let mut buffer = vec![0; size];
        file.read(&mut buffer)?;

        // Determines rom type
        if buffer[0] == 'N' as u8 && buffer[1] == 'E' as u8 && buffer[2] == 'S' as u8 && buffer[3] == 0x1A {
            if buffer[7] & 0x0C == 0x08 {
                self.r_type = RomType::NES20;
            } else {
                self.r_type = RomType::INES;
            }
            return Ok(());
        }

        return Err("Invalid NES Rom")?;
    }

    fn pgr_lsb(&self) -> u8 { 
        self.rom[4]
    }
    fn chr_lsb(&self) -> u8 {
        self.rom[5]
    }
    fn mir_type(&self) -> u8 {
        // Hori/Map = 0; Vert = 1
        self.rom[6] & 0b00000001
    } 
    fn bat_get(&self) -> u8 {
        // Battery and other non volatile memory
        self.rom[6] & 0b00000010
    }
    fn has_trainer(&self) -> u8 {
        // Determines if there is a 512 byte trainer
        self.rom[6] & 0b00000100
    }
    fn four_screen_mode(&self) -> u8 {
        // https://www.nesdev.org/wiki/NES_2.0#Hard-Wired_Mirroring
        self.rom[6] & 0b00001000
    }
    fn console_type(&self) -> u8 {
        // 0: NES; 1: Vs.; 2: Playchoice; 3: Extended
        self.rom[7] & 0b00000011
    }
}
