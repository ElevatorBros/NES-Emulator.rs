#![allow(dead_code)]
#![allow(unused_variables)]
pub struct Ram {
    pub cpu_memory: [u8; 0x800], // 2KB internal RAM
    pub ppu_memory: [u8; 0x2000],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            cpu_memory: [0xFF; 0x800],
            ppu_memory: [0xFF; 0x2000],
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

    pub fn get_ppu_memory(&self, addr: u16) -> u8 {
        let mut actual_addr = addr;
        // Nametable
        if actual_addr >= 0x2000 && actual_addr < 0x3F00 {
            let vertical = true;
            if vertical {
                if actual_addr > 0x2800 {
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
            // println!("Pallet_read_at:{:#x}", actual_addr);
            // Pallet
            if actual_addr == 0x3F10
                || actual_addr == 0x3F14
                || actual_addr == 0x3F18
                || actual_addr == 0x3F1C
            {
                actual_addr -= 0x10;
            }

            // if actual_addr == 0x3F00
            //     || actual_addr == 0x3F04
            //     || actual_addr == 0x3F08
            //     || actual_addr == 0x3F0C
            // {
            //     actual_addr = 0x3F00;
            // }
        }
        actual_addr -= 0x2000;
        self.ppu_memory[actual_addr as usize]
    }

    pub fn set_ppu_memory(&mut self, addr: u16, value: u8) {
        let mut actual_addr = addr;
        // Nametable
        if actual_addr >= 0x2000 && actual_addr < 0x3F00 {
            let vertical = true;
            if vertical {
                if actual_addr > 0x2800 {
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

            // if actual_addr == 0x3F00
            //     || actual_addr == 0x3F04
            //     || actual_addr == 0x3F08
            //     || actual_addr == 0x3F0C
            // {
            //     actual_addr = 0x3F00;
            // }
        }
        actual_addr -= 0x2000;
        self.ppu_memory[actual_addr as usize] = value;
    }
}
