// Basicly mapper 0 right now
use std::fs::File;
use std::io::{prelude::*, Result};

pub struct Cart {
    pub ROM: [u8; 0x8000] // 32 KB of ROM
}

impl Cart {
    pub fn new() -> Self {
        Self {ROM:[0; 0x8000]}
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
}
