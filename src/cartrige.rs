// Basicly mapper 0 right now
use std::fs::File;
use std::io::{prelude::*, Result};

enum RomType {
    INES,
    NES20,
    Invalid
}

pub struct Cart {
    pub ROM: [u8; 0x8000], // 32 KB of ROM
    pub r_type: RomType
}

impl Cart {
    pub fn new() -> Self {
        Self {
            ROM:[0; 0x8000],
            r_type: RomType::Invalid
        }
    }


    fn load_file(filename: &str) -> Result<Vec<u8>> {
        let file = match File::open(filename) {
            Err(why) => panic!("Couldn't open file: {}. Error: {}", filename, why), 
            Ok(f) => f,
        };
        let size = match file.metadata() {
            Err(why) => panic!("Couldn't open file: {}. Error: {}", filename, why), 
            Ok(data) => data.len() as usize,
        };
        let buffer = vec![0; size];
        file.read(&mut buffer)?;
        Ok(buffer)
    }

    fn determine_type(rom: &[u8]) -> RomType {
        let i_nes = false;
        let nes_20 = false;
        if rom[0] == 'N' as u8 && rom[1] == 'E' as u8 && rom[2] == 'S' as u8 && rom[3] == 0x1A {
            if rom[7] & 0x0C == 0x08 {
                return RomType::NES20
            }
            return RomType::INES;
        }
        RomType::Invalid
    }
}
