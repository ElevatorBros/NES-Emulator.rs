#![allow(dead_code)]
#![allow(unused_variables)]
pub struct Ram {
    pub cpu_memory: [u8; 0x800], // 2KB internal RAM
    pub ppu_memory: [u8; 0x2000], // 8KB, pattern tables on cart 
    pub vertical_mirroring: bool, // Nametable mirroring
}

impl Ram {
    pub fn new(vertical_mirroring: bool) -> Self {
        Self {
            cpu_memory: [0xFF; 0x800],
            ppu_memory: [0xFF; 0x2000],
            vertical_mirroring,
        }
    }

    pub fn get_cpu_memory(&mut self, addr: u16) -> u8 {
        let actual_addr = addr % 0x800;
        self.cpu_memory[actual_addr as usize]
    }

    pub fn set_cpu_memory(&mut self, addr: u16, value: u8) {
        let actual_addr = addr % 0x800;
        self.cpu_memory[actual_addr as usize] = value;
    }

    fn ppu_address_mapping(&self, addr: u16) -> usize {
        let mut actual_addr = addr;
        // Nametable
        if actual_addr >= 0x2000 && actual_addr < 0x3F00 {
            if self.vertical_mirroring {
                if actual_addr >= 0x2800 {
                    actual_addr -= 0x800;
                }
            } else {
                if (actual_addr >= 0x2400 && actual_addr < 0x2800)
                    || (actual_addr >= 0x2C00 && actual_addr < 0x3000)
                {
                    actual_addr -= 0x400;
                }
            }
        } else if actual_addr >= 0x3F00 && actual_addr < 0x3F20 {
            // Pallet
            if actual_addr == 0x3F10
                || actual_addr == 0x3F14
                || actual_addr == 0x3F18
                || actual_addr == 0x3F1C
            {
                actual_addr -= 0x10;
            }
        }
        // We are only handling nametables here, so shift that address down
        actual_addr -= 0x2000;
        actual_addr as usize
    }

    pub fn get_ppu_memory(&self, addr: u16) -> u8 {
        self.ppu_memory[self.ppu_address_mapping(addr)]
    }

    pub fn set_ppu_memory(&mut self, addr: u16, value: u8) {
        self.ppu_memory[self.ppu_address_mapping(addr)] = value;
    }
}
