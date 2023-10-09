#![allow(dead_code)]
#![allow(unused_variables)]
pub struct Ram {
    pub cpu_memory: [u8; 0x800], // 2KB internal RAM
    pub ppu_memory: [u8; 0x2000],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            cpu_memory: [0; 0x800],
            ppu_memory: [0; 0x2000],
        }
    }

    pub fn get_cpu_memory(&mut self, addr: u16) -> u8 {
        self.cpu_memory[addr as usize]
    }

    pub fn set_cpu_memory(&mut self, addr: u16, value: u8) {
        self.cpu_memory[addr as usize] = value;
    }

    // Note addrs will come in -0x2000
    pub fn get_ppu_memory(&mut self, addr: u16) -> u8 {
        let mut actual_addr = addr;
        if actual_addr >= 0x1F00 && actual_addr < 0x1F20 {
            // Pallet
            if actual_addr == 0x1F10
                || actual_addr == 0x1F14
                || actual_addr == 0x1F18
                || actual_addr == 0x1F1C
            {
                actual_addr -= 0x10;
            }
        }
        self.ppu_memory[actual_addr as usize]
    }

    pub fn set_ppu_memory(&mut self, addr: u16, value: u8) {
        self.ppu_memory[addr as usize] = value;
    }
}
